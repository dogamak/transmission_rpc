//! This crate provides a macro called `torrent_proc` via a compiler plugin.
//! It is used by the parent crate to create a struct and an enum which represent
//! all torrent fields which can be returned by the Transmission RPC.
//!
//! The macro can potentially be used in other contexts and further generalization
//! of the macro and separating this crate more from the parent crate is in consideration.
//!
//! # Usage
//!
//! ```
//! torrent_proc! {
//!    /// Struct containing response to PersonQuery from PersonDatabase
//!    #[derive(Debug,Clone)]
//!    pub struct PartialPerson {
//!        #[time_t] born: NaiveDateTime,
//!        name: String,
//!        pet_names: Vec<String>
//!    }
//!
//!    /// Enum of fields of `PartialPerson`
//!    #[derive(Debug,Clone,PartialEq)]
//!    pub enum PersonField;
//! }
//! ```
//!
//! The macro takes two item definitions: the struct itself followed by an empty enum that
//! is going to contain variants for each field in the struct.
//! Both of these can be marked as public with `pub` and default to being private. At the moment
//! declaring visibility on the struct fields is not supported, and all fields are public.
//!
//! This code expands to following:
//!
//! ```rust
//! /// Struct containing response to PersonQuery from PersonDatabase
//! #[derive(Debug,Clone)]
//! pub struct PartialPerson {
//!     #[serde(rename="born", deserialize_with="deserialize_time_t_option", default)]
//!     born: Option<NaiveDateTime>,
//!     #[serde(rename="name")]
//!     name: Option<String>,
//!     #[serde(rename="petNames")]
//!     pet_names: Option<Vec<String>>
//! }
//!
//! impl Default for PartialPersion {
//!     PartialPerson {
//!         born: None,
//!         name: None,
//!         pet_names: None
//!     }
//! }
//!
//! /// Enum of fields of `PartialPerson`
//! pub enum PersonField {
//!    Born,
//!    Name,
//!    PetNames
//! }
//!
//! impl ::std::fmt::Display for PersonField {
//!     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//!         f.write_str(match this {
//!             &PersonField::Born => "born",
//!             &PersonField::Name => "name",
//!             &PersonField::PetNames => "petNames"
//!         })
//!     }
//! }
//!
//! impl ::serde::Serialize for PersonField {
//!     fn serialize<S>(&self, ser: &mut S) -> S::Error
//!         where S: ::serde::Serializer
//!     {
//!         ser.serialize_str(match this {
//!             &PersonField::Born => "born",
//!             &PersonField::Name => "name",
//!             &PersonField::PetNames => "petNames"
//!         })
//!     }
//! }
//! ```
//! # Notes to self / What's up next
//! 
//! The `#[time_t]` attibute and `#[serde(deserialize_with="deserialize_time_t_option")]` are
//! required by the parent crate and should be abstracted away and moved elsewhere.
//! In fact, in it's current state the macro fails to compile if `deserialize_time_t_option` is
//! not in scope.
//! The enum's `Display` and `Serialize` implementations should also be moved to a separate
//! macro which could be used with custom derive attribute on the enum declaration.

#![feature(proc_macro, proc_macro_lib, plugin_registrar, rustc_private)]
extern crate proc_macro;
extern crate syntax;
extern crate rustc_plugin;
extern crate aster;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use regex::{Regex, Captures};
use rustc_plugin::Registry;
use syntax::ptr::P;
use syntax::ast::{Attribute, Ty, Ident, Item};
use syntax::codemap::Span;
use syntax::tokenstream::TokenTree;
use syntax::parse::common::SeqSep;
use syntax::util::small_vector::SmallVector;
use syntax::ext::base::{ExtCtxt, MacEager, MacResult};
use syntax::parse::token::{Token, DelimToken};
use syntax::symbol::keywords;
use syntax::parse::parser::Parser;

use syntax::ast::MetaItemKind;
use syntax::ast::Expr;

use aster::path::PathBuilder;
use aster::arm::ArmBuilder;
use aster::expr::ExprBuilder;
use aster::AstBuilder;
use aster::attr::AttrBuilder;
use aster::struct_field::StructFieldBuilder;

use std::borrow::Cow;
use std::iter::repeat;

/// Info about a field to be generated for the struct and the enum.
struct Field {
    ident: String,
    time_t: bool,
    is_pub: bool,
    ty: P<Ty>
}

impl Field {
    /// Returns the field's name capitalized using `lowerCamelCase`
    /// Used in serializing the enum variants.
    fn lower_camel_case<'a>(&'a self) -> Cow<'a, str> {
        lazy_static! {
            static ref REGEX_CAMEL_CASE: Regex = Regex::new("_(.)").unwrap();
        }
        let name = self.pascal_case();
        REGEX_CAMEL_CASE.replace_all(name,
                                     |captures: &Captures| captures.get(1).unwrap().as_str().to_uppercase())
    }

    /// Returns the field's name capitalized using `UpperCamelCase`
    /// Used to name enum variants.
    fn upper_camel_case<'a>(&'a self) -> Cow<'a, str> {
        let mut i = self.lower_camel_case();
        let ch = i.to_mut().remove(0).to_uppercase().next().unwrap();
        i.to_mut().insert(0, ch);
        i
    }

    /// Returns the field's name capitalized with `pascal_case`.
    /// Used to name struckt fields.
    fn pascal_case(&self) -> &str {
        self.ident.as_str()
    }
}

/// Reads a struct field from the parser and returns a `Field` containing it's info.
fn parse_field(parser: &mut Parser) -> Field {
    let attrs = parser.parse_outer_attributes().unwrap();

    let time_t = attrs.iter().find(|attr| {
        if attr.value.node == MetaItemKind::Word {
            &*attr.value.name.as_str() == "time_t"
        } else {
            false
        }
    }).is_some();


    let is_pub = parser.eat_keyword(keywords::Pub);
    
    let ident = match parser.parse_ident() {
        Ok(ident) => ident,
        Err(mut e) => {
            e.emit();
            parser.abort_if_errors();
            unreachable!();
        }
    };

    parser.expect(&Token::Colon).unwrap();

    let ty = parser.parse_ty().unwrap();

    Field {
        ident: ident.to_string(),
        ty: ty,
        is_pub: is_pub,
        time_t: time_t
    }
}

/// Information about an item declaration.
struct ItemInfo {
    attrs: Vec<Attribute>,
    is_pub: bool,
    ident: Ident
}

/// Holds the macro's parsed input.
struct MacroInput {
    struct_info: ItemInfo,
    enum_info: ItemInfo,
    fields: Vec<Field>
}

/// Parses the AST into a `MacroInput`.
fn parse_macro_input(ct: &ExtCtxt, tts: &[TokenTree]) -> MacroInput {
    let mut parser = ct.new_parser_from_tts(tts);

    // Parse Struct Declaration
    let struct_attrs = parser.parse_outer_attributes().unwrap();
    let struct_is_pub = parser.eat_keyword(keywords::Pub);
    parser.expect_keyword(keywords::Struct).unwrap();
    let struct_ident = match parser.parse_ident() {
        Ok(ident) => ident,
        Err(mut e) => {
            e.emit();
            parser.abort_if_errors();
            unreachable!();
        }
    };

    let struct_info = ItemInfo {
        attrs: struct_attrs,
        is_pub: struct_is_pub,
        ident: struct_ident
    };

    // Parse Struct Fields
    parser.expect(&Token::OpenDelim(DelimToken::Brace)).unwrap();

    let sep = SeqSep {
        sep: Some(Token::Comma),
        trailing_sep_allowed: true
    };

    let fields = match parser.parse_seq_to_end(
        &Token::CloseDelim(DelimToken::Brace),
        sep,
        |x| Ok(parse_field(x)))
    {
        Ok(fields) => fields,
        Err(mut e) => {
            e.emit();
            vec![]
        }
    };

    
    
    // Parse Enum Declaration
    let enum_attrs = parser.parse_outer_attributes().unwrap();
    let enum_is_pub = parser.eat_keyword(keywords::Pub);
    parser.expect_keyword(keywords::Enum).unwrap();
    let enum_ident = match parser.parse_ident() {
        Ok(ident) => ident,
        Err(mut e) => {
            e.emit();
            parser.abort_if_errors();
            unreachable!();
        }
    };

    let enum_info = ItemInfo {
        attrs: enum_attrs,
        is_pub: enum_is_pub,
        ident: enum_ident
    };
    
    MacroInput {
        struct_info: struct_info,
        enum_info: enum_info,
        fields: fields
    }
}

/// Creates AST for the struct
fn create_struct(input: &MacroInput) -> P<Item>
{
    let builder = AstBuilder::new();

    let mut fields = vec![];
    
    for field in input.fields.iter() {
        let mut attr = AttrBuilder::new()
            .named("serde")
            .list()
            .name_value("rename")
            .str(&*field.lower_camel_case());

        if field.time_t {
            attr = attr.name_value("deserialize_with").str("deserialize_time_t_option")
                .word("default");
        }
        
        let mut _field = StructFieldBuilder::named(field.pascal_case());

        if field.is_pub {
            _field = _field.pub_();
        }
        
        let _field = _field.with_attrs(vec![attr.build()])
            .ty()
            .option()
            .build(field.ty.clone());
        
        fields.push(_field);
    }
    
    let mut struct_builder = builder.item();
    
    if input.struct_info.is_pub {
        struct_builder = struct_builder.pub_();
    }

    let struct_builder = struct_builder.with_attrs(input.struct_info.attrs.clone()).struct_(input.struct_info.ident).with_fields(fields);
    
    struct_builder.build()
}

/// Creates AST for implementation of the Default trait for the struct.
/// `Default::default()` returns an instance of the struct with all fields
/// set to `None`.
fn create_default_impl(input: &MacroInput) -> P<Item> {
    let builder = AstBuilder::new();

    builder.item().impl_()
        .trait_().id("Default").build()
        .method("default")
        .fn_decl().return_().id(input.struct_info.ident)
        .block()
        .stmt().expr().return_expr().struct_id(input.struct_info.ident)
        .with_id_exprs(input.fields.iter().map(|field| Ident::from_str(field.pascal_case())).zip(repeat(ExprBuilder::new().none()))).build().build()
        .ty().id(input.struct_info.ident)
}

/// Creates AST for the enum containing a variant for every field in the struct. 
fn create_field_enum(input: &MacroInput) -> P<Item> {
    let builder = AstBuilder::new();

    let mut _enum = builder.item().with_attrs(input.enum_info.attrs.clone());
    
    if input.enum_info.is_pub {
        _enum = _enum.pub_();
    }
        
    let mut _enum = _enum.enum_(input.enum_info.ident);

    for field in input.fields.iter() {
        _enum = _enum.variant(&*field.upper_camel_case()).unit();
    }

    _enum.build()
}

/// Creates AST for implementation of `all()` method for the enum.
/// `all()` returns array of all variants of the enum.
fn create_field_enum_impl(input: &MacroInput) -> P<Item> {
    let builder = AstBuilder::new();

    builder.item().impl_()
        .item("all").pub_().method()
        .fn_decl().return_().array(input.fields.len()).id(input.enum_info.ident)
        .block().stmt().expr().return_expr().slice()
        .with_exprs(input.fields.iter().map(|field| ExprBuilder::new().path().ids(&[input.enum_info.ident, Ident::from_str(&*field.upper_camel_case())]).build()))
        .build().build()
        .ty().id(input.enum_info.ident)
}

/// Creates AST for a match expression that matches an reference to the enum
/// and returns the name of that variant as `&'static str`.
/// Used by `create_field_enum_display_impl` and `create_field_enum_serialize_impl`.
fn create_field_enum_to_str_match(input: &MacroInput, expr: P<Expr>) -> P<Expr> {
    let arms = input.fields.iter()
        .map(|field| {
            ArmBuilder::new()
                .pat().ref_().path()
                    .id(input.enum_info.ident)
                     .id(&*field.upper_camel_case()).build()
                .body().str(&*field.lower_camel_case())
        });

    ExprBuilder::new().match_().build(expr).with_arms(arms).build()
}

/// Create AST for implementation of the `fmt::Display` trait for the enum.
fn create_field_enum_display_impl(input: &MacroInput) -> P<Item> {
    let builder = AstBuilder::new();
    
    builder.item().impl_()
        .trait_().global().ids(&["std", "fmt", "Display"]).build()
        .method("fmt")
            .fn_decl()
                .self_().ref_()
                .arg().id("f").ty().ref_().mut_().ty().path().global()
                    .ids(&["std","fmt","Formatter"]).build()
                .return_().path().global().ids(&["std","fmt","Result"]).build()
        .block().expr().method_call("write_str")
            .path().id("f").build()
            .arg().build(create_field_enum_to_str_match(&input, ExprBuilder::new().self_())).build()
        .ty().id(input.enum_info.ident)
}

/// Create AST for the implementation of serde's `Serialize` trait for the enum.
fn create_field_enum_serialize_impl(input: &MacroInput) -> P<Item> {
    let builder = AstBuilder::new();

    builder.item().impl_()
        .trait_().global().ids(&["serde", "Serialize"]).build()
        .method("serialize")
            .generics().ty_param("S").trait_bound(PathBuilder::new().global().ids(&["serde", "Serializer"]).build()).build().build().build()
            .fn_decl()
                .self_().ref_()
                .arg_id("ser").ty().ref_().mut_().ty().id("S")
            .return_().result()
                .tuple().build()
                .path().ids(&["S", "Error"]).build()
            .block().expr().method_call("serialize_str")
                .path().id("ser").build()
                .arg().build(create_field_enum_to_str_match(&input, ExprBuilder::new().self_())).build()
        .ty().id(input.enum_info.ident)
}

/// The macro.
fn torrent_proc(ct: &mut ExtCtxt,
                _sp: Span,
                tts: &[TokenTree])
                -> Box<MacResult + 'static>
{
    let input = parse_macro_input(ct, tts);
    MacEager::items(SmallVector::many(vec![
        create_struct(&input),
        create_default_impl(&input),
        create_field_enum(&input),
        create_field_enum_impl(&input),
        create_field_enum_display_impl(&input),
        create_field_enum_serialize_impl(&input)
        // TODO: Create accessors (getters and setters) using the enum variants
        // Enum should be replaced with a module of unit structs if we really want setters.
        // Each unit struct should implement a trait that whould provide the name of the
        // field and a setter method that visits a struct.
        // Then we could have something like this in the struct:
        // fn set<F,V>(field: F, value: F) where F: Field<Value=V> { F::set(&self, value) }
        // The `field` argument could be removed, but then it doesn't feel like an enum anymore.
    ]))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("torrent_proc", torrent_proc);
}

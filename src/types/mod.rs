macro_rules! impl_enum_serde {
    { $typ:ty {
        $( $number:expr => $variant:path ),*
    } } => {
        impl Into<i64> for $typ {
            fn into(self) -> i64 {
                match self {
                    $($variant => $number),+
                }
            }
        }
        
        impl<'a> Into<i64> for &'a $typ {
            fn into(self) -> i64 {
                match self {
                    $(&$variant => $number),+
                }
            }
        }
        
        impl ::serde::Serialize for $typ {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer
            {
                serializer.serialize_i64(self.into())
            }
        }

        impl ::serde::Deserialize for $typ {
            fn deserialize<D>(deserializer: &mut D) -> Result<$typ, D::Error>
                where D: ::serde::Deserializer
            {
                deserializer.deserialize(EnumVisitor)
            }
        }
        
        fn from_i64(value: i64) -> Option<$typ> {
            match value {
                // Negative numbers don't work without the guard
                $(x if x == $number => Some($variant)),+,
                _ => None
            }
        }
        
        struct EnumVisitor;

        impl ::serde::de::Visitor for EnumVisitor {
            type Value = $typ;

            fn visit_i64<E>(&mut self, value: i64) -> Result<$typ, E>
                where E: ::serde::de::Error
            {
                if let Some(p) = from_i64(value as i64) {
                    Ok(p)
                } else {
                    Err(::serde::de::Error::custom("unexpected value"))
                }
            }

            fn visit_u64<E>(&mut self, value: u64) -> Result<$typ, E>
                where E: ::serde::de::Error
            {
                self.visit_i64(value as i64)
            }
        }
    }
}

mod status;
mod priority;
pub mod time_t;

pub use self::status::Status;
pub use self::priority::Priority;

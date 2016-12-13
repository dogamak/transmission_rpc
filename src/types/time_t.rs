use chrono::NaiveDateTime;
use serde::{Deserializer, Deserialize};
use serde::de::{Error, Visitor};

pub fn deserialize_time_t_option<D>(deserializer: &mut D) -> Result<Option<NaiveDateTime>, D::Error>
    where D: Deserializer
{
    struct TimeTVisitor;
    
    impl Visitor for TimeTVisitor {
        type Value = NaiveDateTime;
        
        fn visit_i64<E>(&mut self, value: i64) -> Result<Self::Value, E>
            where E: Error
        {
            NaiveDateTime::from_timestamp_opt(value, 0)
                .ok_or_else(|| Error::invalid_value("could not interpret as UNIX timestamp"))
        }
        
        fn visit_u64<E>(&mut self, value: u64) -> Result<Self::Value, E>
            where E: Error
        {
            self.visit_i64(value as i64)
        }
    }

    struct WrapVisitor;

    impl Visitor for WrapVisitor {
        type Value = Wrap;
        
        fn visit_unit<E>(&mut self) -> Result<Wrap, E>
            where E: Error
        {
            Ok(Wrap(None))
        }
        fn visit_none<E>(&mut self) -> Result<Wrap, E>
            where E: Error
        {
            Ok(Wrap(None))
        }

        fn visit_some<D>(&mut self, deserializer: &mut D) -> Result<Wrap, D::Error>
            where D: Deserializer
        {
            Ok(Wrap(Some(deserializer.deserialize(TimeTVisitor)?)))
        }
    }
    
    struct Wrap(Option<NaiveDateTime>);
    
    impl Deserialize for Wrap {
        fn deserialize<D>(deserializer: &mut D) -> Result<Wrap, D::Error>
            where D: Deserializer
        {
            deserializer.deserialize_option(WrapVisitor)
        }
    }
    
    Wrap::deserialize(deserializer).map(|wrap| wrap.0)
}

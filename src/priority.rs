use serde;
use serde::de::Error;

#[derive(Clone, Debug)]
pub enum Priority {
    Low,
    Normal,
    High
}

impl Priority {
    fn from_i32(v: i32) -> Option<Priority> {
        match v {
            -1 => Some(Priority::Low),
            0 => Some(Priority::Normal),
            1 => Some(Priority::High),
            _ => None
        }
    }
}

impl Into<i8> for Priority {
    fn into(self) -> i8 {
        From::from(match self {
            Priority::Low => -1,
            Priority::Normal => 0,
            Priority::High => 1
        })
    }
}

impl Into<i64> for Priority {
    fn into(self) -> i64 {
        let i: i8 = self.into();
        i as i64
    }
}

impl serde::Serialize for Priority {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_i8(self.clone().into())
    }
}

impl serde::Deserialize for Priority {
    fn deserialize<D>(deserializer: &mut D) -> Result<Priority, D::Error>
        where D: serde::Deserializer,
    {
        deserializer.deserialize(PriorityVisitor)
    }
}

struct PriorityVisitor;

impl serde::de::Visitor for PriorityVisitor {
    type Value = Priority;

    fn visit_i8<E>(&mut self, value: i8) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }
    
    fn visit_i16<E>(&mut self, value: i16) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }
    
    fn visit_i32<E>(&mut self, value: i32) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }
    
    fn visit_i64<E>(&mut self, value: i64) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }

    fn visit_u8<E>(&mut self, value: u8) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }
    
    fn visit_u16<E>(&mut self, value: u16) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }
    
    fn visit_u32<E>(&mut self, value: u32) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }
    
    fn visit_u64<E>(&mut self, value: u64) -> Result<Priority, E> where E: Error {
        if let Some(p) = Priority::from_i32(value as i32) {
            Ok(p)
        } else {
            Err(serde::de::Error::custom("unexpected value"))
        }
    }
}

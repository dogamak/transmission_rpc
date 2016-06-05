use serde_json::{self, Value};
use serde::Serialize;
use std::collections::BTreeMap;

/// A trait representing a request sent to the daemon.
pub trait Request: Serialize {
    type Response;
    const Name: &'static str;

    fn to_value(&self) -> Value {
        Value::Object({
            let mut obj = BTreeMap::new();
            obj.insert("method".to_string(), Value::String(Self::Name.to_string()));
            obj.insert("arguments".to_string(), serde_json::to_value(self));
            obj
        })
    }
}

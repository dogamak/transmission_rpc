use std::collections::BTreeMap;
use types::Priority;
use serde_json::Value;
use super::{Request, RequestArguments};

#[derive(Clone)]
pub struct TorrentSet {
    _ids: Vec<u64>,
    _fields: BTreeMap<String, Value>,
}

macro_rules! set_method {
    ($method:ident, $field:expr, $v:ident, $t:ty) => {
        pub fn $method(mut self, p: $t) -> Self {
            self._fields.insert($field.to_string(), Value::$v(p.into()));
            self
        }
    }
}

impl TorrentSet {
    pub fn new() -> TorrentSet {
        TorrentSet {
            _ids: Vec::new(),
            _fields: BTreeMap::new(),
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self._ids.push(id);
        self
    }

    pub fn ids(mut self, ids: Vec<u64>) -> Self {
        self._ids = ids;
        self
    }

    // TODO: Implement the rest of the setter methods
    set_method!(set_bandwidth_priority, "bandwidth_priority", I64, Priority);
    set_method!(set_download_limit, "download_limit", U64, u32);
    set_method!(set_download_limited, "download_limited", Bool, bool);
    //set_method!(set_wanted_files, wanted_files, array);
    //set_method!(set_unwanted_files, unwanted_files, array);
    set_method!(set_honors_session_limits, "honors_session_limits", Bool, bool);
    set_method!(set_location, "location", String, String);
    set_method!(set_peer_limit, "peer_limit", U64, u32);
    set_method!(set_queue_position, "queue_position", U64, u32);
    set_method!(set_seed_idle_limit, "seed_idle_limit", U64, u32);
    //set_method!(set_seed_idle_mode, seed_idle_mode, number);
    set_method!(set_seed_ratio_limit, "seed_ratio_limit", F64, f64);
    //set_method!(set_seed_ratio_mode, seed_ratio_mode, number);
    //set_method!(set_tracker_add, tracker_add, array);
    //set_method!(set_tracker_remove, tracker_remove, array);
    //set_method!(set_tracker_replace, tracker_replace, array);
    set_method!(set_upload_limit, "upload_limit", U64, u32);
    set_method!(set_upload_limited, "upload_limited", Bool, bool);
}

#[derive(Deserialize, Debug)]
pub struct TorrentSetResponse;

impl Request for TorrentSet {
    type Response = TorrentSetResponse;

    fn method_name(&self) -> &'static str { "torrent-set" }
}

impl RequestArguments for TorrentSet {
    fn arguments(&self) -> Value {
        Value::Object({
            let mut obj = self._fields.clone();
            obj.insert("ids".to_string(), Value::Array(self._ids.clone().into_iter()
                                                       .map(|x| Value::U64(x)).collect()));
            obj
        })
    } 
}

# transmission_rpc

A stronly typed RPC library for Transmission Daemon written in Rust.

## Example
```rust
extern crate transmission_rpc;

use transmission_rpc::{Transmission, Priority};
use transmission_rpc::requests::{AddTorrent, SetTorrent};

fn main() {
    let mut tr = Transmission::new()
        .set_url("http://seedbox.my.domain:9091/rpc")
        .set_auth("user".to_string(), "password".to_string());

    let req = AddTorrent::from_reader(&mut file);
    let res = tr.send(&req).expect("Failed to add the torrent!");

    let req = SetTorrent::new()
        .id(res.id)
        .set_bandwidth_priority(Priority::High);
    tr.send(&req).expect("Failed to set the torrent's priority!");
}
```

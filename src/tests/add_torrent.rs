use super::create_transmission;
use std::io::Cursor;
use requests::AddTorrent;
use torrent::TorrentField;

#[test]
pub fn from_file() {
    println!("{}", TorrentField::all()[0]);
    
    let mut tr = create_transmission();

    let data = include_bytes!("./dummy.torrent");
    let mut torrent = Cursor::new(&data[..]);
    
    let req = AddTorrent::from_reader(&mut torrent).expect("Error while creating the request!");
    
    let res = tr.send(&req).expect("Error while communicating with the server!");
}

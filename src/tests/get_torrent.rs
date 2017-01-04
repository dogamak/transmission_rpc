use super::create_transmission;
use requests::GetTorrent;
use torrent::TorrentField;

#[test]
fn get_all() {
    let mut tr = create_transmission();

    // Fetch the values of one specific value for all torrents.
    let req = GetTorrent::new()
        .field(TorrentField::Id)
        .field(TorrentField::Name)
        .field(TorrentField::AddedDate);

    let res = tr.send(&req).expect("Error while communicating with the server.");

    let first = res.into_iter().next().expect("No torrents returned by the server!");

    println!("{}", first.added_date.unwrap());
    
    assert!(first.id.is_some());
    assert!(first.name.is_some());
    
    // Fetch all fields of one specific torrent.
    let req = GetTorrent::new()
        .id(first.id.unwrap())
        .field(TorrentField::Name);

    let res = tr.send(&req).expect("Error while communicating with the server.");

    let second = res.into_iter().next().unwrap();
    
    assert!(second.name.is_some());
    assert!(second.name.unwrap() == first.name.unwrap());
}

use super::create_transmission;
use requests::GetTorrent;
use field::Field;

#[test]
fn get_all() {
    let mut tr = create_transmission();

    // Fetch the values of one specific value for all torrents.
    let req = GetTorrent::new()
        .field(Field::Id);

    let res = tr.send(&req).expect("Error while communicating with the server.");

    let first = res.into_iter().next().expect("No torrents returned by the server!");

    // Fetch all fields of one specific torrent.
    let req = GetTorrent::new()
        .id(first.id.unwrap());

    let res = tr.send(&req).expect("Error while communicating with the server.");

    assert!(res.into_iter().next().expect("No torrents returned by the server!")
            .id.expect("No torrent id sent by the server!") != 0);
}

use super::create_transmission;
use requests::{TorrentAction, ActionType, ActionTarget, GetTorrent};
use field::Field;

#[test]
pub fn torrent_action() {
    let mut tr = create_transmission();

    tr.send(&TorrentAction::new(ActionType::Stop, ActionTarget::All)).unwrap();

    let torrent = tr.send(&GetTorrent::new()
                          .fields(vec![Field::Id, Field::Status])).unwrap()
        .into_iter().next().unwrap();

    println!("Status: {:?}", torrent.status.unwrap());

    tr.send(&TorrentAction::new(ActionType::Start, ActionTarget::All)).unwrap();

    let torrent2 = tr.send(&GetTorrent::new()
                           .id(torrent.id.unwrap())
                           .field(Field::Status)).unwrap().into_iter().next().unwrap();

    println!("Status2: {:?}", torrent2.status.unwrap());
}

#[cfg(test)]
pub mod add_torrent;

#[cfg(test)]
pub mod get_torrent;

use super::Transmission;
use hyper::Url;

fn create_transmission() -> Transmission {
    let mut tr = Transmission::new();

    let auth = option_env!("TR_AUTH");
    if let Some(auth) = auth {
        let mut auth = auth.split(':');
        tr.set_auth(auth.next().unwrap().to_string(),
                    auth.next().unwrap().to_string());
    }

    let url = option_env!("TR_URL");
    if let Some(url) = url {
        tr.set_url(Url::parse(url).expect("Failed to parse the RCP URL."));
    }

    tr
}

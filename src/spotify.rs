use regex::Regex;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

pub async fn auth() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("playlist-modify-public playlist-read-collaborative playlist-read-private playlist-modify-private")).unwrap();
    let config = Config {
        token_refreshing: true,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url = spotify.get_authorize_url(true).unwrap();
    spotify
        .prompt_for_token(&url)
        .await
        .expect("Couldn't authenticate successfully");
    spotify
}

// Converts https://open.spotify.com/track/<PlayableId> to spotify/track/<PlayableId>
pub fn convert_message_to_raw_track_uri(raw_message: &String) -> String {
    let re = Regex::new(r"open.spotify.com/track/([a-zA-Z0-9]+)").expect("Track regex is invalid");
    let captures = re.captures(raw_message).expect("Not a track!");

    let raw_track_id = format!("spotify/track/{}", &captures[1]);
    raw_track_id
}

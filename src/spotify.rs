use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth, model::TrackId};

pub async fn auth() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("playlist-modify-public")).unwrap();
    let config = Config {
        token_refreshing: true,
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url = spotify.get_authorize_url(false).unwrap();
    spotify
        .prompt_for_token(&url)
        .await
        .expect("Couldn't authenticate successfully");
    spotify
}

// Converts https://open.spotify.com/track/<PlayableId> to spotify/track/<PlayableId>
pub fn convert_playable_to_uri(raw_message: &String) -> PlayableId {
    // @todo Convert raw_message to propert track_id
    let track_id = TrackId::from_uri(&raw_message).unwrap();
    let playable_id = PlayableId::from(track_id);
    playable_id
}

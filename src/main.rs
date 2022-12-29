mod spotify;

use dotenv::dotenv;
use rspotify::{
    clients::OAuthClient,
    model::{PlayableId, PlaylistId, TrackId},
    AuthCodeSpotify,
};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler {
    spotify_client: AuthCodeSpotify,
}

impl Handler {
    pub fn new(spotify_client: &AuthCodeSpotify) -> Handler {
        Handler {
            spotify_client: spotify_client.to_owned(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let help_command = "!helpRec";
        let help_message = "
        Use the !addRec command to have the suggested song be added to the playlist.
        Example: `!addRec https://open.spotify.com/track/5CLc13Ktc8gDsDihuAqFzM?si=giMo5jDTRGKc1dzmhhWa3w`
        ";
        if msg.content == help_command {
            if let Err(why) = msg.channel_id.say(&ctx.http, help_message).await {
                println!("Error sending message: {:?}", why);
            }
            return;
        }

        let add_rec_command = "!addRec";
        if msg.content.starts_with(add_rec_command) {
            let raw_playlist_id =
                std::env::var("PLAYLIST_URI").expect("Expected PLAYLIST_URI in the environment");
            let playlist_id = PlaylistId::from_uri(&raw_playlist_id).unwrap();

            let raw_track_uri = spotify::convert_message_to_raw_track_uri(&msg.content);
            let track_id = TrackId::from_uri(&raw_track_uri).unwrap();

            let playable_id = PlayableId::from(track_id);

            if let Err(why) = self
                .spotify_client
                .playlist_add_items(playlist_id, [playable_id], None)
                .await
            {
                println!("Error adding item to playlist: {:?}", why);
            }
            return;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn start_discord_client(spotify: &AuthCodeSpotify) {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler::new(spotify);

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating handler");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let spotify_client = spotify::auth().await;

    start_discord_client(&spotify_client).await;
}

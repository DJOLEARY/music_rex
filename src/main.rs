mod spotify;

use rspotify::{clients::OAuthClient, model::PlaylistId};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use dotenv::dotenv;

struct Handler;

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
            let client = spotify::auth().await;

            let raw_playlist_id = std::env::var("PLAYLIST_ID").expect("Expected PLAYLIST_ID in the environment");
            let playlist_id = PlaylistId::from_id(raw_playlist_id).unwrap(); 

            let item = spotify::convert_playable_to_uri(&msg.content);

            if let Err(why) = client.playlist_add_items(playlist_id, [item], None).await {
                println!("Error adding item to playlist: {:?}", why);
            }
            return;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating handler");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

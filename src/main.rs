use std::error::Error;
use config::SpaceConfig;
// use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use serenity::{
    client::{Client},
    http::Http,
};

use songbird::SerenityInit;
use log::*;

// impl TypeMapKey for Lavalink {
//     type Value = LavalinkClient;
// }

// struct LavalinkHandler;

#[macro_use]
extern crate lazy_static;

pub mod apis;
mod commands;
pub mod components;
pub mod config;
pub mod database;
pub mod errors;
mod events;
pub mod utils;

type SpaceError = Box<dyn Error + Send + Sync + 'static>;
type SpaceResult<T> = Result<T, SpaceError>;

// #[async_trait]
// impl LavalinkEventHandler for LavalinkHandler {
//     async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
//         println!("Track started!\nGuild: {}", event.guild_id);
//     }
//     async fn track_finish(&self, _client: LavalinkClient, event: TrackFinish) {
//         println!("Track finished!\nGuild: {}", event.guild_id);
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    println!("[BOT] Starting up...");

    let http = Http::new_with_token(SpaceConfig::get_token().as_str());

    let _bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access app info: {:?}", why),
    };

    #[allow(non_snake_case)]
    let mut Space = Client::builder(SpaceConfig::get_token())
        .event_handler(events::Handler)
        .framework(commands::crete_framework())
        .register_songbird()
        .await
        .expect("[BOT] Failed to start.");

    // let lava_client = LavalinkClient::builder(bot_id, SpaceConfig::get_token())
    //     .set_host("127.0.0.1")
    //     .set_password("root")
    //     .build(LavalinkHandler)
    //     .await?;

    // {
    //     let mut data = Space.data.write().await;
    //     data.insert::<Lavalink>(lava_client);
    // }

    #[allow(non_snake_case)]
    let _ = Space
        .start()
        .await
        .map_err(|why| warn!("Client ended: {:?}", why));
    
    #[allow(non_snake_case)]
    let shard_manager = Space.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("[SHTDWN] Could not register ctrl+c handler");
        println!("[SHTDWN] Shutdown signal received (Ctrl-C) | Shutting down...");
        shard_manager.lock().await.shutdown_all().await;
    });

    #[allow(non_snake_case)]
    if let Err(err) = Space.start().await {
        println!("[BOT] Error: {:?}", err);
    }

    Ok(())
}

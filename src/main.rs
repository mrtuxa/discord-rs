extern crate core;

use core::panicking::panic;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

pub struct ShardManagerContainer;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, bot: Ready) {
        println!("Connected as {}", bot.user.name);
    }
    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    // logger to use environment variables
    tracing_subscriber::fmt::init();

    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owner, info.id);
        },
        Err(why) => panic!("Could not access application info {:?}", why);
    };

    let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix(">")).group(&GENERAL_GROUP);
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }
    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal
    });
}
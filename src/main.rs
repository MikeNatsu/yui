use std::env;

use dotenv::dotenv;
use reqwest::header::HeaderMap;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Tunnel {
    id: String,
    public_url: String,
    proto: String,
    region: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TunnelsBody {
    tunnels: Vec<Tunnel>,
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct McuData {
    days_until: i32,
    title: String,
    overview: String,
    poster_url: String,
    release_date: String,
}

#[group]
#[commands(ping, clear, ngrok, mcu)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    let token = env::var("DISCORD_TOKEN").expect("CANNOT READ TOKEN");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn mcu(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let body = client
        .get("https://www.whenisthenextmcufilm.com/api")
        .send()
        .await;
    let data = body.unwrap().json::<McuData>().await.unwrap();
    msg.channel_id
        .send_message(ctx, |m| {
            m.content("got you!!").tts(false).embed(|e| {
                e.title(data.title).image(data.poster_url).field(
                    "Release date",
                    data.release_date,
                    true,
                )
            })
        })
        .await
        .unwrap();

    Ok(())
}

#[command]
async fn ngrok(ctx: &Context, msg: &Message) -> CommandResult {
    let ngrok_api = env::var("NGROK_API_TOKEN").expect("Something went wrong");
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", ngrok_api).parse().unwrap(),
    );
    headers.insert("Ngrok-Version", "2".to_string().parse().unwrap());
    let body = client
        .get("https://api.ngrok.com/tunnels")
        .headers(headers)
        .send()
        .await;
    let data = body.unwrap().json::<TunnelsBody>().await.unwrap();
    let urls: Vec<String> = data
        .tunnels
        .iter()
        .map(|x| x.public_url.to_string())
        .collect();
    let message_value = format!("Tunnels:\n{} ", urls.join("\n"));
    msg.reply(ctx, message_value).await?;

    Ok(())
}

#[command]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_channel = msg.channel(ctx).await.unwrap().guild().unwrap();
    let messages = guild_channel.messages(ctx, |rtx| rtx.limit(100)).await?;
    let messages_ids = messages.iter().map(|message| message.id);
    guild_channel
        .delete_messages(ctx, messages_ids)
        .await
        .unwrap();

    msg.reply(ctx, "Cleared").await?;

    Ok(())
}

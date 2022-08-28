use std::env;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping, clear)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
}

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
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_channel = msg.channel(ctx).await.unwrap().guild().unwrap();
    let messages = guild_channel.messages(ctx, |rtx| rtx.limit(100)).await?;
    let messages_ids = messages.iter().map(|message|  message.id);
    guild_channel.delete_messages(ctx, messages_ids).await.unwrap();

    msg.reply(ctx, "Cleared").await?;

    Ok(())
}

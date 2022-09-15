use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;

#[command]
async fn ping(ctx: &Context, message: &Message) -> CommandResult {
    message.reply(&ctx.http, "Pong!").await?;
    Ok(())
}

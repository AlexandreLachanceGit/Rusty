use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
async fn hey_archy(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hey <@964693548396068916> !").await?;

    let hello_msg = MessageBuilder::new().push("!hello").build();

    let hello_msg_sent = msg.channel_id.say(&ctx.http, hello_msg).await?;
    hello_msg_sent.delete(&ctx.http).await?;

    Ok(())
}

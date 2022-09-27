use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn merch(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Rusty !merch when?").await?;

    Ok(())
}

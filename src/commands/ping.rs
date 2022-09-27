use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let emoji = ReactionType::Custom {
        animated: false,
        id: EmojiId(958380867565281360),
        name: Some("rust_crab".into()),
    };
    msg.react(ctx, emoji).await?;
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

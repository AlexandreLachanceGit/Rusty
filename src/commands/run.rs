use serde::Deserialize;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

const ERROR_TOO_LONG: &str = "ERROR: Output was too long.";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ApiResponse {
    success: Option<bool>,
    stdout: Option<String>,
    stderr: Option<String>,
    error: Option<String>,
}

#[command]
async fn run(ctx: &Context, msg: &Message) -> CommandResult {
    let content = msg.content_safe(&ctx.cache);

    let start_bytes = content.find("```rust").unwrap() + 8;
    let end_bytes = content.rfind("```").unwrap();
    let mut code = &content[start_bytes..end_bytes];

    let injected_code: String;
    if msg.referenced_message.is_some() {
        let message = &msg.referenced_message.as_ref().unwrap().content;
        injected_code = format!("const MSG: &str = \"{}\"; {}", message, code);
        code = &injected_code;
    }

    let code_response = run_code(code).await;

    let mut reply = MessageBuilder::new();

    if code_response.matches('\n').count() < 15 {
        reply.push_codeblock_safe(&code_response, None);
    } else {
        reply.push_quote(ERROR_TOO_LONG);
    }

    let built_reply = reply.build();
    if built_reply.len() < 2000 {
        msg.reply(ctx, built_reply).await?;
    } else {
        let error_reply = MessageBuilder::new().push_quote(ERROR_TOO_LONG).build();
        msg.reply(ctx, error_reply).await?;
    }

    Ok(())
}

async fn run_code(code: &str) -> String {
    let mut map = serde_json::Map::new();
    map.insert("channel".into(), "stable".into());
    map.insert("mode".into(), "debug".into());
    map.insert("edition".into(), "2021".into());
    map.insert("crateType".into(), "bin".into());
    map.insert("tests".into(), false.into());
    map.insert("code".into(), code.into());
    map.insert("backtrace".into(), false.into());

    let client = reqwest::Client::new();
    let response = client
        .post("https://play.rust-lang.org/execute")
        .json(&map)
        .send()
        .await
        .unwrap()
        .json::<ApiResponse>()
        .await
        .unwrap();

    if response.error.is_some() {
        response.error.unwrap()
    } else if response.success.unwrap() {
        response.stdout.unwrap()
    } else {
        response.stderr.unwrap()
    }
}

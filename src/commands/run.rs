use serde::Deserialize;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

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
    let start_bytes = msg.content.find("```rust").unwrap() + 8;
    let end_bytes = msg.content.rfind("```").unwrap();
    let code = &msg.content[start_bytes..end_bytes];

    let code_response = run_code(code).await;
    let reply = MessageBuilder::new()
        .push_codeblock(&code_response, None)
        .build();
    msg.reply(ctx, reply).await?;

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

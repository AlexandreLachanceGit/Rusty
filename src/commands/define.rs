use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::utils::MessageBuilder;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    list: Box<[Description]>,
}

#[derive(Deserialize, Debug)]
struct Description {
    definition: String,
    permalink: String,
    thumbs_up: usize,
    sound_urls: Box<[String]>,
    author: String,
    word: String,
    defid: usize,
    current_vote: String,
    written_on: String,
    example: String,
    thumbs_down: usize,
}

pub async fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    if let CommandDataOptionValue::String(word) = option {
        define(word).await
    } else {
        "Please provide a valid attachment".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("define")
        .description("Get word descript")
        .create_option(|option| {
            option
                .name("word")
                .description("Word to define")
                .kind(prelude::command::CommandOptionType::String)
                .required(true)
        })
}

async fn define(word: &str) -> String {
    let body = reqwest::get(format!(
        "https://api.urbandictionary.com/v0/define?term={}",
        word
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let json: ApiResponse = serde_json::from_str(&body).unwrap();

    let definition = &json.list[0].definition;
    let example = &json.list[0].example;

    let content = MessageBuilder::new()
        .push_bold_line(&word)
        .push_line("")
        .push_bold_line("Definition: ")
        .push_line(definition)
        .push_line("")
        .push_bold_line("Example: ")
        .push(example)
        .build();

    content.to_string()
}

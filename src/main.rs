mod structs;

use std::{fs, process};

use crate::structs::Punlines;
use chrono::{Datelike, Weekday};
use config::{Config, ConfigError, Environment, File, FileFormat};
use frankenstein::AsyncTelegramApi;
use frankenstein::{
    AsyncApi, BotCommand, GetUpdatesParams, Message, ReplyParameters, SendAnimationParams,
    SendPollParams, SetMyCommandsParams, UpdateContent,
};
use rand::prelude::IteratorRandom;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct BotConfig {
    bot_token: String,
}

impl BotConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::new(".env", FileFormat::Toml))
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }
}

static CHAT_ID: i64 = 231642019;

#[tokio::main]
async fn main() {
    match BotConfig::new() {
        Ok(config) => connect_to_api(&config).await,
        Err(e) => {
            eprintln!("Could not build bot configuration: {e}");
            process::exit(1);
        }
    };
}

async fn connect_to_api(config: &BotConfig) {
    let api = AsyncApi::new(&config.bot_token);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    let dodo_command = BotCommand::builder()
        .command("dodo")
        .description("Polls for dota availability")
        .build();
    let set_my_commands_params = SetMyCommandsParams::builder()
        .commands(vec![dodo_command])
        .build();

    if let Err(..) = api.set_my_commands(&set_my_commands_params).await {
        println!("Failed to set commands");
    }

    let mut sent_dodo = false;

    loop {
        if get_day_prefix() == "Do" && !sent_dodo {
            sent_dodo = true;
            send_dodo_poll(api.clone(), None, config).await;
        } else if get_day_prefix() != "Do" {
            sent_dodo = false
        }

        let result = api.get_updates(&update_params).await;

        println!("result: {result:?}");

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone = api.clone();
                        let config_clone = config.clone();

                        tokio::spawn(async move {
                            process_message(message, api_clone, &config_clone).await;
                        });
                    }

                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            }
            Err(_error) => {
                println!("Token seems wrong or connection is lost!")
            }
        }
    }
}

async fn process_message(message: Message, api: AsyncApi, config: &BotConfig) {
    // Check for doubt and its other text forms
    if let Some(ref message_content) = message.text {
        if message_content.contains("doubt") || message_content.contains("daut") {
            let reply_parameters = ReplyParameters::builder()
                .message_id(message.message_id)
                .build();
            let file_path = std::path::PathBuf::from("resources/i_daut_it.gif");
            let send_animation_params = SendAnimationParams::builder()
                .chat_id(CHAT_ID)
                .animation(file_path)
                .reply_parameters(reply_parameters)
                .build();

            if let Err(err) = api.send_animation(&send_animation_params).await {
                println!("Failed to send message: {err:?}");
            }
        }
        if message_content == "/dodo" {
            send_dodo_poll(api, Some(message.clone()), config).await;
        }
    }
}

fn get_day_prefix() -> &'static str {
    let current_time = chrono::offset::Local::now();
    return match current_time.weekday() {
        Weekday::Mon => "Mo",
        Weekday::Tue => "Di",
        Weekday::Wed => "Mi",
        Weekday::Thu => "Do",
        Weekday::Fri => "Fr",
        Weekday::Sat => "Sa",
        Weekday::Sun => "So",
    };
}

async fn send_dodo_poll(api: AsyncApi, message: Option<Message>, config: &BotConfig) {
    let result: Punlines = {
        let file_content =
            fs::read_to_string("resources/punlines.json").expect("Error reading punlines json");
        serde_json::from_str(&file_content).unwrap()
    };

    let yes_answers = &result.dodo_poll.ja;
    let no_answers = &result.dodo_poll.nein;

    let yes_answer = yes_answers.iter().choose(&mut rand::thread_rng()).unwrap();
    let no_answer = no_answers.iter().choose(&mut rand::thread_rng()).unwrap();

    if let Some(message) = message {
        let reply_parameters = ReplyParameters::builder()
            .message_id(message.message_id)
            .build();
        let send_poll_params = SendPollParams::builder()
            .reply_parameters(reply_parameters)
            .chat_id(CHAT_ID)
            .question("Dodo?")
            .options(vec![yes_answer.clone(), no_answer.clone()])
            .build();
        if let Err(err) = api.send_poll(&send_poll_params).await {
            println!("Failed to send message: {err:?}");
        }
    } else {
        let send_poll_params = SendPollParams::builder()
            .chat_id(CHAT_ID)
            .question("Do".to_owned() + get_day_prefix())
            .options(vec![yes_answer.clone(), no_answer.clone()])
            .build();
        if let Err(err) = api.send_poll(&send_poll_params).await {
            println!("Failed to send message: {err:?}");
        }
    }
}

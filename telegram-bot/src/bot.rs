use crate::commands::Command;
use std::env;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::handlers::{command_handler::command_handler, message_handler::message_handler};

pub async fn run() {
    pretty_env_logger::init();
    log::info!("Starting simple-rust-bot!");

    let bot = Bot::new(env::var("TOKEN").expect("TOKEN not found"));
    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(command_handler),
        )
        .branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

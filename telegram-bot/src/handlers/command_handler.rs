use crate::commands::Command;
use crate::controllers::tg_controller::TgMessageController;
use crate::err::Error;
use teloxide::prelude::*;

pub async fn command_handler(msg: Message, bot: Bot, cmd: Command) -> Result<(), Error> {
    let ctl = TgMessageController::from_msg(&bot, &msg).await;
    match cmd {
        Command::Help => ctl.reply("Help message").await,
        Command::Start => ctl.reply("Start message").await,
    }
    .map_err(From::from)
}

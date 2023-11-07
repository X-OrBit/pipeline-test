use crate::controllers::tg_controller::TgMessageController;
use crate::err::Error;
use std::env;
use teloxide::prelude::*;

pub async fn message_handler(msg: Message, bot: Bot) -> Result<(), Error> {
    let ctl = TgMessageController::from_msg(&bot, &msg).await;
    ctl.answer(format!(
        "Current commit *{}*",
        env::var("COMMIT").unwrap_or("not linked".to_owned())
    ))
    .await
    .map_err(From::from)
}

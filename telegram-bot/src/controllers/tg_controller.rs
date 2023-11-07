use teloxide::types::{InlineKeyboardMarkup, ParseMode};
use teloxide::{prelude::*, RequestError};

pub struct TgMessageController<'bot, 'msg> {
    bot: &'bot Bot,
    message: &'msg Message,
}

impl<'bot, 'msg> TgMessageController<'bot, 'msg> {
    pub async fn from_msg(
        bot: &'bot Bot,
        message: &'msg Message,
    ) -> TgMessageController<'bot, 'msg> {
        TgMessageController { bot, message }
    }

    pub async fn answer<R: ToString>(&self, text: R) -> Result<(), RequestError> {
        self.bot
            .send_message(self.message.chat.id, text.to_string())
            .parse_mode(ParseMode::MarkdownV2)
            .disable_web_page_preview(true)
            .send()
            .await
            .map(|_| ())
    }

    pub async fn reply<R: ToString>(&self, text: R) -> Result<(), RequestError> {
        self.bot
            .send_message(self.message.chat.id, text.to_string())
            .parse_mode(ParseMode::MarkdownV2)
            .reply_to_message_id(self.message.id.into())
            .disable_web_page_preview(true)
            .send()
            .await
            .map(|_| ())
    }

    pub async fn edit_text<R: ToString>(
        &self,
        text: R,
        reply_markup: Option<InlineKeyboardMarkup>,
    ) -> Result<(), RequestError> {
        let mut edit_message_reply_markup = self
            .bot
            .edit_message_text(self.message.chat.id, self.message.id, text.to_string())
            .parse_mode(ParseMode::MarkdownV2);
        if let Some(reply_markup) = reply_markup {
            edit_message_reply_markup = edit_message_reply_markup.reply_markup(reply_markup)
        }
        edit_message_reply_markup.send().await.map(|_| ())
    }

    pub async fn edit_reply_markup<R: ToString>(
        &self,
        reply_markup: InlineKeyboardMarkup,
    ) -> Result<(), RequestError> {
        self.bot
            .edit_message_reply_markup(self.message.chat.id, self.message.id)
            .reply_markup(reply_markup)
            .send()
            .await
            .map(|_| ())
    }
}

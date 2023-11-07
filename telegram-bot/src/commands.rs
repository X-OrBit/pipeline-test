use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(description = "Commands:", rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "show this text")]
    Help,
    #[command(description = "start")]
    Start,
}

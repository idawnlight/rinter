use std::error::Error;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Rinter...");

    let bot = Bot::from_env();

    teloxide::repl(bot, handler).await;
}

async fn handler(message: UpdateWithCx<Bot, Message>)
    -> Result<(), Box<dyn Error + Send + Sync>> {

    if message.update.left_chat_member().is_some() {
        message.requester.delete_message(message.update.chat.id, message.update.id).send().await?;
        return Ok(())
    }

    if message.update.new_chat_members().is_none() {
        return Ok(())
    }

    log::info!("Deleting message...");
    message.requester.delete_message(message.update.chat.id, message.update.id).send().await?;
    for user in message.update.new_chat_members().unwrap() {
        log::info!("{}", format!("Kicking member, {}, {}, @{}",
            user.id, user.first_name, user.username.as_deref().unwrap_or_else(|| "[]")));
        message.requester.ban_chat_member(message.update.chat.id, user.id).send().await?;
        message.requester.unban_chat_member(message.update.chat.id, user.id).send().await?;
    }

    Ok(())
}
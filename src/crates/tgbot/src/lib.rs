use core::panic;

use async_trait::async_trait;
use features::{
    llm_api::{self},
    workers::Observer,
};
use shared::{
    llm,
    state::{self, get_tg_accepted_users},
    types::AiRecognizeMethod,
    shell,
};
use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::{Requester, *},
    types::{Message, ParseMode},
    utils::command::BotCommands,
    Bot,
};

use features::lexicon::get_lexicon;

pub mod prerun;

mod music_dispatching;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "control music. examples: \n\t/music pause\n\t/music resume")]
    Music(String),
    #[command(description = "execute shell command")]
    Execute(String),
}

async fn security_check(bot: Bot, msg: Message) -> ResponseResult<()> {
    let authorized_name = if let Some(username) = msg.chat.username() {
        username.to_string()
    } else if let Some(telegram_id) = msg.chat_id() {
        telegram_id.to_string()
    } else {
        return Ok(());
    };
    // sub_to_getactionworker(&msg, &bot).await; // регулярные сообщения от аси
    let accepted_users = get_tg_accepted_users().expect("Accepted users was checked, but empty.");

    if !accepted_users.contains(&authorized_name.to_owned()) {
        bot.send_message(msg.chat.id, get_lexicon("unauthorized"))
            .await?;
    } else if let Some(text) = msg.text() {
        handle_command(text, authorized_name, bot, &msg).await?;
    }

    Ok(())
}

async fn handle_command(
    text: &str,
    username: String,
    bot: Bot,
    msg: &Message,
) -> Result<(), teloxide::RequestError> {
    let slash_command = if state::is_llm_obtained() && !text.starts_with('/') {
        recognize_command_with_llm(text.to_string()).await
    } else {
        text.to_string()
    };

    if text.starts_with('/') || slash_command.starts_with('/') {
        let cmd = Command::parse(&slash_command, username.as_str()).unwrap();
        dispatch(cmd, &bot, msg).await?;
    } else {
        bot.send_message(msg.chat.id, slash_command).await?;
    };

    Ok(())
}

async fn dispatch(cmd: Command, bot: &Bot, msg: &Message) -> Result<(), teloxide::RequestError> {
    match cmd {
        Command::Help => {
            // bot.send_message(msg.chat.id, Command::descriptions().to_string())
            bot.send_message(msg.chat.id, get_lexicon("help"))
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Music(command) => {
            let response = music_dispatching::dispatch_music_command(command, msg).await;
            bot.send_message(msg.chat.id, response)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        Command::Execute(command) => {
            let args = command.split_whitespace().collect();
            let response = format!(
                "<pre>{}\n</pre>",
                shell::execute_command(args).unwrap()
            );
            bot.send_message(msg.chat.id, response)
                .parse_mode(ParseMode::Html)
                .await?;
        }
    };
    Ok(())
}

async fn recognize_command_with_llm(msg: String) -> String {
    let prompt = match state::get_ai_req_method().unwrap() {
        AiRecognizeMethod::Groq => format_for_groq(msg),
        AiRecognizeMethod::AltaS => msg,
        AiRecognizeMethod::None => panic!("Ai recognizing with unspecified model"),
    };
    let response = llm_api::send_request(prompt);
    response
        .await
        .expect("Fail due recognizing command with llm.")
}

fn format_for_groq(msg: String) -> String {
    let commands = r#"
        /music resume, 
        /music pause
        /music status",
    "#;
    let prompt = llm::get_prompt("/telegram/recognize_command");
    let formatted_prompt = prompt
        .replace("{commands}", commands)
        .replace("{message}", msg.as_str());
    formatted_prompt
}

// async fn sub_to_getactionworker(msg: &Message, bot: &Bot) {
//     static INIT: OnceCell<()> = OnceCell::const_new();
//     let worker = workers::get_actionworker().await;
//     let observer = Box::new(PrintObserver {
//         chatid: msg.chat.id,
//         bot: bot.clone(), // todo: fix cloning
//     });
//     INIT.get_or_init(|| async {
//         worker.subscribe(observer).await;
//     })
//     .await;
// }

pub struct PrintObserver {
    chatid: ChatId,
    bot: Bot,
}

#[async_trait]
impl Observer<String> for PrintObserver {
    async fn update(&self, phrase: &String) {
        self.bot.send_message(self.chatid, phrase).await.unwrap();
    }
}

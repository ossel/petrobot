extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

mod tests;

static COMMAND_TODO: &'static str = "/todo";
static COMMAND_TODO_LIST: &'static str = "/todolist";
static COMMAND_TODO_DELETE: &'static str = "/todo_loesche";

fn main() {

    for (key, value) in env::vars() {
        if key.starts_with("TELEGRAM_BOT"){
            println!("{}: {}", key, value);
        }
    }

    let mut core = Core::new().unwrap();
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();
    // read chat Id from environment
    let chat_id = env::var("TELEGRAM_BOT_CHAT_ID").unwrap();
    // cast chat_id into signed integer 64 bit
    let chat_id = chat_id.parse::<i64>().unwrap();
    let chat = ChatId::new(chat_id);
    api.spawn(chat.text("Bin wieder online!"));

    println!("Fetch new updates via long poll method...");
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.

                println!("Chat: {}", chat_id);
                println!("<{}>: {}",&message.from.first_name, data);

                if data.starts_with(COMMAND_TODO_LIST) {
                    api.spawn(chat.text("Todo-Liste:leer"));
                }else if data.starts_with(COMMAND_TODO_DELETE) {
                    api.spawn(chat.text(format!("{} möchte ein TODO löschen.",&message.from.first_name)));
                }else if data.starts_with(COMMAND_TODO){
                    api.spawn(chat.text(format!("{} möchte ein TODO anlegen.",&message.from.first_name)));
                }
                // Answer message with "Hi".
                /*api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));*/
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}

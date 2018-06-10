extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

mod tests;


fn main() {
    let mut core = Core::new().unwrap();
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle()).unwrap();
    let chat_id = 0;//TODO
    let chat = ChatId::new(chat_id);
    api.spawn(chat.text("Wer kümmert sich heute abend um die Enten?"));

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.

                println!("Chat: {}", chat_id);
                println!("<{}>: {}",&message.from.first_name, data);

                if data.starts_with("/todo") {
                    //resident_in_charge = format!("{}",&message.from.first_name.unwrap());
                    api.spawn(chat.text(format!("{} möchte einen TODO anlegen.",&message.from.first_name)));
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

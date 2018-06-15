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

static COMMAND_SHOPPING_LIST: &'static str = "/einkaufsliste";
static COMMAND_SHOPPING_LIST_TYPO: &'static str = "/einkaufliste";
static COMMAND_SHOPPING_ITEM: &'static str = "/einkauf";
static COMMAND_SHOPPING_LIST_DELETE: &'static str = "/einkaufsliste_loeschen";
static COMMAND_SHOPPING_LIST_DELETE_TYPO: &'static str = "/einkaufliste_loeschen";

static COMMAND_POOL_TEMPERATURE: &'static str = "/pool";

static COMMAND_DUCK_CHECK: &'static str = "/entendienst";
static COMMAND_DUCK_RESP_CLAIM_M: &'static str = "/entenpapa";
static COMMAND_DUCK_RESP_CLAIM_F: &'static str = "/entenmama";
static COMMAND_DUCK_POINTS: &'static str = "/entenpunkte";

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


    let mut todo_list:Vec<String> = Vec::new();
    let mut shopping_list:Vec<String> = Vec::new();
    let mut duck_father = String::from("");

    println!("// Fetch new updates via long poll method...");
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {

                println!("<{}>: {}",&message.from.first_name, data);

                if data.starts_with(COMMAND_TODO_LIST) {
                    let mut todo_string = String::new();
                    for (i, todo_task) in todo_list.iter().enumerate() {
                        todo_string.push_str(&format!("{}. ",i));
                        todo_string.push_str(&todo_task);
                        todo_string.push_str("\n");
                    }
                    api.spawn(chat.text(format!("{}",todo_string)));
                }else if data.starts_with(COMMAND_TODO_DELETE) {
                    api.spawn(chat.text(format!("{} möchte einen TODO löschen. Letzter Eintrag gelöscht.",&message.from.first_name)));
                    todo_list.pop();
                }else if data.starts_with(COMMAND_TODO){
                    let chat_input_string = format!("{}",&data.clone());
                    //let mut split = ;
                    let command_and_task: Vec<&str> = chat_input_string.split(COMMAND_TODO).collect();
                    let mut todo_task = String::from(command_and_task[1]);
                    todo_list.push(todo_task.clone());
                }else if data.starts_with(COMMAND_POOL_TEMPERATURE){
                    api.spawn(chat.text(format!("Die Pooltemperatur beträgt 20 Grad Celsius.")));
                    api.spawn(chat.text(format!("... NIIICHT https://www.youtube.com/watch?v=JMAVaefZ384")));
                }else if data.starts_with(COMMAND_DUCK_CHECK){
                    api.spawn(chat.text(format!("{} ist heute für die Enten zuständig.",duck_father)));
                }else if data.starts_with(COMMAND_DUCK_RESP_CLAIM_M)||data.starts_with(COMMAND_DUCK_RESP_CLAIM_F){
                    duck_father = message.from.first_name;
                }else if data.starts_with(COMMAND_DUCK_POINTS){
                    api.spawn(chat.text("Noch nicht implementiert."));
                }else if data.starts_with(COMMAND_SHOPPING_LIST_DELETE)||data.starts_with(COMMAND_SHOPPING_LIST_DELETE_TYPO){
                    shopping_list.clear();
                    api.spawn(chat.text("Liste gelöscht."));
                }else if (data.starts_with(COMMAND_SHOPPING_LIST)
                    || data.starts_with(COMMAND_SHOPPING_LIST_TYPO) )
                    && shopping_list.is_empty(){
                    api.spawn(chat.text("Einkaufsliste leer. Tippe '/einkauf <item>' um etwas hinzuzufügen."));
                }else if data.starts_with(COMMAND_SHOPPING_LIST) || data.starts_with(COMMAND_SHOPPING_LIST_TYPO) {
                    let mut shopping_list_string = String::new();
                    for (i, item) in shopping_list.iter().enumerate() {
                        shopping_list_string.push_str(&format!("{}. ",i));
                        shopping_list_string.push_str(&item);
                        shopping_list_string.push_str("\n");
                    }
                    api.spawn(chat.text(format!("{}",shopping_list_string)));
                }else if data.starts_with(COMMAND_SHOPPING_ITEM){
                    let chat_input_string = format!("{}",&data.clone());
                    //let mut split = ;
                    let command_and_item: Vec<&str> = chat_input_string.split(COMMAND_SHOPPING_ITEM).collect();
                    let mut item = String::from(command_and_item[1]);
                    item.push_str(&format!(" ({})",message.from.first_name));
                    shopping_list.push(item.clone());
                }else{
                    println!("Command '{}' unknown.",data);
                }
                /* Answer message with "Hi".
                api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));*/
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}

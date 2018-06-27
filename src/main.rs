extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate chrono;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use chrono::prelude::*;


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

struct Mate {
    name: String,
    duck_points: i8
}
impl Mate {
    fn to_csv_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.name);
        result.push_str(";");
        result.push_str(&self.duck_points.to_string());
        result
    }
}

impl fmt::Display for Mate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.duck_points)
    }
}


fn read_mates() -> HashMap<String,Mate>{
    let mut mates = HashMap::new();
    let filename = "roommates.csv";
    let mut f = File::open(filename).expect("file <roommates.csv> not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    for line in contents.split("\n") {
        if !line.is_empty(){
            let mate_attributes: Vec<&str> = line.split(";").collect();
            let name = String::from(mate_attributes[0]);
            let points = String::from(mate_attributes[1]);
            let m = Mate {
                name: name,
                duck_points: points.parse::<i8>().unwrap()
            };
            mates.insert(m.name.clone(),m);
        }
    }
    mates
}

fn get_mates() -> Vec<String>{
    let mut mates:Vec<String> = Vec::new();
    for (_key, value) in read_mates() {
        mates.push(value.to_string());
    }
    mates
}

fn to_csv_string(mates:HashMap<String,Mate>) -> String{
    let mut result = String::new();
    for (_key, value) in mates {
        result.push_str(&value.to_csv_string());
        result.push_str("\n");
    }
    result
}


fn write_mates(content: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("roommates.csv")
        .unwrap();
    println!("write to file: {}", content);
    file.write_all(content.as_bytes())?;
    Ok(())
}

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
    let mut duck_father = String::new();
    let mut duck_father_claim_time = Local::now().date().pred();
    println!("// Fetch new updates via long poll method...");
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {

                println!("<{}>: {}",&message.from.first_name, data);

                if data.starts_with(COMMAND_TODO_LIST) {
                    api.spawn(chat.text(format!("{}",to_ordered_list_string(todo_list.clone()))));
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
                }else if data.starts_with(COMMAND_DUCK_CHECK){
                    api.spawn(chat.text(format!("{} ist heute für die Enten zuständig.",duck_father)));
                }else if data.starts_with(COMMAND_DUCK_RESP_CLAIM_M)||data.starts_with(COMMAND_DUCK_RESP_CLAIM_F){
                    let today = Local::now().date();
                    if today == duck_father_claim_time {
                        api.spawn(chat.text(format!("{} ist heute bereits für die Enten zuständig. Versuche es morgen erneut.",duck_father)));
                    }else{
                        duck_father_claim_time = today;
                        duck_father = message.from.first_name;
                        api.spawn(chat.text(String::from("Du bist heute für die Enten zuständig.")));
                        let mut mates = read_mates();
                        let mut points = match mates.get(&duck_father) {
                            None => 0,
                            Some(i) => i.duck_points,
                        };
                        let m = Mate {
                            name: duck_father.to_string(),
                            duck_points: points+1
                        };
                        mates.insert(m.name.clone(),m);
                        write_mates(to_csv_string(mates));
                    }

                }else if data.starts_with(COMMAND_DUCK_POINTS){
                    api.spawn(chat.text(format!("{}",to_ordered_list_string(get_mates()))));
                }else if data.starts_with(COMMAND_SHOPPING_LIST_DELETE)||data.starts_with(COMMAND_SHOPPING_LIST_DELETE_TYPO){
                    shopping_list.clear();
                    api.spawn(chat.text("Liste gelöscht."));
                }else if (data.starts_with(COMMAND_SHOPPING_LIST)
                    || data.starts_with(COMMAND_SHOPPING_LIST_TYPO) )
                    && shopping_list.is_empty(){
                    api.spawn(chat.text("Einkaufsliste leer. Tippe '/einkauf <item>' um etwas hinzuzufügen."));
                }else if data.starts_with(COMMAND_SHOPPING_LIST) || data.starts_with(COMMAND_SHOPPING_LIST_TYPO) {
                    api.spawn(chat.text(format!("{}",to_ordered_list_string(shopping_list.clone()))));
                }else if data.starts_with(COMMAND_SHOPPING_ITEM){
                    let chat_input_string = format!("{}",&data.clone());
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

fn to_ordered_list_string(list:Vec<String>) -> String{
    let mut result = String::new();
    for (i, item) in list.iter().enumerate() {
        result.push_str(&format!("{}. ",i+1));
        result.push_str(&item);
        result.push_str("\n");
    }
    result
}

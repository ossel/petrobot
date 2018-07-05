extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate chrono;
#[macro_use] extern crate log;
extern crate log4rs;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::fs;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use chrono::prelude::*;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};


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
    let mut f = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            File::create(filename).expect("Could not create file <roommates.csv>");
            let result = File::open(filename).expect("Could not open newly created file <roommates.csv>");
            result
        },
    };

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

fn write_mates(content: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("roommates.csv")
        .unwrap();
    match file.write_all(content.as_bytes()){
        Ok(_) => info!("Updated <roommates.csv> : {}", content),
        Err(_) => info!("Could not write to file <roommates.csv> : {}", content),
    };
}

fn get_mates() -> Vec<String>{
    let mut mates:Vec<String> = Vec::new();
    for (_key, value) in read_mates() {
        mates.push(value.to_string());
    }
    mates
}

fn read(filename:String) -> Vec<String>{
    let mut f = match File::open(filename.clone()) {
        Ok(file) => file,
        Err(_) => {
            File::create(filename.clone()).expect("Could not create file");
            let result = File::open(filename.clone()).expect("Could not open newly created file");
            result
        },
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut result:Vec<String> = Vec::new();
    for value in contents.split(";"){
        if !value.is_empty(){
            result.push(value.to_string());
        }
    }
    result
}

fn write(filename: String, data:Vec<String>) {
    let mut content = String::new();
    for value in data {
        content.push_str(&value);
        content.push_str(";");
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open(filename)
        .unwrap();
    match file.write_all(content.as_bytes()){
        Ok(_) => info!("Updated file: {}", content),
        Err(_) => info!("Could not write to file: {}", content),
    };
}

fn to_csv_string(mates:HashMap<String,Mate>) -> String{
    let mut result = String::new();
    for (_key, value) in mates {
        result.push_str(&value.to_csv_string());
        result.push_str("\n");
    }
    result
}


fn main() {
    // init logger
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/petrobot.log").expect("Could not create log/petrobot.log file.");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(LevelFilter::Debug)).expect("Error building log config.");

    log4rs::init_config(config).expect("log4rs init error");

    //print token and chat id
    for (key, value) in env::vars() {
        if key.starts_with("TELEGRAM_BOT"){
            info!("{}: {}", key, value);
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

    println!("Petrobot started. See log/petrobot.log");

    let mut duck_father = String::new();
    let mut duck_father_claim_time = Local::now().date().pred();


    info!("// Fetch new updates via long poll method...");
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {

                info!("<{}>: {}",&message.from.first_name, data);

                if data.starts_with(COMMAND_TODO_LIST) {
                    api.spawn(chat.text(format!("{}",to_ordered_list_string(read("todo-list.csv".to_string())))));
                }else if data.starts_with(COMMAND_TODO_DELETE) {
                    api.spawn(chat.text(format!("Letzter Eintrag gelöscht.")));
                    let mut todo_list = read("todo-list.csv".to_string());
                    todo_list.pop();
                    match fs::remove_file("todo-list.csv"){
                        Ok(_) => info!("todo-list.csv removed."),
                        Err(_) => info!("Could not remove todo-list.csv"),
                    };
                    File::create("todo-list.csv").expect("Could not create file");
                    write("todo-list.csv".to_string(),todo_list);
                }else if data.starts_with(COMMAND_TODO){
                    let chat_input_string = format!("{}",&data.clone());
                    //let mut split = ;
                    let command_and_task: Vec<&str> = chat_input_string.split(COMMAND_TODO).collect();
                    let mut todo_task = String::from(command_and_task[1]);
                    let mut todo_list = read("todo-list.csv".to_string());
                    todo_list.push(todo_task.clone());
                    write("todo-list.csv".to_string(),todo_list);
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
                    write("shopping-list-old.csv".to_string(),read("shopping-list.csv".to_string()));
                    fs::remove_file("shopping-list.csv").expect("Could not remove list");
                    api.spawn(chat.text("Liste gelöscht."));
                }else if (data.starts_with(COMMAND_SHOPPING_LIST)
                    || data.starts_with(COMMAND_SHOPPING_LIST_TYPO) )
                    && read("shopping-list.csv".to_string()).is_empty(){
                    api.spawn(chat.text("Einkaufsliste leer. Tippe '/einkauf <item>' um etwas hinzuzufügen."));
                }else if data.starts_with(COMMAND_SHOPPING_LIST) || data.starts_with(COMMAND_SHOPPING_LIST_TYPO) {
                    api.spawn(chat.text(format!("{}",to_ordered_list_string(read("shopping-list.csv".to_string())))));
                }else if data.starts_with(COMMAND_SHOPPING_ITEM){
                    let chat_input_string = format!("{}",&data.clone());
                    let command_and_item: Vec<&str> = chat_input_string.split(COMMAND_SHOPPING_ITEM).collect();
                    let mut item = String::new();
                    if command_and_item[1].starts_with("@Petrobot") {
                        let prefix_and_item: Vec<&str> = command_and_item[1].split("@Petrobot").collect();
                        item.push_str(prefix_and_item[1]);
                    }else{
                        item.push_str(command_and_item[1]);
                    }

                    item.push_str(&format!(" ({})",message.from.first_name));
                    let mut shopping_list = read("shopping-list.csv".to_string());
                    shopping_list.push(item.clone());
                    write("shopping-list.csv".to_string(),shopping_list);
                }else{
                    info!("Command '{}' unknown.",data);
                }
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


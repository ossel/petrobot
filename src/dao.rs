use std::fmt;

use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;


pub struct Mate {
    pub name: String,
    pub duck_points: i8
}
impl Mate {
    pub fn to_csv_string(&self) -> String {
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


pub fn read_mates() -> HashMap<String,Mate>{
    let mut mates = HashMap::new();
    let filename = "data/roommates.csv";
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

pub fn write_mates(mates: HashMap<String,Mate>) {
    let content = to_csv_string(mates);
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("data/roommates.csv")
        .unwrap();
    match file.write_all(content.as_bytes()){
        Ok(_) => info!("Updated <roommates.csv> : {}", content),
        Err(_) => info!("Could not write to file <roommates.csv> : {}", content),
    };
}

pub fn get_mates() -> Vec<String>{
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

pub fn read_shopping_list() -> Vec<String>{
    read("data/shopping-list.csv".to_string())
}

pub fn write_shopping_list(data:Vec<String>){
    write("data/shopping-list.csv".to_string(),data)
}

pub fn delete_shopping_list(){
    write("data/shopping-list-old.csv".to_string(),read_shopping_list());
    fs::remove_file("data/shopping-list.csv").expect("Could not remove shopping-list.csv file");
}

pub fn read_todo_list() -> Vec<String>{
    read("data/todo-list.csv".to_string())
}

pub fn write_todo_list(data:Vec<String>){
    write("data/todo-list.csv".to_string(),data)
}

pub fn delete_todo_list(){
    match fs::remove_file("todo-list.csv"){
        Ok(_) => info!("todo-list.csv removed."),
        Err(_) => info!("Could not remove todo-list.csv"),
    };
    File::create("todo-list.csv").expect("Could not create file");

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
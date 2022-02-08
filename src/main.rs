use std::{env, fs};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Datelike;

use colored::Colorize;
use terminal_size::{Width, Height, terminal_size};
use serde::{Serialize, Deserialize};

fn main() {
    
    let args = get_args();

    init(); // Make sure nothing breaks later :0
    parse_args(args);
    //help();
}

fn get_width() -> usize {
    let size = terminal_size().expect("Terminal size not found");
    let (Width(W),Height(H)) = size; // w,h are now variables :)
    W.into()
}
fn get_args() -> Vec<String> {
    let mut args : Vec<String> = env::args().collect();
    args.remove(0); // remove first garbage
    return args;
}

fn parse_args(args : Vec<String>) {
    let arg = args.get(0);
    if let Some(arg) = arg {
        match arg.as_str() {
            "help"|"-h" => help(),
            "init" => set_dir(),
            "subject" => parse_subject(args),
            "task" => parse_task(args),
            "generate" => generate(),
            _ => panic!("Wrong args :p"),
        }
    } else {
        help();
    }
}
struct Help_Item {
    name : String,
    desc : String,
    head : bool,
}
impl Help_Item {
    fn new(name : &str, desc : &str, head : bool) -> Help_Item {
        let name = if head { format!(" {} ", name) } else { name.to_string() };
        let desc = desc.to_string();
        Help_Item { name, desc, head }
    }
    fn print(self, w : usize) {
        if self.head {
            println!("\n{:-^width$}", self.name.green(), width=w);
        } else {
            println!("\n{:^width$}", self.name.cyan().bold(), width=w);
        }
        println!("\n{:^width$}", self.desc.blue(), width=w)
    }
}
fn help() {
    let w = get_width();
    println!("\n{:=^width$}", " Help ".red(), width=w);
    let mut items : Vec<Help_Item> = vec![];

    items.push(Help_Item::new("init", "Set current directory as vault", false));
    items.push(Help_Item::new("subject", "Modify subjects", true));
    items.push(Help_Item::new("add", "Add subject", false));
    items.push(Help_Item::new("task", "Modify tasks", true));
    items.push(Help_Item::new("add", "Add task", false));

    for item in items {
        item.print(w);
    }
    //println!("\n{:^width$}\n{:^width$}", "init".cyan().bold(), "Set current directory as vault".blue(), width=w);
}

fn init() -> Result<(), String>{
    // Make sure everything is setup
    // Make sure config exists
    let config = match env::var("HOME") {
        Ok(dir) => dir,
        Err(e) => panic!("HOME is not set"),
    };
    let path = format!("{}/.config/todo", config);
    fs::create_dir_all(path).expect("Could not create config directory ;(");
    // Create config file
    if !std::path::Path::new(format!("{}/.config/todo/config.txt", config).as_str()).exists() {
       // conf file doesnt exist
       println!("\n{:=^width$}", " Not configured ".red(), width=get_width());
       println!("\n{:^width$}", "Please type todo help to configure".cyan(), width=get_width());
       return Ok(());
    }

    create_json().expect("Unable to load json");
    Ok(())
}

fn create_json() -> Result<(), String>{
    if !std::path::Path::new(get_store().as_str()).exists() {
        let data = Data::new();
        let json = serde_json::to_string(&data).expect("cannot serialize data");
        fs::write(get_store(), json).expect("Connot write data");
    } 
    Ok(())
}

fn load_data() -> Result<Data, String> {
    let json = fs::read_to_string(get_store()).expect("Cannot read store file");
    let data : Data = serde_json::from_str(&json).expect("cannot deserialize data");
    Ok(data)
}

fn save_data(data : Data) -> Result<(), String> {
    let json = serde_json::to_string(&data).expect("cannot serialize data");
    fs::write(get_store(), json).expect("Connot write data");
    Ok(())
}

fn get_cfg() -> String {
    env::var("HOME").expect("$HOME is not set") + "/.config/todo/config.txt"
}

fn set_dir() {
    fs::write(get_cfg(), env::current_dir().expect("Cwd unavailable").to_str().unwrap()).expect("Cannot write to cfg file");
}

fn get_dir() -> String {
    fs::read_to_string(get_cfg()).expect("Error reading cfg file")
}

fn get_store() -> String {
    let store = format!("{}/{}", get_dir(), ".todo");
    store
}

fn parse_subject(args : Vec<String>) {
    let arg = args.get(1);
    if let Some(arg) = arg {
        match arg.as_str() {
            "add" => {
                let arg = args.get(2);
                if let Some(arg) = arg {
                    add_subject(arg.to_string());
                } else {
                    help();
                }
            },
            _ => help(),
        }
    } else {
        help();
    }
}

fn add_subject(subject : String) {
    let dir = get_dir();
    fs::create_dir_all(format!("{}/{}", dir, subject)).expect("Could not create subject directory");
    // Load data
    let mut data = load_data().expect("cannot load data");
    data.subjects.push(Subject::new(subject));
    save_data(data).expect("cannot save data");
}

fn parse_task(args : Vec<String>) {
    let arg = args.get(1);
    if let Some(arg) = arg {
        match arg.as_str() {
            "add" => add_task(),
            "complete" => complete_task(),
            _ => help(),
        }
    }
}

fn select_subject() -> usize {
    let mut data = load_data().expect("cannot load data");
    return if data.subjects.len() > 1 {
        loop {
            println!("here are all subjects:");
            for (i, subject) in data.subjects.iter().enumerate() {
                println!("{}) {}", i, subject.name.green());
            }
            println!("{}", "Which one corisponds to the task: ".cyan());
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line = line.split("\n").collect::<Vec<&str>>()[0].to_string();
            if let Ok(n) = line.parse::<usize>() {
                if n < data.subjects.len() {
                    break n;
                } else {
                    println!("{} {}", n, "Is not one of the options".red());
                }
            } else {
                println!("{}", "Err, unable to parse. Try again".red());
            }
        }
    } else {
        0
    };
    
}
fn add_task() {
    // Load data
    let mut data = load_data().expect("cannot load data");
    let n = select_subject();
    let mut name = String::new();
    println!("{}", "What is the name of this task?".green());
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.split("\n").collect::<Vec<&str>>()[0].to_string();

    let task_type : TaskType = loop {
        println!("{}", "What type is your task".cyan());
        for (i, option) in vec!["Project", "Test"].iter().enumerate() {
            println!("{}) {}", i, option.green());
        }
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.split("\n").collect::<Vec<&str>>()[0].to_string();
        if let Ok(n) = line.parse::<usize>() {
            match n {
                0 => break TaskType::Project,
                1 => break TaskType::Test,
                _ => println!("{} {}", n, "Is not one of the options".red()),
            }
        } else {
            println!("{}", "Err, unable to parse. Try again".red());
        }
    };

    data.subjects[n].tasks.push(Task::new(name, Date::new(), task_type));
    save_data(data).expect("unable to save data");
}

fn complete_task() {
    let mut data = load_data().expect("unable to load data");
    let subject = select_subject();
    let i = loop {
        println!("{}", "Which of the following");
        for (i, task) in data.subjects[subject].tasks.iter().enumerate() {
            println!("{}) {}", i, task.name.green());
        }
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.split("\n").collect::<Vec<&str>>()[0].to_string();
        if let Ok(n) = line.parse::<usize>() {
            if let Some(task) = data.subjects[subject].tasks.get(n) {
                break n;
            } else {
                println!{"{}", "That is not an option".red()};
            }
        } else {
            println!("{}", "Unable to parse".red());
        }
    };

    data.subjects[subject].tasks.remove(i);

    save_data(data).expect("Unable to save data");

}

fn generate() {
    let data = load_data().expect("Unable to load data");
    fs::remove_dir_all(get_dir() + "/todo").expect("error purging old");
    for subject in data.subjects {
        for task in subject.tasks {
            task.generate();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum TaskType {
    Test,
    Project,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name : String,
    due : Date,
    task_type : TaskType,
}

#[derive(Serialize, Deserialize, Debug)]
struct Subject {
    name : String,
    tasks : Vec<Task>
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    subjects : Vec<Subject>
}

#[derive(Serialize, Deserialize, Debug)]
struct Date {
    day : u16,
    month : u16,
    year : u16,
}

impl Task {
    fn new(name : String, due : Date, task_type : TaskType) -> Task {
        Task { name, due, task_type }
    }

    fn generate(self) {
        let dir = get_dir() + "/todo";
        let dir = dir.as_str();
        fs::create_dir_all(dir).expect("unable to create dir");
        let date = self.due;
        let date = format!("{}-{}-{}", date.year, date.month, date.day);
        let file_path = dir.to_owned() + "/" + date.as_str() + ".md";
        let file_path = file_path.as_str();
        if !std::path::Path::new(file_path).exists() {
            let mut file = OpenOptions::new().write(true).create(true).open(file_path).expect("error opening file");
            writeln!(file, "Todo").expect("Error writing to file");
            writeln!(file, "#todo").expect("Error writing to file");
        }
        let mut file = OpenOptions::new().append(true).open(file_path).expect("error opening file");
        writeln!(file, "- [ ] {}", self.name).expect("Error writing to file");
    }
}

impl Subject {
    fn new(name : String) -> Subject {
        let tasks : Vec<Task> = vec![];
        Subject { name, tasks }
    }
}

impl Data {
    fn new() -> Data {
        let subjects : Vec<Subject> = vec![];
        Data { subjects }
    }
}

impl Date {
    fn new() -> Date {
        let current_date = chrono::Utc::now().date();
        let mut nums : Vec<u16> = vec![];
        'main :loop {
            println!("{}", "Please enter the due date in form dd-mm-yyyy. Ignored information will use current date:".cyan());
            nums = vec![];
            let mut line = String::new();
            std::io::stdin().read_line(&mut line);
            let date : Vec<&str> = line.split("\n").collect::<Vec<&str>>()[0].split("-").collect();
            let date : Vec<&str> = if date[0] == "" {vec![]} else {date}; // If its empty, use todays date
            for i in 0..3 {
                if let Some(n) = date.get(i) {
                    if let Ok(n) = n.parse::<u16>() {
                        nums.push(n);
                    } else {
                        println!("{}", "Unable to parse input. Try again".red());
                        continue 'main;
                    }
                } else {
                    nums.push(match i {
                        0 => current_date.day() as u16,
                        1 => current_date.month() as u16,
                        2 => current_date.year() as u16,
                        _ => 0
                    });
                }
            }
            break;
        }
        Date { day: nums[0], month : nums[1], year : nums[2] }
    }
}

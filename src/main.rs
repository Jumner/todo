use std::{env, fs};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Datelike;

use colored::Colorize;
use terminal_size::{Width, terminal_size};
use serde::{Serialize, Deserialize};

fn main() {
    let args = get_args(); // Get the passed in args

    init().expect("Unable to initialize"); // Make sure nothing breaks later :0
    parse_args(args); // Parse the args to determine what to fo
}

fn get_width() -> usize { // Get the terminal width for printing
    let size = terminal_size().expect("Terminal size not found");
    let Width(w) = size.0;
    w.into() // Return width as usize
}
fn get_args() -> Vec<String> { // Get the arguments
    let mut args : Vec<String> = env::args().collect(); // Collect into a vector of strings
    args.remove(0); // remove first garbage
    args // return args
 }

fn parse_args(args : Vec<String>) { // Parse the arguments
    let arg = args.get(0); // get first arg if it exists
    if let Some(arg) = arg { // set arg to first arg if it exists
        match arg.as_str() { 
            "help"|"-h" => help(),
            "init" => set_dir(),
            "subject" => parse_subject(args),
            "task" => parse_task(args),
            "update" => generate(),
            _ => panic!("Wrong args :p"),
        }
    } else { // No first arg
        help();
    }
}
struct HelpItem { // Make printing easier
    name : String,
    desc : String,
    head : bool,
}
impl HelpItem {
    fn new(name : &str, desc : &str, head : bool) -> HelpItem { // Create a new item
        let name = if head { format!(" {} ", name) } else { name.to_string() };
        let desc = desc.to_string();
        HelpItem { name, desc, head }
    }
    fn print(self, w : usize) { // Print the help item
        if self.head {
            println!("\n{:-^width$}", self.name.green(), width=w);
        } else {
            println!("\n{:^width$}", self.name.cyan().bold(), width=w);
        }
        println!("\n{:^width$}", self.desc.blue(), width=w)
    }
}
fn help() { // Help page
    let w = get_width();
    println!("\n{:=^width$}", " Help ".red(), width=w);
    let mut items : Vec<HelpItem> = vec![]; // Create a list of help items
    // Populate list
    items.push(HelpItem::new("init", "Set current directory as vault", false));
    items.push(HelpItem::new("subject", "Modify subjects", true));
    items.push(HelpItem::new("add", "Add subject", false));
    items.push(HelpItem::new("task", "Modify tasks", true));
    items.push(HelpItem::new("add", "Add task", false));
    items.push(HelpItem::new("Update", "Update date of task", false));
    items.push(HelpItem::new("complete", "Complete a task", false));
    items.push(HelpItem::new("update", "Update todo", true));

    for item in items {
        item.print(w); // Print each item
    }
}

fn init() -> Result<(), String>{
    // Make sure everything is setup
    let config = match env::var("HOME") { // Check that $HOME is set
        Ok(dir) => dir,
        Err(_e) => panic!("HOME is not set"),
    };
    let path = format!("{}/.config/todo", config); // Create the config dir
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

fn create_json() -> Result<(), String>{ // Creates the .todo file if it doesnt exist
    if !std::path::Path::new(get_store().as_str()).exists() {
        let data = Data::new();
        let json = serde_json::to_string(&data).expect("cannot serialize data");
        fs::write(get_store(), json).expect("Connot write data");
    } 
    Ok(())
}

fn load_data() -> Result<Data, String> { // Load all the date from the .todo json file
    let json = fs::read_to_string(get_store()).expect("Cannot read store file");
    let data : Data = serde_json::from_str(&json).expect("cannot deserialize data");
    Ok(data)
}

fn save_data(data : Data) -> Result<(), String> { // Save data to the .todo json file
    let json = serde_json::to_string(&data).expect("cannot serialize data");
    fs::write(get_store(), json).expect("Connot write data");
    generate(); // Update it once we save
    Ok(())
}

fn get_cfg() -> String { // Get the path of the config file that stores the dir
    env::var("HOME").expect("$HOME is not set") + "/.config/todo/config.txt"
}

fn set_dir() { // Write the current dir to the config file
    fs::write(get_cfg(), env::current_dir().expect("Cwd unavailable").to_str().unwrap()).expect("Cannot write to cfg file");
}

fn get_dir() -> String { // Get the dir stored in the config file
    fs::read_to_string(get_cfg()).expect("Error reading cfg file")
}

fn get_store() -> String { // Get the storage json file
    let store = format!("{}/{}", get_dir(), ".todo");
    store
}

fn parse_subject(args : Vec<String>) { // Parse the subject commands
    let arg = args.get(1);
    if let Some(arg) = arg { // If arg is passed in
        match arg.as_str() {
            "add" => {
                let arg = args.get(2); // Get name arg
                if let Some(arg) = arg { // Name passed
                    add_subject(arg.to_string());
                } else { // No name passed in
                    help();
                }
            },
            _ => help(),
        }
    } else { // No arg passed in
        help();
    }
}

fn add_subject(subject : String) { // Add subject
    let dir = get_dir();
    fs::create_dir_all(format!("{}/{}", dir, subject)).expect("Could not create subject directory"); // Create subject folder
    let mut data = load_data().expect("cannot load data"); // Load data
    data.subjects.push(Subject::new(subject)); // Add subject to data
    save_data(data).expect("cannot save data"); // Save data
}

fn parse_task(args : Vec<String>) { // Parse the task
    let arg = args.get(1);
    if let Some(arg) = arg { // Arg passed in
        match arg.as_str() {
            "add" => add_task(),
            "update" => update_task(),
            "complete" => complete_task(),
            _ => help(),
        }
    } else { // Arg not passed
        help()
    }
}

fn select_subject() -> usize { // Ask user to pick a subject
    let data = load_data().expect("cannot load data");
    return if data.subjects.len() > 1 { // Dont ask if there is only one subject
        loop {
            println!("here are all subjects:"); // Print all the subjects
            for (i, subject) in data.subjects.iter().enumerate() {
                println!("{}) {}", i, subject.name.green());
            }
            println!("{}", "Which one corisponds to the task: ".cyan());
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap(); // Read in the input
            line = line.split("\n").collect::<Vec<&str>>()[0].to_string();
            if let Ok(n) = line.parse::<usize>() { // If it can parsed into usize
                if n < data.subjects.len() { // If they picked an option
                    break n;
                } else { // Wrong option
                    println!("{} {}", n, "Is not one of the options".red());
                }
            } else { // Wrong input
                println!("{}", "Err, unable to parse. Try again".red());
            }
        }
    } else { // Only one subject
        0
    };
    
}

fn select_task(subject : usize) -> usize {
    let data = load_data().expect("Unable to load data");
    loop {
        println!("{}", "Which of the following"); // Print tasks
        for (i, task) in data.subjects[subject].tasks.iter().enumerate() {
            println!("{}) {}", i, task.name.green());
        }
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.split("\n").collect::<Vec<&str>>()[0].to_string(); // Read input
        if let Ok(n) = line.parse::<usize>() { // If it can be parsed
            if let Some(_task) = data.subjects[subject].tasks.get(n) { // If its a task
                break n; // Return it 
            } else { // Not a task
                println!{"{}", "That is not an option".red()};
            }
        } else { // Cant parse
            println!("{}", "Unable to parse".red());
        }
    }
}

fn add_task() { // Add a task
    let mut data = load_data().expect("cannot load data");
    let n = select_subject(); // Select the subject
    let mut name = String::new();
    println!("{}", "What is the name of this task?".green());
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.split("\n").collect::<Vec<&str>>()[0].to_string(); // Read name

    let task_type : TaskType = loop { // Get type
        println!("{}", "What type is your task".cyan());
        // Print type options
        for (i, option) in vec!["Project", "Test"].iter().enumerate() {
            println!("{}) {}", i, option.green());
        }
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.split("\n").collect::<Vec<&str>>()[0].to_string();
        if let Ok(n) = line.parse::<usize>() { // Try to parse to usize
            match n {
                0 => break TaskType::Project,
                1 => break TaskType::Test,
                _ => println!("{} {}", n, "Is not one of the options".red()),
            }
        } else { // Can't parse
            println!("{}", "Err, unable to parse. Try again".red());
        }
    };

    data.subjects[n].tasks.push(Task::new(name, Date::new(), task_type));
    save_data(data).expect("unable to save data");
}

fn update_task() {
    let mut data = load_data().expect("unable to load data");
    let subject = select_subject(); // Select subject
    let i = select_task(subject); // Select task
    data.subjects[subject].tasks[i].due = Date::new(); // Reset date
    save_data(data).expect("Unable to save data");
}

fn complete_task() { // Complete a task
    let mut data = load_data().expect("unable to load data");
    let subject = select_subject(); // Select subject
    let i = select_task(subject);
    data.subjects[subject].tasks.remove(i); // Delete that boi

    save_data(data).expect("Unable to save data");

}

fn generate() { // Generate markdown files
    let data = load_data().expect("Unable to load data");
    match fs::remove_dir_all(get_dir() + "/todo") { // Purge old files
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {},
        Err(_e) => panic!("Unable to purge file"),
        _ => {}
    }
    for subject in data.subjects { // Foreach subject
        for task in subject.tasks { // For each task
            task.generate(subject.name.as_str()); // Generate it
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum TaskType {
    Test,
    Project,
    Study,
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Date { // Needed as libs cant serialize to json
    day : u16,
    month : u16,
    year : u16,
}

impl Task {
    fn new(name : String, due : Date, task_type : TaskType) -> Task {
        Task { name, due, task_type }
    }

    fn generate(self, subject : &str) { // Generate the MD for a task
        let dir = get_dir() + "/todo";
        let dir = dir.as_str(); // Create the todo dir
        fs::create_dir_all(dir).expect("unable to create dir");
        let date = self.due;
        let date = format!("{}-{}-{}", date.year, date.month, date.day); // Reverse the formatting so a-z sort sorts by date
        let file_path = dir.to_owned() + "/" + date.as_str() + ".md"; // File name
        let file_path = file_path.as_str();
        if !std::path::Path::new(file_path).exists() { // If file doesnt exist
            let mut file = OpenOptions::new().write(true).create(true).open(file_path).expect("error opening file"); // Create the file
            writeln!(file, "Todo").expect("Error writing to file"); // Add header
            writeln!(file, "#todo").expect("Error writing to file"); // Add todo marker for plugin
        }
        let mut file = OpenOptions::new().append(true).open(file_path).expect("error opening file"); // Open it
        writeln!(file, "- [ ] {}-{:?}: {}", subject, self.task_type, self.name).expect("Error writing to file"); // Write the checkbox
        match self.task_type { // Check the type
            TaskType::Test => { // If its a test
                let now = Date::now(); // Get date
                let diff = self.due.days() - now.days(); // Check how many days appart they are (days till test)
                if diff <= 7 { // Test less than a week ago
                    let study = Task::new(self.name, Date::now(), TaskType::Study); // Create a study task for today
                    study.generate(subject); // Generate it right away (Never stored in the date structure)
                }
            },
            _ => {},
        }
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
    fn new() -> Date { // Generate a date from user input
        let current_date = chrono::Utc::now().date();
        let mut nums : Vec<u16>; // Stores nums
        'main :loop {
            println!("{}", "Please enter the due date in form dd-mm-yyyy. Ignored information will use current date:".cyan());
            nums = vec![]; // Set it to an emtpy vec
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("unable to read file"); // Read it in
            let date : Vec<&str> = line.split("\n").collect::<Vec<&str>>()[0].split("-").collect(); // Split it on - and store each num in a vec
            let date : Vec<&str> = if date[0] == "" {vec![]} else {date}; // If its empty, make sure its treated as such (use todays date)
            for i in 0..3 { // For each number day month year
                if let Some(n) = date.get(i) { // If the user included it
                    if let Ok(n) = n.parse::<u16>() { // Try to parse it
                        nums.push(n); // Push it if it can be parsed
                    } else { // Parsing error
                        println!("{}", "Unable to parse input. Try again".red());
                        continue 'main; // Restart main loop
                    }
                } else { // User excluded info
                    // Include todays date info
                    nums.push(match i {
                        0 => current_date.day() as u16,
                        1 => current_date.month() as u16,
                        2 => current_date.year() as u16,
                        _ => 0
                    });
                }
            }
            break; // All done! break out
        }
        Date { day: nums[0], month : nums[1], year : nums[2] } // Return this new date object
    }
    fn now() -> Date { // Get the current date
        let current_date = chrono::Utc::now().date();
        Date { day : current_date.day() as u16, month : current_date.month() as u16, year : current_date.year() as u16 }
    }

    fn days(self) -> u64 { // How many days since Jan 1, 2000
        let mut days : u64 = 0;
        days += self.day as u64;
        days += self.month as u64 * 30;
        days += (self.year-2000) as u64 * 365;
        days
    }
}

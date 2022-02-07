use std::{env, fs};

use colored::Colorize;
use terminal_size::{Width, Height, terminal_size};

fn main() {
    
    let args = get_args();
    println!("{:?}", args);

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
            _ => panic!("Wrong args :p"),
        }
    } else {
        help();
    }
}

fn help() {
    let w = get_width();
    println!("\n{:=^width$}", " Help ".red(), width=w);
    println!("\n{:^width$}\n{:^width$}", "init".cyan().bold(), "Set current directory as vault".blue(), width=w);
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
    }
    Ok(())
}

fn get_cfg() -> String {
    env::var("HOME").expect("$HOME is not set") + "/.config/todo/config.txt"
}

fn set_dir() {
    println!("{}", get_cfg());
    fs::write(get_cfg(), env::current_dir().expect("Cwd unavailable").to_str().unwrap()).expect("Cannot write to cfg file");
}

fn get_dir() -> String {
    fs::read_to_string(get_cfg()).expect("Error reading cfg file")
}

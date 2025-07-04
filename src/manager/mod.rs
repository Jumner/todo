pub mod cli;
use dirs::config_dir;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

use cli::main_menu;

use crate::list::List;

fn get_datafile() -> PathBuf {
    let mut dir = config_dir().unwrap();
    dir.push("todo");
    dir.push("data.yaml");
    return dir;
}

pub fn save(list: &List) {
    let mut file = if let Ok(file) = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_datafile())
    {
        file
    } else {
        create_dir_all(get_datafile().parent().unwrap()).unwrap();
        File::create(get_datafile()).unwrap()
    };

    file.write_all(serde_yaml::to_string(list).unwrap().as_bytes())
        .unwrap();
}

pub fn load() -> Option<List> {
    let mut file = if let Ok(file) = File::open(get_datafile()) {
        file
    } else {
        return None;
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let list = serde_yaml::from_str(&contents).unwrap();
    return Some(list);
}

pub fn spin() {
    let mut list = load().unwrap_or(List::new(vec![Duration::from_secs(10)]));
    loop {
        main_menu(&mut list);
        save(&list);
    }
}

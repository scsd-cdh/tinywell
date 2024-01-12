use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use dirs::config_dir;
use crate::microplate::MicroPlate;

pub fn get_results_directory() -> PathBuf {
    if let Some(path) = get_config_path() {
        if path.exists(){
            let mut file = File::open(path).unwrap();

            let mut contents: String = Default::default();
            file.read_to_string(&mut contents).unwrap();
            let save_directory = serde_json::from_str(&contents).unwrap();
            return save_directory
        }
    }

    PathBuf::from(
        env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Unable to find parent folder"),
    )
}

pub fn set_results_directory(save_directory: PathBuf) {
    if let Some(path) = get_config_path() {
        // Make sure the directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }
        }

        // Save the projects to the JSON file
        let json_data = serde_json::to_string_pretty(&save_directory).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
    }
}

pub fn save_sequence_as(file_path: PathBuf, sequence: Vec<MicroPlate>) {
    let json_data = serde_json::to_string_pretty(&sequence)
        .expect("Was unable to serialize the sequence provided");
    let mut file = File::create(file_path.clone())
        .unwrap_or_else(|_| panic!("Was unable to write to file {:?}", file_path.clone()));
    file.write_all(json_data.as_bytes())
        .unwrap_or_else(|_| panic!("Was unable to write serialized sequence to file {:?}", file_path.clone()));
}

pub fn load_sequence(file_path: PathBuf) -> Vec<MicroPlate> {
    if file_path.exists() {
        let mut file = File::open(file_path.clone())
            .unwrap_or_else(|_| panic!("Was unable to open file {:?}", file_path.clone()));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .unwrap_or_else(|_| panic!("Was unable to read from file {:?}", file_path.clone()));
        let sequence: Vec<MicroPlate> = serde_json::from_str(&contents)
            .unwrap_or_else(|_| panic!("Was unable to serialize contents of file {:?}", file_path.clone()));
        return sequence;
    }

    vec![]
}

fn get_config_path() -> Option<PathBuf> {
    if let Some(mut config_path) = config_dir() {
        config_path.push("tinywell");
        config_path.push("config.json");
        Some(config_path)
    } else {
        None
    }
}

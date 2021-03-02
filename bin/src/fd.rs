use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs::File,
    io::{prelude::*, Error, ErrorKind},
    path::PathBuf,
    str,
};

pub fn r_file<'de, T>(file_name: &str, template: T) -> Result<T, Error>
where
    T: Debug + Default + Serialize + DeserializeOwned,
{
    let f = File::open(file_name);
    let mut buffer = String::from("");
    match f {
        Ok(mut file) => match file.read_to_string(&mut buffer) {
            Ok(_) => match serde_json::from_str(&buffer) {
                Ok(d) => Ok(d),
                Err(e) => Err(Error::new(ErrorKind::Other, e)),
            },
            Err(e) => Err(e),
        },
        Err(e) => {
            // Err(Error::new(ErrorKind::Other, e))
            match e.kind() {
                ErrorKind::NotFound => {
                    let template_data = T::default();
                    match w_file(file_name, template_data) {
                        Ok(_) => r_file(file_name, template),
                        Err(e) => Err(e),
                    }
                }
                _ => Err(Error::new(ErrorKind::Other, e)),
            }
        }
    }
}

pub fn w_file<'de, T>(file_name: &str, data: T) -> Result<(), Error>
where
    T: Debug + Serialize + Deserialize<'de>,
{
    let f = File::create(file_name);
    let string_data = serde_json::to_string_pretty(&data).unwrap();
    match f {
        Ok(mut file) => match file.write_all(string_data.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    use std::process::Command;
                    let cmdlineargs: Vec<String> = std::env::args().collect();
                    let _output = Command::new("sudo")
                        .args(&cmdlineargs)
                        .output()
                        .expect("failed to execute process");
                }
                ErrorKind::NotFound => println!("File not found: {}", file_name),
                _ => println!("Unknow error"),
            }
            Err(e)
        }
    }
}

pub fn create_dir(path: PathBuf) {
    if !path.exists() {
        match std::fs::create_dir_all(path.clone()) {
            Ok(_) => println!("Created {}", path.clone().display()),
            Err(e) => {
                match e.kind() {
                    ErrorKind::PermissionDenied => {
                        // If error because of permission denied then restart the program with sudo
                        use std::process::Command;
                        println!("Creating {}", path.clone().display());
                        Command::new("sudo")
                            .args(&["mkdir", "-p", &path.clone().display().to_string()])
                            .output()
                            .expect("failed to execute process");
                    }
                    _ => println!("Failed to create working directory"),
                }
                return;
            }
        }
    }
}

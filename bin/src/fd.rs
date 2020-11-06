use std::env::temp_dir;
use std::{  
    fs::File,
    io::{prelude::*, Error, ErrorKind},
    path::PathBuf,
    str,
    env,
};

pub fn r_file(file_name: &str) -> Result<String, Error> {
    let mut output = temp_dir();
    output.push(file_name);

    let f = File::open(output);
    match f {
        Ok(mut file) => {
            let mut buffer = String::from("");
            match file.read_to_string(&mut buffer) {
                Ok(_) => Ok(buffer),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(Error::new(ErrorKind::Other, e)),
    }
}

pub fn w_file(file_name: &str, data: &str) -> Result<(), Error> {
    let f = File::create(file_name);

    match f {
        Ok(mut file) => match file.write_all(data.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
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
                        use std::os::unix::process::CommandExt;

                        let cmdlineargs:Vec<String>=env::args().collect();
                        let _output = Command::new("sudo")
                                        .args(&cmdlineargs)
                                        .exec();
                    },
                    _ => println!("Failed to create working directory")
                }
                return;
            }
        }
    }
}
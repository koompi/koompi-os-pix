use serde_json::to_string_pretty;
use std::fs::{File, metadata, read_dir};
use std::path::PathBuf;
use std::io::{Result, Error, ErrorKind, prelude::*, BufReader};
use crate::models::packages::Packages;

pub fn list_dir(path: PathBuf) {
    match read_dir(path) {
        Ok(entries) => {
            for i in entries {
                match i {
                    Ok(data) => match metadata(data.path()) {
                        Ok(meta) => match meta.is_dir() {
                            true => list_dir(data.path()),
                            false => println!("{:#?}", data),
                        },
                        Err(e) => println!("{}", e),
                    },
                    Err(e) => println!("{}", e),
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}

pub fn f_writer(file_path: &str, data: &Packages ) -> Result<()> {
    let file = File::create(file_path);
    match file {
        Err(e) => Err(e),
        Ok( mut f) => {
            let write_able_data = to_string_pretty(data);
            match write_able_data {
                Ok(s) => f.write_all(s.as_bytes()),
                Err(e) => Err(Error::new(ErrorKind::Other, e))
            }
        }
    }
}


pub fn file_reader(_path: &str) -> Packages {
    let file = File::open(_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let store: Packages = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    store
}
use std::env::temp_dir;
use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter, Error, ErrorKind};
use std::path::Path;
use std::str;

pub fn r_file(file_name: &str) -> Result<String, Error> {
    let mut output = temp_dir();
    output.push(file_name);

    let f = File::open(output);
    match f {
        Ok(mut file) => {
            let mut buffer = String::from("");
            match file.read_to_string(&mut buffer) {
                Ok(d) => Ok(buffer),
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

// rw file
// match w_file("/tmp/hello.txt", "Hello world") {
//     Ok(_) => match r_file("hello.txt") {
//         Ok(d) => {
//             println!("{:?}", d);
//             Ok(())
//         }
//         Err(e) => Err(e),
//     },
//     Err(e) => Err(e),
// }

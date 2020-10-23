// Global Macro
#![allow(unused_variables, dead_code, unused_imports)]

// Import modules
mod handlers;

// Import standard libs
use std::io::{Error, ErrorKind};

// Import crates
use tokio::{fs, io::AsyncWriteExt};

// Use local mods
use handlers::download::download;
use handlers::rw_file::{r_file, w_file};
// CONSTANTS
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Error> {
    Ok(())
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

// Downloadn
// download(
//     "koompi-themes_20200309.tar.gz",
//     "koompi-themes",
//     "http://repo.koompi.org/pix/koompi-themes_20200309.tar.gz",
// )?;

// Check development or production environment
// if cfg!(debug_assertions) {
//     println!("dev")
// } else {
//     println!("pro")
// }

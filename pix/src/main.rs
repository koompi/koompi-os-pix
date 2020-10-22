// Global Macro
#![allow(unused_variables, dead_code, unused_imports)]

// Import modules
mod handlers;

// Import standard libs
use std::io::BufWriter;
// Import crates
use tokio::{fs, io::AsyncWriteExt};
// Use local mods
use handlers::download::download;

// CONSTANTS
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), anyhow::Error> {
    download(
        "koompi-themes_20200309.tar.gz",
        "koompi-themes",
        "http://repo.koompi.org/pix/koompi-themes_20200309.tar.gz",
    )?;
    Ok(())
}

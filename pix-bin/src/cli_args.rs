use clap::{App, Arg, SubCommand};

// This function does:
// Declare argument list and specs
// Return a matched arguement

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn command_line_interface<'a, 'b>() -> App<'a, 'b> {
    App::new("pix")
        .version(VERSION)
        .author(AUTHOR)
        .arg(
            Arg::with_name("install")
                .help("install applications")
                .short("i")
                .long("install")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("update")
                .help("update applications")
                .short("u")
                .long("update"),
        )
        .arg(
            Arg::with_name("remove")
                .help("remove applications")
                .short("r")
                .long("remove")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("search")
                .help("search applications")
                .short("s")
                .long("search")
                .takes_value(true)
                .multiple(true),
        )
}

use clap::{App, Arg, SubCommand};

// This function does:
// Declare argument list and specs
// Return a matched arguement

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn cmd_args<'a, 'b>() -> App<'a, 'b> {
    App::new("pix")
        .version(VERSION)
        .author(AUTHOR)
        .arg(
            Arg::with_name("install")
                .help("Install applications")
                .short("i")
                .long("install")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("update")
                .help("Update applications")
                .short("u")
                .long("update"),
        )
        .arg(
            Arg::with_name("remove")
                .help("Remove applications")
                .short("r")
                .long("remove")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("search")
                .help("Search applications")
                .short("s")
                .long("search")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("list")
                .help("List applications")
                .short("l")
                .long("list"),
        )
        .arg(
            Arg::with_name("fix")
                .help("Fix system issues")
                .short("f")
                .long("fix")
                .takes_value(true)
                .multiple(true),
        )
}

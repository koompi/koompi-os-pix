// Global Macro
#![allow(unused_variables, dead_code, unused_imports)]

// Import modules
mod cli_args;
mod download;
mod install;
mod operations;
mod rw_file;
// Import standard libs
use std::io::{Error, ErrorKind};

// Import crates
use futures::executor::block_on;
// Use local mods
use cli_args::command_line_interface;
use download::download;
use install::install;
use operations::Operation;
use rw_file::{r_file, w_file};
#[tokio::main]
async fn main() {
    // let pix = command_line_interface();
    // let matches = pix.clone().get_matches();

    // let mut apps: Vec<&str> = Vec::new();

    // // Check for given operation variants from user input
    // let op = if matches.is_present("install") {
    //     Operation::Install
    // } else if matches.is_present("update") {
    //     Operation::Update
    // } else if matches.is_present("remove") {
    //     Operation::Remove
    // } else if matches.is_present("search") {
    //     Operation::Search
    // } else if matches.is_present("list") {
    //     Operation::List
    // } else if matches.is_present("fix") {
    //     Operation::Fix
    // } else {
    //     Operation::Help
    // };

    // // Bind each operation variants to operation functions
    // match op {
    //     Operation::Install => {
    //         let args_list = matches.values_of("install").unwrap().collect::<Vec<_>>();
    //         for arg in args_list.iter() {
    //             apps.push(*arg);
    //         }
    //         // registry.install(&local_db, apps);
    //     }
    //     Operation::Update => {
    //         // registry.update(&mut local_db);
    //     }
    //     Operation::Remove => {
    //         let args_list = matches.values_of("remove").unwrap().collect::<Vec<_>>();
    //         for arg in args_list.iter() {
    //             apps.push(*arg);
    //         }
    //         // registry.remove(apps);
    //     }
    //     Operation::Search => {
    //         let args_list = matches.values_of("search").unwrap().collect::<Vec<_>>();
    //         for arg in args_list.iter() {
    //             apps.push(*arg);
    //         }
    //         // registry.search_papps(apps);
    //     }
    //     Operation::List => {
    //         let args_list = matches.values_of("search").unwrap().collect::<Vec<_>>();
    //         for arg in args_list.iter() {
    //             apps.push(*arg);
    //         }
    //         // registry.search_papps(apps);
    //     }
    //     Operation::Fix => {
    //         let args_list = matches.values_of("search").unwrap().collect::<Vec<_>>();
    //         for arg in args_list.iter() {
    //             apps.push(*arg);
    //         }
    //         // registry.search_papps(apps);
    //     }
    //     _ => {
    //         let helper = pix.clone().print_help();
    //         helper.unwrap();
    //         println!();
    //     }
    // }
    block_on(install());

    // Ok(())
}

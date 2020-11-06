#![allow(dead_code, unused_variables, unused_imports)]
pub mod cli;
pub mod config;
pub mod db;
pub mod download;
pub mod extract;
pub mod fd;
pub mod graph;
pub mod install;
pub mod types;

use cli::cmd_args;
use graph::{gql_all_apps, gql_app_by_name, gql_apps_by_names, gql_db_version};
use types::Operation;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    config::configure();
    let pix = cmd_args();
    let matches = pix.clone().get_matches();

    let mut apps: Vec<&str> = Vec::new();

    // Check for given operation variants from user input
    let op = if matches.is_present("install") {
        Operation::Install
    } else if matches.is_present("update") {
        Operation::Update
    } else if matches.is_present("remove") {
        Operation::Remove
    } else if matches.is_present("search") {
        Operation::Search
    } else if matches.is_present("list") {
        Operation::List
    } else if matches.is_present("fix") {
        Operation::Fix
    } else {
        Operation::Help
    };

    // Bind each operation variants to operation functions
    match op {
        Operation::Install => {
            let args_list = matches.values_of("install").unwrap().collect::<Vec<_>>();
            let mut search_list: Vec<String> = Vec::new();
            args_list
                .iter()
                .for_each(|arg| search_list.push(arg.to_string()));
            install::install(search_list).await?;
        }
        Operation::Update => {
            // registry.update(&mut local_db);
        }
        Operation::Remove => {
            let args_list = matches.values_of("remove").unwrap().collect::<Vec<_>>();
            for arg in args_list.iter() {
                apps.push(*arg);
            }
            // registry.remove(apps);
        }
        Operation::Search => {
            let args_list = matches.values_of("search").unwrap().collect::<Vec<_>>();
            let mut search_list: Vec<String> = Vec::new();
            for arg in args_list.iter() {
                search_list.push(arg.to_string());
            }
            println!("{:#?}", gql_apps_by_names(search_list).await?);

            // registry.search_papps(apps);
        }
        Operation::List => {
            println!("{:#?}", gql_all_apps().await?);
        }
        // Operation::Fix => {
        //     let args_list = matches.values_of("search").unwrap().collect::<Vec<_>>();
        //     for arg in args_list.iter() {
        //         apps.push(*arg);
        //     }
        //     registry.search_papps(apps);
        // }
        _ => {
            let helper = pix.clone().print_help();
            helper.unwrap();
            println!();
        }
    }

    Ok(())
}

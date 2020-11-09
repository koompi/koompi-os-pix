// #![allow(dead_code, unused_variables, unused_imports)]
pub mod cli;
pub mod config;
pub mod db;
pub mod download;
pub mod extract;
pub mod fd;
pub mod graph;
pub mod list;
pub mod types;

use cli::cmd_args;
use db::Database;
use graph::{gql_all_apps, gql_apps_by_names};
use list::list_online;
use types::Operation;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Get all application configurations
    let get_config = config::configure();
    let db_path: String = format!("{}/db.json", get_config.db_dir);
    let mut db = Database::from(&db_path);
    let pix = cmd_args();
    let matches = pix.clone().get_matches();

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
    }
    // else if matches.is_present("fix") {
    //     Operation::Fix
    // }
    else {
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
            db.install(get_config, search_list).await?;
        }
        Operation::Update => db.update(get_config).await?,
        Operation::Remove => {
            let args_list = matches.values_of("remove").unwrap().collect::<Vec<_>>();
            let remove_list: Vec<String> = args_list
                .iter()
                .map(|arg| arg.clone().to_string())
                .collect();
            let db_file = format!("{}/db.json", get_config.db_dir);
            db.remove(remove_list).save(&db_file)?
        }
        Operation::Search => {
            let args_list = matches.values_of("search").unwrap().collect::<Vec<_>>();
            let mut search_list: Vec<String> = Vec::new();
            for arg in args_list.iter() {
                search_list.push(arg.to_string());
            }
            println!();
            let data = gql_apps_by_names(search_list).await?;
            match data {
                Some(d) => {
                    list_online(
                        true,
                        "Name".to_string(),
                        "Installation".to_string(),
                        "Description".to_string(),
                    );
                    d.app_by_names.iter().for_each(|app| {
                        let cmd = format!("pix -i {}", app.name);
                        list_online(false, app.name.clone(), cmd, app.description.clone());
                    })
                }
                None => println!("No apps found."),
            }
            println!();
        }
        Operation::List => {
            println!();
            let data = gql_all_apps().await?;
            match data {
                Some(d) => {
                    list_online(
                        true,
                        "Name".to_string(),
                        "Installation".to_string(),
                        "Description".to_string(),
                    );
                    d.apps.iter().for_each(|app| {
                        let cmd = format!("pix -i {}", app.name);
                        list_online(false, app.name.clone(), cmd, app.description.clone());
                    })
                }
                None => println!("No apps found."),
            }
            println!();
        }
        _ => {
            let helper = pix.clone().print_help();
            helper.unwrap();
            println!();
        }
    }

    Ok(())
}

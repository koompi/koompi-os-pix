// Import modules
// pub mod cli;
// pub mod implements;
// pub mod structs;
// pub mod utils;
pub mod graphql;
use graphql_client::Error;
use serde_json;
use graphql::graph::{gql_all_apps, gql_app_by_name, gql_apps_by_names, gql_db_version};
use serde_derive::{Serialize, Deserialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub errors: Option<Error>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Version {
    version: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Apps {
    pub apps: Vec<App>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct App {
    pub name: String,
    pub description: String,
    pub maintainer: String,
    #[serde(rename="pgpKey")]
    pub pgp_key: String,
    #[serde(rename="buildDate")]
    pub build_date: String,
    pub address: String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AppByName {
    #[serde(rename="appByName")]
    pub app_by_name: App,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AppByNames {
    #[serde(rename="appByNames")]
    pub app_by_names: Vec<App>,
}

#[tokio::main]
async fn main() -> Result<(), failure::Error>{
    // All Apps
    // let all_apps = gql_all_apps().unwrap();
    // let all_apps_data: Response<Apps> = serde_json::from_str(all_apps.as_str())?;
    // println!("{:#?}", all_apps_data);

    // App by name
    // let app_by_name = gql_app_by_name("vscode".to_string())?;
    // let app_by_name_data: Response<AppByName> = serde_json::from_str(app_by_name.as_str())?;
    // println!("{:#?}", app_by_name_data);

    // Many apps by name
    // let apps_by_names = gql_apps_by_names(vec!["vscode".to_string(), "ato".to_string()])?;
    // let apps_by_names_data: Response<AppByNames> = serde_json::from_str(apps_by_names.as_str())?;
    // println!("{:#?}", apps_by_names_data);
    
    // Version
    // let version = gql_db_version()?;
    // let data: Response<Version> = serde_json::from_str(version.as_str()).unwrap();
    // println!("{:#?}", data.data.unwrap().version);
    Ok(())
}



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
    // block_on(install());

    // Ok(())
use super::graph::{gql_all_apps, gql_app_by_name, gql_apps_by_names, gql_db_version};
use super::download::download;
// use std::process::{Command, Stdio};
// use std::os::unix::process::CommandExt;
use super::extract::extract;

pub async fn install(search_list: Vec<String>) -> Result<(), anyhow::Error>{
    let data = gql_apps_by_names(search_list).await?;
        match data {
            Some(d) => {
                let mut file_names: Vec<String> = Vec::new();
                for app in d.app_by_names.iter() {
                    let split_address: Vec<&str> = app.clone().address.split('/').collect();
                    let file_name = split_address.last().unwrap();
                    let file_path = format!("fakeroot/tmp/{}",file_name);
                    file_names.push(file_path.to_string());
                    download(file_path.as_str(), &app.name, &app.address).await?
                }

                for file_name in file_names.iter() {
                    println!("Extracting {}", file_name); 
                    extract(&file_name, "fakeroot/tmp/")?;
                }

            },
            None => println!("No app specified...")
        }
    Ok(())
}

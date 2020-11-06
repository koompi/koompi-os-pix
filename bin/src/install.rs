use super::download::download;
use super::extract::extract;
use super::graph::gql_apps_by_names;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
#[derive(Debug, Clone)]
struct ProcessData {
    pub name: String,
    pub web_src: String,
    pub disk_src: String,
    pub dest: String,
    pub installer: String,
}

pub async fn install(search_list: Vec<String>) -> Result<(), anyhow::Error> {
    let data = gql_apps_by_names(search_list).await?;
    match data {
        Some(d) => {
            let mut process_data: Vec<ProcessData> = Vec::new();

            for app in d.app_by_names.iter() {
                let split_address: Vec<&str> = app.clone().address.split('/').collect();
                let file_name = split_address.last().unwrap();
                let file_path = format!("fakeroot/tmp/{}", file_name);
                process_data.push(ProcessData {
                    name: app.name.to_string(),
                    web_src: app.address.to_string(),
                    disk_src: file_path,
                    dest: "fakeroot/tmp/".to_string(),
                    installer: "".to_string(),
                });
            }

            println!("\nDownloading packages...");
            for app in process_data.iter() {
                download(&app.disk_src, &app.name, &app.web_src).await?
            }

            println!("\nExtracting packages...");
            for app in process_data.iter_mut() {
                extract(&app.name, &app.disk_src, &app.dest)?;
                app.dest = format!("{}{}", app.dest, app.name);
            }

            println!("\nInstalling packages...");
            for app in process_data.iter() {
                let cmd = Command::new("sh")
                    .current_dir(&app.dest)
                    .arg("installer.sh")
                    .stderr(Stdio::piped())
                    .output();
                io::stdout().write_all(cmd.unwrap().stderr.as_ref())?;
            }

            println!("\nUpdating transactions...");
            println!("\nInstallation completed.\n");
        }
        None => println!("No app specified..."),
    }
    Ok(())
}

use super::{
    config::Conf,
    download::download,
    extract::extract,
    fd::{r_file, w_file},
    graph::gql_apps_by_names,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    io::{self, Error, ErrorKind, Write},
    process::{Command, Stdio},
};

#[derive(Debug, Clone)]
struct ProcessData {
    pub name: String,
    pub web_src: String,
    pub disk_src: String,
    pub dest: String,
    pub installer: String,
    pub date: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Database {
    pub packages: Vec<Package>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub date: String,
    pub uninstaller: String,
}

impl Package {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.date
    }

    pub fn get_uninstaller(&self) -> &str {
        &self.uninstaller
    }
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(file: &str) -> Self {
        let data = r_file(file, Self::default()).unwrap();
        data
    }

    pub async fn install(
        &mut self,
        conf: Conf,
        search_list: Vec<String>,
    ) -> Result<(), anyhow::Error> {
        let data = gql_apps_by_names(search_list).await?;
        let mut data_for_db: Vec<Package> = Vec::new();
        match data {
            Some(d) => {
                let mut process_data: Vec<ProcessData> = Vec::new();
                for app in d.app_by_names.iter() {
                    let split_address: Vec<&str> = app.clone().address.split('/').collect();
                    let file_name = split_address.last().unwrap();
                    let file_path = format!("{}/{}", conf.tmp_dir.clone(), file_name);
                    process_data.push(ProcessData {
                        name: app.name.to_string(),
                        web_src: app.address.to_string(),
                        disk_src: file_path,
                        dest: format!("{}/", conf.tmp_dir.clone()),
                        installer: "".to_string(),
                        date: app.build_date.to_string(),
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
                    match io::stdout().write_all(cmd.unwrap().stderr.as_ref()) {
                        Ok(_) => {
                            let uninstaller: String =
                                format!("{}/{}/uninstaller.sh", conf.registry_dir, app.name);

                            let installed = Package {
                                name: app.name.clone(),
                                date: app.date.clone(),
                                uninstaller: uninstaller,
                            };

                            data_for_db.push(installed);
                        }
                        Err(e) => println!("{}", e),
                    }
                }

                println!("\nUpdating transactions...");
                self.add(data_for_db)
                    .save(&format!("{}/db.json", conf.db_dir))
                    .unwrap();
                println!("\nInstallation completed.\n");
            }
            None => println!("No app specified..."),
        }
        Ok(())
    }

    pub fn add(&mut self, apps: Vec<Package>) -> &Self {
        apps.iter()
            .for_each(|app| match self.find_one_index(app.name.clone()) {
                Some(p) => self.packages[p] = app.clone(),
                None => {
                    self.packages.push(app.clone());
                }
            });
        self
    }

    pub fn find_one(&self, name: String) -> Option<Package> {
        let mut found = false;
        let mut res = Package::default();
        for app in self.packages.iter() {
            if app.name == name {
                res = app.clone();
                found = true;
            }
        }

        match found {
            true => Some(res),
            false => None,
        }
    }

    pub fn find_many(&self, names: Vec<String>) -> Result<Vec<Package>, Error> {
        let mut found: Vec<Package> = Vec::new();
        let mut not_found: Vec<String> = Vec::new();
        names
            .iter()
            .for_each(|name| match self.find_one(name.to_owned()) {
                Some(app) => found.push(app),
                None => not_found.push(name.to_owned()),
            });

        match not_found.is_empty() {
            true => Ok(found),
            false => Err(Error::from(ErrorKind::NotFound)),
        }
    }

    fn find_one_index(&self, name: String) -> Option<usize> {
        let mut index: Option<usize> = None;

        self.packages.iter().enumerate().for_each(|(i, p)| {
            if p.name == name {
                index = Some(i)
            }
        });

        index
    }

    // fn update_one(&mut self, name: String, data: Package) -> &Self {
    //     match self.find_one_index(name) {
    //         Some(p) => {
    //             self.packages[p] = data;
    //             self
    //         }
    //         None => self,
    //     }
    // }

    pub async fn update(&self, conf: Conf) -> Result<(), anyhow::Error> {
        let installed_app: Vec<String> = self.packages.iter().map(|p| p.name.clone()).collect();
        let mut reinstall_target: Vec<String> = Vec::new();
        let data = gql_apps_by_names(installed_app.clone()).await?;

        match data {
            Some(d) => {
                for app in d.app_by_names.iter() {
                    for target in installed_app.iter() {
                        if app.name == target.to_string() {
                            let local_version_string = self
                                .find_one(target.to_string())
                                .unwrap()
                                .get_version()
                                .to_string();
                            let local_version_number: u64 = local_version_string.parse().unwrap();
                            let server_version: u64 = app.build_date.to_string().parse().unwrap();

                            if server_version > local_version_number {
                                reinstall_target.push(target.to_string())
                            }
                        }
                    }
                }
            }
            None => println!("No app specified..."),
        }

        if !reinstall_target.is_empty() {
            self.clone().install(conf, reinstall_target).await
        } else {
            Ok(())
        }
    }

    pub fn remove(&mut self, names: Vec<String>) -> &Self {
        names.iter().for_each(|n| {
            self.packages.iter().for_each(|p| {
                if p.name == n.to_string() {
                    let cmd = Command::new("sh")
                        .arg(&p.uninstaller)
                        .stderr(Stdio::piped())
                        .output();
                    io::stdout()
                        .write_all(cmd.unwrap().stderr.as_ref())
                        .unwrap();
                }
            })
        });

        names
            .iter()
            .for_each(|n| self.packages.retain(|p| p.name != n.to_string()));
        self
    }

    pub fn save(&self, file_name: &str) -> Result<(), Error> {
        w_file(file_name, self.clone())
    }
}

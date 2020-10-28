use async_graphql::Object;
use super::application::Application;
use serde_derive::{Serialize, Deserialize};
// use chrono::{DateTime, TimeZone, Utc};
use crate::utils::json_fs::f_writer;
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Packages {
    pub applications: Vec<Application>,
    pub version: String,
}

impl Packages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, app: Application) {
        self.applications.push(app);
    }

    pub fn find_one(&self, name: String) -> Result<Application, String> {
        let mut found = false;
        let mut res: Application = Application::new();
        for i in self.applications.iter() {
            if i.name == name {
                found = true;
                res = i.clone();
            }
        }

        match found {
            true => Ok(res),
            false => Err(String::from("App not found")),
        }
    }

    pub fn find_many(&self, names: Vec<String>) -> Result<Vec<Application>, String> {

        let mut res: Vec<Application> = Vec::new();
        for name in names.iter() {
            match self.find_one(String::from(name)) {
                Ok(app) => res.push(app),
                Err(_) => continue,
            }
        }

        if res.is_empty() {
            Err(String::from("No apps found"))
        } else {
            Ok(res)
        }
    }

    pub fn find_all(&self) -> Vec<Application> {
        self.applications.clone()
    }

    pub fn update_one(&mut self, target_name: String, data: Application) {
        let mut old_data = self.clone();
        let mut target_index: usize = 0;
        let mut found: bool = false;

        for (i, app) in old_data.applications.iter_mut().enumerate() {
            if app.name == target_name {
                target_index = i;
                found = true;
            }
        }

        match found {
            false => {},
            true => {
                old_data.applications[target_index] = data;
                *self = old_data;
            }
        }
    }

    pub fn remove_one(&mut self, name: String) {
        self.applications.retain(|app| app.name != name);
    }

    pub fn save(&self) {
        f_writer("db.json", self).unwrap()
    }
}

#[Object]
impl Packages {
    async fn applications(&self) -> &Vec<Application> {
        &self.applications
    }
    async fn version(&self) -> &str {
        &self.version
    }
}
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Database {
    pub packages: Vec<Package>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub date: u64,
    pub uninstaller: String,
}

impl Package {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> u64 {
        self.date
    }

    pub fn get_uninstaller(&self) -> &str {
        &self.uninstaller
    }
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn form(file: &str) -> Self {
        unimplemented!()
    }

    pub fn add(&mut self, apps: Vec<Package>) -> &Self {
        apps.iter().for_each(|app| self.packages.push(app.clone()));
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
}

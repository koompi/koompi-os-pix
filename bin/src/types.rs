use graphql_client::Error;
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

#[derive(Debug)]
pub enum Operation {
    Install,
    Update,
    Remove,
    Search,
    Help,
    List,
    Fix,
}


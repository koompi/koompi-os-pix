use graphql_client::*;
use reqwest;
use super::types;
use anyhow;
#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_all_apps.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct AllApps;

pub async fn gql_all_apps() -> Result<Option<types::Apps>, anyhow::Error> {
    let req = AllApps::build_query(all_apps::Variables);

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send().await?;
    let response_body: Response<all_apps::ResponseData> = res.json().await?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => {
            let all_apps_data: Response<types::Apps> = serde_json::from_str(s.as_str())?;
            match all_apps_data.data {
                Some(d) => Ok(Some(d)),
                None => Ok(None)
            }
        },
        Err(e) => Err(anyhow::Error::from(e))
    }
    
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_app_by_name.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct AppByName;

pub async fn gql_app_by_name(name: String) -> Result<Option<types::AppByName>, anyhow::Error> {
    let req = AppByName::build_query(app_by_name::Variables { name: name.clone() });

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send().await?;
    let response_body: Response<app_by_name::ResponseData> = res.json().await?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => {
            let app_by_name_data: Response<types::AppByName> = serde_json::from_str(s.as_str())?;
            match app_by_name_data.data {
                Some(data) => Ok(Some(data)),
                None => Ok(None)
            }
        },
        Err(e) => Err(anyhow::Error::from(e))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_apps_by_names.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct AppsByNames;

pub async fn gql_apps_by_names(names: Vec<String>) -> Result<Option<types::AppByNames>, anyhow::Error> {
    let req = AppsByNames::build_query(apps_by_names::Variables { names: names.clone()});

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send().await?;
    let response_body: Response<apps_by_names::ResponseData> = res.json().await?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => {
            let apps_by_names_data: Response<types::AppByNames> = serde_json::from_str(s.as_str())?;
            match apps_by_names_data.data {
                Some(d) => Ok(Some(d)),
                None => Ok(None)
            }
        }
        Err(e) => Err(anyhow::Error::from(e))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_db_version.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct DBVersion;

pub async fn gql_db_version() -> Result<Option<types::Version>, anyhow::Error> {
    let req = DBVersion::build_query(db_version::Variables);

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send().await?;
    let response_body: Response<db_version::ResponseData> = res.json().await?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => {
            let data: Response<types::Version> = serde_json::from_str(s.as_str())?;
            match data.data {
                Some(d) => Ok(Some(d)),
                None => Ok(None)
            }
        },
        Err(e) => Err(anyhow::Error::from(e))
    }

        // Version

    // let version = gql_db_version()?;
    // let data: Response<Version> = serde_json::from_str(version.as_str()).unwrap();
    // println!("{:#?}", data.data.unwrap().version);
}

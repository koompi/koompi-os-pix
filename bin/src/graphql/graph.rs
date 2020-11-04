use graphql_client::*;
use reqwest;
#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_all_apps.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct AllApps;

pub fn gql_all_apps() -> Result<String, failure::Error> {
    let req = AllApps::build_query(all_apps::Variables);

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send()?;
    let response_body: Response<all_apps::ResponseData> = res.json()?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => Ok(s),
        Err(e) => Err(failure::Error::from(e))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_app_by_name.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct AppByName;

pub fn gql_app_by_name(name: String) -> Result<String, failure::Error> {
    let req = AppByName::build_query(app_by_name::Variables { name: name.clone() });

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send()?;
    let response_body: Response<app_by_name::ResponseData> = res.json()?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => {
            Ok(s)
        },
        Err(e) => Err(failure::Error::from(e))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_apps_by_names.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct AppsByNames;

pub fn gql_apps_by_names(names: Vec<String>) -> Result<String, failure::Error> {
    let req = AppsByNames::build_query(apps_by_names::Variables { names: names.clone()});

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send()?;
    let response_body: Response<apps_by_names::ResponseData> = res.json()?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => Ok(s),
        Err(e) => Err(failure::Error::from(e))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "../graphql/query_db_version.graphql",
    schema_path = "../graphql/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct DBVersion;

pub fn gql_db_version() -> Result<String, failure::Error> {
    let req = DBVersion::build_query(db_version::Variables);

    let client = reqwest::Client::new();
    let mut res = client.post("http://localhost:4000").json(&req).send()?;
    let response_body: Response<db_version::ResponseData> = res.json()?;

    match serde_json::to_string_pretty(&response_body) {
        Ok(s) => Ok(s),
        Err(e) => Err(failure::Error::from(e))
    }
}

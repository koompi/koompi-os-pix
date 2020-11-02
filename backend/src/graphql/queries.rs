use async_graphql::{Context, Object, FieldResult, FieldError};

// local imports
// use super::root::Storage;
use crate::models::{application::Application};
use crate::utils::json_fs;
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn apps(&self, _ctx: &Context<'_>) -> Vec<Application> {
        // let db = ctx.data_unchecked::<DB>().package.to_owned();
        let db = json_fs::file_reader("db.json");
        db.find_all()
    }

    pub async fn app_by_name(&self, _ctx: &Context<'_>, name: String) -> FieldResult<Application> {
        // let db = ctx.data_unchecked::<DB>().package.clone();
        let db = json_fs::file_reader("db.json");
        match db.find_one(name) {
            Ok(data) => Ok(data),
            Err(e) => Err(FieldError::from(e))
        }
    }

    pub async fn app_by_names(&self, _ctx: &Context<'_>, names: Vec<String>) -> FieldResult<Vec<Application>> {
        // let db = ctx.data_unchecked::<DB>().package.clone();
        let db = json_fs::file_reader("db.json");
        match db.find_many(names) {
            Ok(data) => Ok(data),
            Err(e) => Err(FieldError::from(e))
        }
    }

    pub async fn version(&self, _ctx: &Context<'_>) -> String {
        // let db = ctx.data_unchecked::<DB>().package.clone();
        let db = json_fs::file_reader("db.json");
        db.version
    }
}

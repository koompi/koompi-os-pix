use async_graphql::{Context, Object, FieldResult};

// local imports
// use super::root::Storage;
use crate::models::{application::Application};
use super::{queries::QueryRoot};
use crate::utils::json_fs;
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add(&self, ctx: &Context<'_>, name: String, description: String, maintainer: String, pgp_key: String, build_date: String) -> FieldResult<Application> {
        // let mut db = ctx.data_unchecked::<DB>().package.to_owned();
        let mut db = json_fs::file_reader("db.json");
        let new_app = Application {
            name: name.clone(), 
            description: description, 
            maintainer: maintainer, 
            pgp_key: pgp_key, 
            build_date: build_date,
        };
        db.add(new_app);
        db.save();
        QueryRoot.app_by_name(&ctx, name).await

    }
}

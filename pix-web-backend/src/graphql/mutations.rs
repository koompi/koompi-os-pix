use async_graphql::{Context, Object, FieldResult, FieldError};

// local imports
// use super::root::Storage;
use crate::models::{application::Application};
use super::{queries::QueryRoot};
use crate::utils::json_fs;
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add(&self, ctx: &Context<'_>, name: String, description: String, maintainer: String, pgp_key: String, build_date: String) -> FieldResult<Application> {
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

    async fn update(&self, ctx: &Context<'_>, target_name: String, name: String, description: String, maintainer: String, pgp_key: String, build_date: String) -> FieldResult<Application> {
        let mut db = json_fs::file_reader("db.json");
        let new_app = Application {
            name: name.clone(), 
            description: description, 
            maintainer: maintainer, 
            pgp_key: pgp_key, 
            build_date: build_date,
        };
        db.update_one(target_name, new_app);
        db.save();
        QueryRoot.app_by_name(&ctx, name).await

    }

    async fn remove(&self, ctx: &Context<'_>, name: String) -> FieldResult<String> {
        let mut db = json_fs::file_reader("db.json");
        match db.find_one(name) {
            Ok(app) => {
                db.remove_one(app.name.clone());
                db.save();

                match QueryRoot.app_by_name(&ctx, app.name).await {
                    Ok(_) => Ok(String::from("App deleted")),
                    Err(_) => Ok(String::from("App deleted"))
                }
            }
            Err(_) => Err(FieldError::from(String::from("App not found")))
        }
    }
}

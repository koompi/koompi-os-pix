// #![allow(unused_imports)]
pub mod utils;
pub mod graphql;
pub mod models;
use actix_web::{
    guard, web, App, HttpResponse, HttpServer, Result,
};
use envfile::EnvFile;
use models::packages::Packages;
use utils::json_fs;
use chrono::Utc;
use std::{
    fs::metadata,
    path::Path
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use graphql::{mutations::MutationRoot, queries::QueryRoot, root::{AppSchema}};

async fn index(
    // db: web::Data<Packages>, 
    schema: web::Data<AppSchema>, req: Request) -> Response {
    // let ctx = DB {
    //     package: db.get_ref().to_owned()
    // };
    schema.execute(req.into_inner()
        // .data(ctx)
    ).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let env = EnvFile::new(&Path::new(".env")).unwrap();
    let file_exists = metadata("db.json").is_ok();
    if !file_exists {
        let mut db: Packages = Packages::new();
        db.version = Utc::now().to_string();
        json_fs::f_writer("db.json", &db).unwrap();
    }

    // let db = json_fs::file_reader("db.json");
    
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            // .data(db.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind(format!("{}:{}", env.get("IP").unwrap(), env.get("PORT").unwrap()))?
    .run()
    .await
}

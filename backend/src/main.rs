// #![allow(unused_imports)]
pub mod graphql;
pub mod models;
pub mod utils;
use actix_web::{
    guard,
    http::{header, StatusCode},
    web, App, HttpRequest, HttpResponse, HttpServer, Result,
};

use actix_cors::Cors;
use actix_files;
use actix_files as fs;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, MultipartOptions};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use chrono::Utc;
use envfile::EnvFile;
use graphql::{
    mutations::MutationRoot,
    queries::QueryRoot,
    root::{AppSchema, JwtToken},
};
use models::packages::Packages;
use std::{fs::metadata, path::Path};
use utils::json_fs;
async fn index(schema: web::Data<AppSchema>, req: HttpRequest, gql_request: Request) -> Response {
    // Get authorization token for headers
    let token = req.headers().get("Authorization").and_then(|value| {
        value
            .to_str()
            .map(|s| JwtToken {
                token: s.to_string(),
            })
            .ok()
    });

    // Inject token into context
    let mut request = gql_request.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./public/index.html")?.set_status_code(StatusCode::NOT_FOUND))
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

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allowed_methods(vec!["POST"])
                    .allow_any_origin()
                    .allowed_header(header::CONTENT_TYPE),
            )
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(index)
                    .app_data(MultipartOptions::default().max_num_files(3)),
            )
            .service(actix_files::Files::new("/public", "./public"))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_playground),
            )
            .service(fs::Files::new("/packages", "./public").index_file("index.html"))
            .service(fs::Files::new("/login", "./public").index_file("index.html"))
            .service(fs::Files::new("/dashboard", "./public").index_file("index.html"))
            .service(fs::Files::new("/", "./public").index_file("index.html"))
            .default_service(
                // 404 for GET request
                web::resource("").route(web::get().to(p404)),
            )
    })
    .bind(format!(
        "{}:{}",
        env.get("IP").unwrap(),
        env.get("PORT").unwrap()
    ))?
    .run()
    .await
}

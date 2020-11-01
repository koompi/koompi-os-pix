use async_graphql::{Context, Object, FieldResult, FieldError, SimpleObject, ID, Upload};
use crate::models::{application::Application};
use super::{queries::QueryRoot, root::JwtToken};
use crate::utils::json_fs;
use envfile::EnvFile;
use std::path::Path;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm, DecodingKey, Validation, decode};
use bcrypt::{hash, verify};
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde_derive::{Serialize, Deserialize};
use chrono::prelude::*;
use uuid::Uuid;
pub struct MutationRoot; 

#[derive(Debug, Serialize, Deserialize, Default)]
struct Claims {
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    sub: String,         // Optional. Subject (whom token refers to)
}

#[derive(Clone, SimpleObject, Debug)]
pub struct FileInfo {
    id: ID,
    filename: String,
    mimetype: Option<String>,
}

fn verifier(token: String) -> FieldResult<()> {
    let env = EnvFile::new(&Path::new(".env")).unwrap();
    #[allow(non_snake_case)]
    let SECRET = env.get("SECRET").unwrap();
    let token_message = decode::<Claims>(&token, &DecodingKey::from_secret(SECRET.as_ref()), &Validation::new(Algorithm::HS256));
    
    match token_message {
        Ok(_) => Ok(()),
        Err(e) => Err(FieldError::from(e))
    }
}

#[Object]
impl MutationRoot {
    async fn add(&self, ctx: &Context<'_>, name: String, description: String, maintainer: String, pgp_key: String, build_date: String) -> FieldResult<Application> {
        let token = ctx.data_opt::<JwtToken>();
        match token {
            Some(t) => {
                match verifier(t.token.clone()) {
                    Ok(_) => {
                        let mut db = json_fs::file_reader("db.json");
                        match db.find_one(name.clone()) {
                            Ok(_) => Err(FieldError::from(format!("NOT ALLOWED! App with the same name existed."))),
                            Err(_) => {
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
                        
                    },
                    Err(e) => Err(FieldError::from(format!("Unauthorized {:?}", e)))
                }
            },
            None => Err(FieldError::from(format!("Unauthorized")))
        }
    }

    async fn update(&self, ctx: &Context<'_>, target_name: String, name: String, description: String, maintainer: String, pgp_key: String, build_date: String) -> FieldResult<Application> {
        let token = ctx.data_opt::<JwtToken>();
        match token {
            Some(t) => {
                match verifier(t.token.clone()) {
                    Ok(_) => {
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
                    },
                    Err(e) => Err(FieldError::from(format!("Unauthorized {:?}", e)))
                }
            },
            None => Err(FieldError::from(format!("Unauthorized")))
        }
        

    }

    async fn remove(&self, ctx: &Context<'_>, name: String) -> FieldResult<Application> {
        let token = ctx.data_opt::<JwtToken>();
        match token {
            Some(t) => {
                match verifier(t.token.clone()) {
                    Ok(_) => {
                        let mut db = json_fs::file_reader("db.json");
                        match db.find_one(name) {
                            Ok(app) => {
                                db.remove_one(app.name.clone());
                                db.save();
        
                                match QueryRoot.app_by_name(&ctx, app.name).await {
                                    Ok(_) => Ok(Application::new()),
                                    Err(_) => Ok(Application::new()),
                                }
                            }
                            Err(_) => Err(FieldError::from(String::from("App not found")))
                        }
                    },
                    Err(e) => Err(FieldError::from(format!("Unauthorized {:?}", e)))
                }
            },
            None => Err(FieldError::from(format!("Unauthorized")))
        }
        
    }

    async fn signup(&self, email: String, password: String) -> FieldResult<String> {

        let env = EnvFile::new(&Path::new(".env")).unwrap();

        let allow_signup = env.get("ALLOW_SIGNUP");
        match allow_signup {
            None => {
                Err(FieldError::from("Singing up rule not set."))
            },
            Some(text) => match text {
                "true" => {
                    let hashed = hash(password, 6).unwrap();
                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(".env")
                        .unwrap();

                    

                    let data = std::fs::read_to_string(".env").unwrap();
                    let new_data = data.replace("true", "false");

                    let mut  new_env = OpenOptions::new().write(true).append(false).open(".env").unwrap();
                    new_env.write_all(new_data.as_bytes()).unwrap();
                    new_env.flush().unwrap();

                    writeln!(file, "EMAIL={}", email).unwrap();
                    writeln!(file, "PASSWORD={}", hashed).unwrap();

                    Ok(String::from("User created"))
                }
                "false" | _ => {
                    Err(FieldError::from("Singing up is not allowed."))
                },
            }
        }

    }

    async fn signin(&self, email: String, password: String) -> FieldResult<String> {
        let env = EnvFile::new(&Path::new(".env")).unwrap();

        let e_email = env.get("EMAIL").unwrap();
        let e_pass = env.get("PASSWORD").unwrap();

        match email.clone() == e_email {
            true => {
                match verify(password, e_pass).unwrap() {
                    true => {
                        #[allow(non_snake_case)]
                        let SECRET = env.get("SECRET").unwrap();
                        let mut option = Claims::default();
                        option.iss = String::from("pix@koompi.org");
                        option.iat = Utc::now().timestamp() as usize; 
                        option.exp = option.iat + (24 * 3600);
                        option.sub = email;
                        let token = encode(&Header::default(), &option, &EncodingKey::from_secret(SECRET.as_ref()))?;
                        
                        Ok(token.to_string())
                    }
                    false => {
                        Err(FieldError::from(String::from("Incorrect password.")))
                    }
                }
            },
            false => {
                Err(FieldError::from(String::from("Incorrect email.")))
            }
        }
    }

    async fn single_upload(&self, ctx: &Context<'_>, file: Upload) -> FieldResult<FileInfo> {

        let token = ctx.data_opt::<JwtToken>();
        match token {
            Some(t) => {
                match verifier(t.token.clone()) {
                    Ok(_) => {
                        let upload = file.value(ctx).unwrap();
                        let info = FileInfo {
                            id: ID::from(Uuid::new_v4()),
                            filename: upload.filename.clone(),
                            mimetype: upload.content_type.clone(),
                        };
 
                        let mut file_data: Vec<u8> = Vec::new();
                        let data = upload.try_clone().unwrap().into_read().read_to_end(&mut file_data);

                        match data {
                            Ok(_) => {
                                let file = std::fs::File::create(format!("public/applications/{}", info.filename.clone()));
                                match file {
                                    Ok(mut f) =>{
                                        match f.write_all(file_data.as_ref()) {
                                            Ok(_) => Ok(info),
                                            Err(e) => Err(FieldError::from(e))
                                        }
                                    } ,
                                    Err(e) => Err(FieldError::from(e))
                                }
                            },
                            Err(e) => Err(FieldError::from(e))
                        }
                    },
                    Err(e) => Err(FieldError::from(format!("Invalid token {:?}", e)))
                }
            },
            None => Err(FieldError::from(format!("Unauthorized")))
        }   
    }
}


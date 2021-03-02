use super::fd::create_dir;
use lazy_static::*;
use std::io::prelude::*;
use std::{env, path::PathBuf};

// #[derive(Debug)]
// pub enum ProcessEnv {
//     Dev,
//     Pro,
// }

lazy_static! {
    // Dirs
    static ref ROOT: PathBuf = rd();
    static ref PIX_DIR: PathBuf = ROOT.join("var/lib/pix");
    static ref DB_DIR: PathBuf = PIX_DIR.join("db");
    static ref REG_DIR: PathBuf = PIX_DIR.join("reg");
    static ref TMP_DIR: PathBuf = ROOT.join("tmp");
    static ref CONF_DIR: PathBuf = rd().join("etc/pi.conf.d");

    // Files
    static ref DB_SUFFIX: String = String::from(".db");
    static ref CONF_FILE: PathBuf = CONF_DIR.join("pi.conf");

}

fn rd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs")
    } else {
        PathBuf::from("/")
    }
}

#[derive(Debug, Default)]
pub struct Conf {
    // pub env: ProcessEnv,
    pub root_dir: String,
    pub registry_dir: String,
    pub db_dir: String,
    pub tmp_dir: String,
}

pub fn configure() -> Conf {
    // let mut conf: Conf = Conf::new();
    // #[allow(unused_assignments)]
    // let mut root: PathBuf = PathBuf::from("/");
    // if cfg!(debug_assertions) {
    //     println!("Running in development mode");
    //     let path = env::current_exe().unwrap().display().to_string();
    //     let splitted_path: Vec<&str> = path.split("/").collect();
    //     let project_dir =
    //         PathBuf::from(splitted_path[0..splitted_path.len() - 3].join("/")).join("fakeroot");
    //     root = project_dir;
    // } else {
    //     conf.env = ProcessEnv::Pro;
    //     root = PathBuf::from("/");
    // }
    // // Check if needed directories exist
    // let needed_dir: Vec<&str> = vec!["var/lib/pix/registry", "var/lib/pix/db", "tmp"];
    // needed_dir
    //     .iter()
    //     .for_each(|path| create_dir(root.join(path)));

    // conf.root_dir = root.display().to_string();
    // conf.db_dir = format!("{}/var/lib/pix/db", conf.root_dir);
    // conf.registry_dir = format!("{}/var/lib/pix/registry", conf.root_dir);
    // conf.tmp_dir = format!("{}/tmp", conf.root_dir);
    prepare();
    Conf::new()
}

// impl Default for ProcessEnv {
//     fn default() -> Self {
//         Self::Dev
//     }
// }

impl Conf {
    pub fn new() -> Self {
        Self {
            root_dir: ROOT.as_path().to_str().unwrap().to_string(),
            registry_dir: REG_DIR.as_path().to_str().unwrap().to_string(),
            db_dir: DB_DIR.as_path().to_str().unwrap().to_string(),
            tmp_dir: TMP_DIR.as_path().to_str().unwrap().to_string(),
        }
    }
}

fn prepare() {
    #[cfg(debug_assertions)]
    if !ROOT.as_path().exists() {
        create_dir(ROOT.to_path_buf());
    }

    if !PIX_DIR.as_path().exists() {
        create_dir(PIX_DIR.to_path_buf());
    }
    if !DB_DIR.as_path().exists() {
        create_dir(DB_DIR.to_path_buf());
    }
    if !REG_DIR.as_path().exists() {
        create_dir(REG_DIR.to_path_buf());
    }
    if !REG_DIR.as_path().exists() {
        create_dir(REG_DIR.to_path_buf());
    }
}

use super::fd::create_dir;
use std::{env, path::PathBuf};

#[derive(Debug)]
pub enum ProcessEnv {
    Dev,
    Pro,
}

#[derive(Debug, Default)]
pub struct Conf {
    pub env: ProcessEnv,
    pub root_dir: String,
    pub registry_dir: String,
    pub db_dir: String,
    pub tmp_dir: String,
}

pub fn configure() -> Conf {
    let mut conf: Conf = Conf::new();
    #[allow(unused_assignments)]
    let mut root: PathBuf = PathBuf::new();
    if cfg!(debug_assertions) {
        println!("Running in development mode");
        let path = env::current_exe().unwrap().display().to_string();
        let splitted_path: Vec<&str> = path.split("/").collect();
        let project_dir =
            PathBuf::from(splitted_path[0..splitted_path.len() - 3].join("/")).join("fakeroot");
        root = project_dir;
    } else {
        conf.env = ProcessEnv::Pro;
        root = PathBuf::from("");
    }
    // Check if needed directories exist
    let needed_dir: Vec<&str> = vec!["var/lib/pix/registry", "var/lib/pix/db", "tmp"];
    needed_dir
        .iter()
        .for_each(|path| create_dir(root.join(path)));

    conf.root_dir = root.display().to_string();
    conf.db_dir = format!("{}/var/lib/pix/db", conf.root_dir);
    conf.registry_dir = format!("{}/var/lib/pix/registry", conf.root_dir);
    conf.tmp_dir = format!("{}/tmp", conf.root_dir);
    conf
}

impl Default for ProcessEnv {
    fn default() -> Self {
        Self::Dev
    }
}

impl Conf {
    pub fn new() -> Self {
        Self::default()
    }
}

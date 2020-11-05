use std::{env, path::{PathBuf, Path}, io::ErrorKind};
use super::fd::create_dir;

pub fn configure() {
    #[allow(unused_assignments)]
    let mut root: PathBuf = PathBuf::new();
    if cfg!(debug_assertions) {
        println!("Running in development mode");
        let path = env::current_exe().unwrap().display().to_string();
        let splitted_path: Vec<&str> = path.split("/").collect();
        let project_dir =  PathBuf::from(splitted_path[0..splitted_path.len() - 3].join("/")).join("fakeroot");
        root = project_dir;
    } else {
        root = PathBuf::from("/");
    }

    let needed_dir: Vec<&str> = vec!["var/lib/pix/cache", "var/lib/pix/db", "var/lib/pix/sync", "tmp"];
    needed_dir.iter().for_each(|path| create_dir(root.join(path)));
}



use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::File,
    io::*,
    process::{Command, Stdio},
};
use tar::Archive;

pub fn extract(name: &str, src: &str, dest: &str) -> std::io::Result<()> {
    let file = File::open(src)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    let cmd = Command::new("gzip")
        .arg("-lq")
        .arg(src)
        .stdout(Stdio::piped())
        .spawn();
    let mut buf = String::new();
    cmd.unwrap()
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut buf)?;

    let splitted_data: Vec<&str> = buf.split_whitespace().collect();
    let total_size_string = splitted_data[1].to_string();
    let total_size: u64 = total_size_string.parse().unwrap();

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "=> {app} {bar}",
                app = name,
                bar = "{wide_msg}[{bar:60.red/yellow}] {percent:>3}% {total_bytes:>10}"
            ))
            .progress_chars("#>-"),
    );

    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();

        file.unpack(format!("{}/{}", dest, file.path().unwrap().display()))
            .unwrap();
        pb.inc(file.header().entry_size().unwrap());
    }
    pb.finish();
    Ok(())
}

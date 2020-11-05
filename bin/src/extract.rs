use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

pub fn extract(source_file_path: &str, destination_dir: &str) -> std::io::Result<()> {
    let target = &format!("{}",source_file_path);

    let file = File::open(target)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    match archive.unpack(destination_dir) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}
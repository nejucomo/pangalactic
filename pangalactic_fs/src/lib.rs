use std::io::Result;
use std::path::Path;

pub fn ensure_directory_exists<P: AsRef<Path>>(dir: P) -> Result<()> {
    use std::io::ErrorKind::AlreadyExists;

    let dirpath: &Path = dir.as_ref();

    match std::fs::create_dir(dirpath) {
        Ok(_) => {
            log::info!("Created directory: {:?}", dirpath);
            Ok(())
        }
        Err(e) => match e.kind() {
            AlreadyExists => {
                log::debug!("Directory {:?} exists.", dirpath);
                Ok(())
            }
            _ => Err(e),
        },
    }
}

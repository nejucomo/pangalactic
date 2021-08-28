use log;
use pangalactic_appdirs::AppDirs;
use std::io::Result;
use std::path::Path;

#[cfg(test)]
mod tests;

pub fn init(dirs: AppDirs, path: &Path) -> Result<()> {
    if log::log_enabled!(log::Level::Debug) {
        log::debug!("init{:?}", (&dirs, path));
    } else {
        log::info!("Initializing {:?}", path);
    }
    std::fs::create_dir(path)?;
    std::fs::create_dir(path.join(crate::PG_REPO_ATTIC))?;
    Ok(())
}

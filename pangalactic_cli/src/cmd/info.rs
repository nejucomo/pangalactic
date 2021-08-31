use pangalactic_appdirs::AppDirs;
use std::io::Result;
use std::path::Path;

pub fn info(dirs: AppDirs, path: &Path, json: bool) -> Result<()> {
    let repo = crate::repo::Repo::find_from(path)?;
    if json {
        serde_json::to_writer_pretty(std::io::stdout(), &repo).unwrap();
        Ok(())
    } else {
        todo!("info{:?} : non-json support)", (&dirs, path, json));
    }
}

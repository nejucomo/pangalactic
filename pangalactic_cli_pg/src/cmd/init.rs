use log;
use std::io::Result;
use std::path::Path;

#[cfg(test)]
mod tests;

pub fn init(path: &Path) -> Result<()> {
    use pangalactic_fs::create_dir;

    log::info!("Initializing {:?}", path);
    create_dir(path)?;
    let attic = path.join(crate::PG_REPO_CONTROL);
    let secretdir = attic.join("SECRET");

    create_dir(&attic)?;
    create_dir(&secretdir)?;
    init_publisher(&secretdir)?;

    Ok(())
}

fn init_publisher(secretdir: &Path) -> Result<()> {
    use pangalactic_codecpath::CodecPath;
    use pangalactic_cryptopubsub::Publisher;

    let pubpath = secretdir.join("publisher");
    log::debug!("Generating {:?}", &pubpath);
    pubpath.create_with(&Publisher::generate())?;
    Ok(())
}

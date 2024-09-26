use std::path::PathBuf;

use anyhow::Result;
use anyhow_std::PathAnyhow;

pub fn setup(dataset: &str) -> Result<PathBuf> {
    let targetdir = find_target()?;
    let testcasedir = targetdir.join("testdata").join(dataset);

    dbg!(&testcasedir);

    testcasedir.remove_dir_all_anyhow().or_else(|e| {
        use std::io::ErrorKind::NotFound;

        match e.downcast_ref::<std::io::Error>() {
            Some(e) if e.kind() == NotFound => Ok(()),
            _ => Err(e),
        }
    })?;

    testcasedir.create_dir_all_anyhow()?;

    Ok(testcasedir)
}

fn find_target() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    for ancestor in cwd.ancestors() {
        let candidate = ancestor.join("target");
        if candidate.is_dir() {
            return Ok(candidate);
        }
    }
    anyhow::bail!(
        r#"could not find "target/" directory above {:?}"#,
        cwd.display()
    );
}

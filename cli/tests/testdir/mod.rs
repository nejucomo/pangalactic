use std::path::PathBuf;

use anyhow::Result;
use anyhow_std::PathAnyhow;

pub fn setup(dataset: &str) -> Result<PathBuf> {
    let testcasedir = PathBuf::from(get_path_string(dataset));
    dbg!(&testcasedir);

    testcasedir.remove_dir_all_anyhow().or_else(|e| {
        use std::io::ErrorKind::NotFound;

        match e.downcast_ref::<std::io::Error>() {
            Some(e) if e.kind() == NotFound => Ok(()),
            _ => Err(e),
        }
    })?;

    testcasedir.create_dir_all_anyhow()?;

    Ok(PathBuf::from(testcasedir))
}

pub fn get_path_string(dataset: &str) -> String {
    format!(
        "{}/cli_store_tests_data/{dataset}",
        env!("CARGO_TARGET_TMPDIR")
    )
}

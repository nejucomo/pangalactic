use std::path::PathBuf;

#[macro_export]
macro_rules! datadir {
    () => {
        $crate::datadir_for_crate(env!("CARGO_PKG_NAME"))
    };
}

#[derive(Debug, derive_more::From)]
pub enum InitError {
    UnsupportedPlatform,
    StdIo(std::io::Error),
}

pub fn datadir_for_crate(cratename: &str) -> Result<PathBuf, InitError> {
    use InitError::*;

    let pg = "pangalactic";

    let parts: Vec<&str> = cratename.splitn(1, "_").collect();
    assert!(parts[0] == pg);
    let cratenick = parts[1];

    let mut data = dirs::data_dir().ok_or(UnsupportedPlatform)?;
    data.push(pg);
    data.push(cratenick);

    std::fs::create_dir_all(&data)?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

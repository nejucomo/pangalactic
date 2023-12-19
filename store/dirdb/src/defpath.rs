use std::path::PathBuf;

pub fn default_path() -> PathBuf {
    dirs::data_local_dir()
        .expect("required `dirs::data_local_dir` unavailable on this system")
        .join("pangalactic")
        .join("dirdb")
}

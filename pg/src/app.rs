use pangalactic_appdirs::AppDirs;

pub fn get_appdirs() -> std::io::Result<AppDirs> {
    AppDirs::new(crate::APP_NAME)
}

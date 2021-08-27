use pangalactic_appdirs::AppDirs;

pub trait Execute {
    fn execute(self, dirs: AppDirs) -> std::io::Result<()>;
}

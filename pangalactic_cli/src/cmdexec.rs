use enum_dispatch::enum_dispatch;
use pangalactic_appdirs::AppDirs;

#[enum_dispatch]
pub trait Execute {
    fn execute(self, dirs: AppDirs) -> std::io::Result<()>;
}

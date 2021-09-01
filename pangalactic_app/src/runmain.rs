use crate::Command;

pub fn run_main<T: Command>() -> std::io::Result<()> {
    let cmd = T::from_args();
    cmd.execute()
}

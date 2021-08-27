pub trait Execute {
    fn execute(self) -> std::io::Result<()>;
}

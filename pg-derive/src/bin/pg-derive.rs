fn main() -> std::io::Result<()> {
    use pangalactic_app::Command;
    use structopt::StructOpt;

    pg_derive::Options::from_args().execute()
}

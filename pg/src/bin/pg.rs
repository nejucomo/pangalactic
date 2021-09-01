fn main() -> std::io::Result<()> {
    use pangalactic_app::Command;
    use structopt::StructOpt;

    pg::opts::Options::from_args().execute()
}

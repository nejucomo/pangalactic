use log;
use pangalactic;

#[derive(Debug, derive_more::From)]
enum Error {
    Pangalactic(pangalactic::Error),
    Logger(log::SetLoggerError),
}

fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new().init()?;

    println!("=== {} ===", env!("CARGO_PKG_NAME"));
    let mut args = std::env::args().skip(1);
    let guestpath = args.next().unwrap();
    assert_eq!(None, args.next());

    pangalactic::execute_path(guestpath)?;

    Ok(())
}

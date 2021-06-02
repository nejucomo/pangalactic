use std::path::PathBuf;

#[derive(Debug)]
pub enum Command {
    TestWasm(PathBuf),
    TestIpfs,
}

#[derive(Debug)]
pub enum Error {
    CommandMissing,
    CommandUnknown(String),
    ArgumentMissing,
    ArgumentUnexpected(String),
}

impl Command {
    pub fn parse_args<I>(mut args: I) -> Result<Command, Error>
    where
        I: Iterator<Item = String>,
    {
        parse_subcommand(&mut args, |subargs, arg| match arg {
            "test" => parse_test_args(subargs),
            _ => None,
        })
        .expect("parse_subcommand always returns Some(result...)")
        .and_then(|cmd| {
            if let Some(unexpected) = args.next() {
                Err(Error::ArgumentUnexpected(unexpected))
            } else {
                Ok(cmd)
            }
        })
    }
}

fn parse_test_args<I>(args: I) -> Option<Result<Command, Error>>
where
    I: Iterator<Item = String>,
{
    parse_subcommand(args, |subargs, arg| match arg {
        "ipfs" => Some(Ok(Command::TestIpfs)),
        "wasm" => parse_test_wasm_args(subargs),
        _ => None,
    })
}

fn parse_test_wasm_args<I>(mut args: I) -> Option<Result<Command, Error>>
where
    I: Iterator<Item = String>,
{
    if let Some(path) = args.next() {
        Some(Ok(Command::TestWasm(PathBuf::from(path))))
    } else {
        Some(Err(Error::ArgumentMissing))
    }
}

fn parse_subcommand<I, F>(mut args: I, f: F) -> Option<Result<Command, Error>>
where
    I: Iterator<Item = String>,
    F: FnOnce(I, &str) -> Option<Result<Command, Error>>,
{
    if let Some(arg) = args.next() {
        f(args, arg.as_str()).or(Some(Err(Error::CommandUnknown(arg))))
    } else {
        Some(Err(Error::CommandMissing))
    }
}

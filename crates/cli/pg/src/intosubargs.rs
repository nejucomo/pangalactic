use either::Either::{self, Left, Right};
use pangalactic_cli_derive::options as derive;
use pangalactic_cli_revcon::options as revcon;
use pangalactic_cli_store::options as store;

pub(crate) trait IntoSubArgs {
    fn into_args(self) -> impl IntoIterator<Item = String>;
}

fn subcommand<A>(s: &str, args: A) -> impl IntoIterator<Item = String>
where
    A: IntoSubArgs,
{
    Some(s.to_string()).into_iter().chain(args.into_args())
}

impl<L, R> IntoSubArgs for Either<L, R>
where
    L: IntoSubArgs,
    R: IntoSubArgs,
{
    fn into_args(self) -> impl IntoIterator<Item = String> {
        self.map_left(IntoSubArgs::into_args)
            .map_right(IntoSubArgs::into_args)
            .into_iter()
    }
}

impl IntoSubArgs for revcon::Command {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        use revcon::Command::*;

        match self {
            Info(opts) => subcommand("info", Left(opts)),
            Init(opts) => subcommand("init", Right(opts)),
        }
    }
}

impl IntoSubArgs for revcon::InfoOptions {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        self.detail
            .map(IntoSubArgs::into_args)
            .into_iter()
            .flatten()
    }
}

impl IntoSubArgs for revcon::InfoDetail {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        match self {
            revcon::InfoDetail::Path(opts) => subcommand("path", opts),
        }
    }
}

impl IntoSubArgs for revcon::InfoPathOptions {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        []
    }
}

impl IntoSubArgs for revcon::InitOptions {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        [
            "--workdir".to_string(),
            self.workdir
                .to_str()
                .unwrap_or_else(|| panic!("non-utf8 path: {:?}", self.workdir.display()))
                .to_string(),
        ]
    }
}

impl IntoSubArgs for store::Command {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        use store::Command::*;

        match self {
            Put(opts) => subcommand("put", Left(opts)),
            Get(opts) => subcommand("get", Right(Left(opts))),
            Xfer(opts) => subcommand("xfer", Right(Right(opts))),
        }
    }
}

impl IntoSubArgs for store::PutOptions {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        []
    }
}

impl IntoSubArgs for store::GetOptions {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        [self.source.to_string()]
    }
}

impl IntoSubArgs for store::XferOptions {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        self.excludes
            .exclude
            .into_iter()
            .flat_map(|glob| ["--exclude".to_string(), glob.to_string()])
            .chain([self.source.to_string(), self.dest.to_string()])
    }
}

impl IntoSubArgs for derive::Options {
    fn into_args(self) -> impl IntoIterator<Item = String> {
        Some(self.plan_or_exec)
            .into_iter()
            .chain(self.input)
            .map(|x| x.to_string())
    }
}

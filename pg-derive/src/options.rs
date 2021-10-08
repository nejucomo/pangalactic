use pangalactic_app::{CommonOptions, OutputCommand};
use pangalactic_storage::Link;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Derive a pangalactic result deterministically")]
pub struct Options {
    #[structopt(flatten)]
    common: CommonOptions,

    #[structopt(help = "The executable link")]
    exec: Link,

    #[structopt(help = "The input link")]
    input: Link,
}

impl OutputCommand for Options {
    type Output = Link;

    fn execute_output(&self) -> std::io::Result<Link> {
        use pangalactic_app::Command;
        use pangalactic_derive::derive;
        use pangalactic_storage::Storage;

        self.common.execute()?;

        let storage = Storage::open_default()?;
        let link = derive(storage.unwrap(), &self.exec, &self.input)?;
        Ok(link)
    }
}

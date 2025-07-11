use anyhow::Result;
use pangalactic_runopt::Application;

use crate::options::Options;

impl Application for Options {
    async fn run(self) -> Result<()> {
        todo!()
    }
}

// impl<A> RunApp<A> for Command
// where
//     A: Send,
//     GenerateOptions: RunApp<A>,
// {
//     async fn run_app(self, app: A) -> Result<()> {
//         use Command::*;

//         match self {
//             Generate(opts) => opts.run_app(app).await,
//         }
//     }
// }

// impl RunApp<()> for GenerateOptions {
//     async fn run_app(self, _: ()) -> Result<()> {
//         let pubcap = PublishCap::generate(rand::rng());
//         let pcbytes = serialize(&pubcap)?;
//         self.pubcapopts.pubcap.write_anyhow(pcbytes)?;

//         let subcap = pubcap.subscribe_cap();
//         let scbytes = b64::serialize(&subcap)?;
//         tokio::io::stdout().write_all(scbytes.as_bytes()).await?;
//         Ok(())
//     }
// }

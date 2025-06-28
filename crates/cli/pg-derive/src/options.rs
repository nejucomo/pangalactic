use clap::Parser;
use pangalactic_runopt::Application;
use pangalactic_schemata::Plan;
use pangalactic_std_store::{StdOrigin, StdStore};
use pangalactic_store_dirdb::DirDbStore;

/// Derive a plan
#[derive(Clone, Debug, Parser)]
pub struct Options {
    /// The path to the dirdb store directory
    #[clap(short, long, default_value_t)]
    pub dirdb: DirDbStore,

    /// The plan to derive, or an exec file if `INPUT` is provided
    pub plan_or_exec: StdOrigin,

    /// An input to derive; if absent `PLAN_OR_EXEC` must be a plan
    pub input: Option<StdOrigin>,
}

impl Application for Options {
    async fn run(self) -> anyhow::Result<()> {
        let mut store = StdStore::from(self.dirdb);

        // Transfer any source into the store to get a store path:
        // Assert: Final unwrap never fails because `DestinationEndpoint::Store` always produces a path:
        let exec = store.transfer(self.plan_or_exec, ()).await?;

        let plan = if let Some(input) = self.input {
            let input = store.transfer(input, ()).await?;
            store.commit(Plan { exec, input }).await?
        } else {
            exec
        };

        let (_, attestation) = store.derive(&plan).await?;
        println!("{attestation}");
        Ok(())
    }
}

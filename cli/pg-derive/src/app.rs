use anyhow::Result;
use pangalactic_runopt::{Application, RunOptions};
use pangalactic_schemata::Plan;
use pangalactic_std_store::StdStore;

use crate::options::DeriveOptions;

/// The standalone `pg-seed` application
#[derive(Debug, Default)]
pub struct DeriveApplication;

impl Application for DeriveApplication {
    type Options = DeriveOptions;
}

impl RunOptions<DeriveOptions> for DeriveApplication {
    async fn run_options(&self, options: DeriveOptions) -> Result<()> {
        let options = options.clone();

        let mut store = StdStore::default();

        // Transfer any source into the store to get a store path:
        // Assert: Final unwrap never fails because `DestinationEndpoint::Store` always produces a path:
        let exec = store.transfer(options.plan_or_exec, ()).await?;

        let plan = if let Some(input) = options.input {
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

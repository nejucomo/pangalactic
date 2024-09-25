use anyhow::Result;
use pangalactic_runopt::{Application, RunApp};
use pangalactic_schemata::Plan;
use pangalactic_std_store::StdStore;

use crate::options::Options;

/// The standalone `pg-seed` application
#[derive(Debug, Default)]
pub struct DeriveApplication;

impl Application for DeriveApplication {
    type Options = Options;
}

impl RunApp<DeriveApplication> for Options {
    async fn run_app(self, _: DeriveApplication) -> Result<()> {
        let mut store = StdStore::default();

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

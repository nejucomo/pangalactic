use std::future::Future;

use anyhow::Result;
use pangalactic_name::Name;

use crate::IntoSource;

pub trait BranchIter: Send {
    type IntoSource: IntoSource;

    fn next_branch_entry(
        &mut self,
    ) -> impl Future<Output = Result<Option<(Name, Self::IntoSource)>>> + Send;
}

use clap::Parser;
use pangalactic_std_store::StdOrigin;

/// Derive a plan
#[derive(Clone, Debug, Parser)]
pub struct Options {
    /// The plan to derive, or an exec file if `INPUT` is provided
    pub plan_or_exec: StdOrigin,

    /// An input to derive; if absent `PLAN_OR_EXEC` must be a plan
    pub input: Option<StdOrigin>,
}

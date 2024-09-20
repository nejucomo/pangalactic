use clap::Parser;
use pangalactic_endpoint::OriginEndpoint;
use pangalactic_hash::Hash;
use pangalactic_layer_cidmeta::CidMeta;

pub type DeriveOrigin = OriginEndpoint<CidMeta<Hash>>;

/// Derive a plan
#[derive(Clone, Debug, Parser)]
pub struct DeriveOptions {
    /// The plan to derive, or an exec file if `INPUT` is provided
    pub plan_or_exec: DeriveOrigin,

    /// An input to derive; if absent `PLAN_OR_EXEC` must be a plan
    pub input: Option<DeriveOrigin>,
}

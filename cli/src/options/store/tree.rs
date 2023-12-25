use crate::dagops::LinkDo;
use clap::{Args, Subcommand};
use std::path::PathBuf;

/// Low-level dir operations
#[derive(Debug, Subcommand)]
pub enum Command {
    Manifest(ManifestOptions),
    Import(ImportOptions),
    Export(ExportOptions),
}

/// Generate a manifest of the tree at `ROOT`
#[derive(Debug, Args)]
pub struct ManifestOptions {
    /// The root link
    pub root: LinkDo,
}

/// Generate a manifest of the tree at `ROOT`
#[derive(Debug, Args)]
pub struct ImportOptions {
    /// The import source
    pub src: PathBuf,
}

/// Generate a manifest of the tree at `ROOT`
#[derive(Debug, Args)]
pub struct ExportOptions {
    /// The root link to export
    pub root: LinkDo,
    /// The export destination
    pub dest: PathBuf,
}

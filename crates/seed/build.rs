#![feature(exit_status_error)]

use anyhow::Context;
use anyhow_std::PathAnyhow as _;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let r = main_inner();
    if let Some(e) = r.as_ref().err() {
        eprintln!("{e:#}");
    }
    r
}

fn main_inner() -> anyhow::Result<()> {
    use std::process::Command;

    let guestworkspace = Path::new("../../seed-crates");
    assert!(guestworkspace.is_dir(), "{:?}", guestworkspace.display());

    produce_rerun_if_changed_directives(guestworkspace)?;

    let mut cmd = Command::new(env!("CARGO"));
    cmd.arg("build");
    cmd.current_dir(guestworkspace);

    let status = cmd.status().with_context(|| format!("{:?}", &cmd))?;
    status.exit_ok().with_context(|| format!("{:?}", &cmd))?;

    let guesttarget = guestworkspace.join("target");
    let wasmdir = guesttarget.join("wasms");

    recreate_dir(&wasmdir)?;

    let debugdir = guesttarget.join("wasm32-unknown-unknown").join("debug");
    for entres in debugdir
        .read_dir()
        .with_context(|| format!("{:?}", debugdir.display()))?
    {
        let entry = entres?;
        if entry.file_type()?.is_file() {
            let path = entry.path();
            if path
                .extension()
                .map(|s| s.to_str() == Some("wasm"))
                .unwrap_or(false)
            {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let dst = wasmdir.join(file_name);
                std::fs::copy(&path, &dst)
                    .with_context(|| format!("from {:?} to {:?}", path.display(), dst.display()))?;
            }
        }
    }

    Ok(())
}

fn produce_rerun_if_changed_directives(seedcrates: &Path) -> anyhow::Result<()> {
    use walkdir::{DirEntry, WalkDir};

    let is_target = |e: &DirEntry| {
        e.file_name()
            .to_str()
            .map(|n| n == "target")
            .unwrap_or(false)
    };

    for entres in WalkDir::new(seedcrates).into_iter().filter_entry(is_target) {
        println!(
            "cargo:rerun-if-changed={}",
            entres?.file_name().to_str().unwrap()
        );
    }
    Ok(())
}

fn recreate_dir(dir: &Path) -> anyhow::Result<()> {
    dir.remove_dir_all_anyhow().or_else(|anyerr| {
        use std::io::ErrorKind::NotFound;

        if let Some(e) = anyerr.downcast_ref::<std::io::Error>() {
            if e.kind() == NotFound {
                return Ok(());
            }
        }

        Err(anyerr)
    })?;

    dir.create_dir_anyhow()?;

    Ok(())
}

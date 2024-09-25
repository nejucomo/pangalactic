#![feature(exit_status_error)]

use anyhow::Context;
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

    produce_rerun_if_changed_directives()?;

    let guestworkspace = Path::new("guests");

    let mut cmd = Command::new(env!("CARGO"));
    cmd.arg("build");
    cmd.current_dir(guestworkspace);

    let status = cmd.status().with_context(|| format!("{:?}", &cmd))?;
    status.exit_ok()?;

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

fn produce_rerun_if_changed_directives() -> anyhow::Result<()> {
    use walkdir::{DirEntry, WalkDir};

    let is_target = |e: &DirEntry| {
        e.file_name()
            .to_str()
            .map(|n| n == "target")
            .unwrap_or(false)
    };

    for entres in WalkDir::new("guests").into_iter().filter_entry(is_target) {
        println!(
            "cargo:rerun-if-changed={}",
            entres?.file_name().to_str().unwrap()
        );
    }
    Ok(())
}

fn recreate_dir(dir: &Path) -> anyhow::Result<()> {
    std::fs::remove_dir_all(dir)
        .or_else(|e| {
            use std::io::ErrorKind::NotFound;

            if e.kind() == NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })
        .with_context(|| format!("{:?}", dir.display()))?;

    std::fs::create_dir(dir).with_context(|| format!("{:?}", dir.display()))?;

    Ok(())
}

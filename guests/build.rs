#![feature(exit_status_error)]

fn main() -> anyhow::Result<()> {
    let r = main_inner();
    if let Some(e) = r.as_ref().err() {
        eprintln!("{:#}", e);
    }
    r
}

fn main_inner() -> anyhow::Result<()> {
    use anyhow::Context;
    use std::path::Path;
    use std::process::Command;

    let guestworkspace = Path::new("guests");

    let mut cmd = Command::new(env!("CARGO"));
    cmd.arg("build");
    cmd.current_dir(guestworkspace);

    let status = cmd.status().with_context(|| format!("{:?}", &cmd))?;
    status.exit_ok()?;

    let guesttarget = guestworkspace.join("target");
    let wasmdir = guesttarget.join("wasms");

    std::fs::remove_dir_all(&wasmdir)
        .or_else(|e| {
            use std::io::ErrorKind::NotFound;

            if e.kind() == NotFound {
                Ok(())
            } else {
                Err(e)
            }
        })
        .with_context(|| format!("{:?}", wasmdir.display()))?;

    std::fs::create_dir(&wasmdir).with_context(|| format!("{:?}", wasmdir.display()))?;

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
                let dst = wasmdir.join(path.file_name().unwrap());
                std::fs::copy(&path, &dst)
                    .with_context(|| format!("from {:?} to {:?}", path.display(), dst.display()))?;
            }
        }
    }

    Ok(())
}

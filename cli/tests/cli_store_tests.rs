#![feature(exit_status_error)]

use std::{
    path::{Path, PathBuf},
    process::ExitStatus,
};

use anyhow_std::PathAnyhow;
use test_case::test_case;

#[test_case("")]
#[test_case("Hello World!")]
fn put_get_round_trip(input: &str) -> anyhow::Result<()> {
    dbg!(std::process::id());

    let testcasedir = setup_test_case_dir(&format!(
        "put_get_round_trip-{}",
        input.replace(' ', "_").replace('!', "_")
    ))?;

    let rawlink = run_pg_ok(&testcasedir, &["store", "put"], input)?;
    let link = rawlink.trim();

    let output = run_pg_ok(&testcasedir, &["store", "get", link], "")?;
    assert_eq!(input, output);
    Ok(())
}

mod consts {
    pub const STDIN_CONTENTS: &'static str = "I am a stdin file.";
    pub const HOST_FILE_CONTENTS: &'static str = "I am a host file.";
    pub const STORE_FILE_CONTENTS: &'static str = "I am a store file.";
    pub const STORE_FILE_CONTENTS_2: &'static str = "I am also a store file.";

    pub const MKSOURCE_FILE_CID: &'static str =
        "pg:file-ddb--GvvRcHHjkJrbg4eN1NJ3Q0bsCEjhXsKS5DzmVprckAS";

    // Note: We wish we could evaluate this in const stage to remove redundancy:
    // pub const MKSOURCE_DIR_CID: &'static str = MKSOURCE_DIR_STORE_PATH.split_once('/').unwrap().0;
    pub const MKSOURCE_DIR_CID: &'static str =
        "pg:dir-ddb-xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW";
    pub const MKSOURCE_FILE_STORE_PATH: &'static str =
        "pg:dir-ddb-xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW/subdir/c";
    pub const MKSOURCE_DIR_STORE_PATH: &'static str =
        "pg:dir-ddb-xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW/subdir";

    pub const MKDEST_STORE_DEST: &'static str =
        "pg:dir-ddb-xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW/subdir/dest";

    pub const STDIN_TO_STORE_BARE: &'static str =
        "pg:file-ddb-QtBvYWotoTIPRBUkniYjLhNjgt65hkYUzj91Ax3yyyES";
    pub const STDIN_TO_STORE_DEST: &'static str =
        "pg:dir-ddb-E14qcnMPIy0hrURgpxygXDv628fgKtVeJtrNQB9Z_4RX";
    pub const HOST_FILE_TO_STORE_BARE: &'static str =
        "pg:file-ddb-VIs1dAsBTGIiYh92Nqk2Eeq0C6WaJfrhvPQi9tnYTacR";
    pub const HOST_FILE_TO_STORE_DEST: &'static str =
        "pg:dir-ddb-2FTxXVthHGe0DZxQXkIB27wGRd20H-F9-aUHcLXtamBX";
    pub const HOST_DIR_TO_STORE_DEST: &'static str =
        "pg:dir-ddb-GlR2nsXYF7oiQSgHAEQAb_TMBGkfQB7ZvNvwjMLSC3ZX";
    pub const STORE_CID_FILE_TO_STORE_DEST: &'static str =
        "pg:dir-ddb-AQSwJQ7qQsi58KESmz6izCd_DQDHv8-aUu3uwIxIUlRX";
    pub const STORE_CID_DIR_TO_STORE_DEST: &'static str = HOST_DIR_TO_STORE_DEST;
    pub const STORE_PATH_FILE_TO_STORE_BARE: &'static str =
        "pg:file-ddb-9haKuYOiSb5F0GTpXwoLbu2zqM2OfR9b8z48R3vXiCwX";
    pub const STORE_PATH_DIR_TO_STORE_BARE: &'static str =
        "pg:dir-ddb-JS-NoYzJP2xBPG-H4TEQuOyxOrsU4yUze5bV-9A2sHJu";
    pub const STORE_PATH_FILE_TO_STORE_DEST: &'static str =
        "pg:dir-ddb--UFyHlmmfl0BJLb__TznvYDCiOk2Fiad0Oo4cet5PUpX";
    pub const STORE_PATH_DIR_TO_STORE_DEST: &'static str =
        "pg:dir-ddb-Da29BWShEVjO74u6mYjxZumeuBsu_whlSxi_z1ZDNENX";
}

#[derive(Copy, Clone, Debug)]
enum FoD {
    File,
    Dir,
}
use FoD::*;

/// I specify which `Source` to setup and xfer from
#[derive(Copy, Clone, Debug)]
enum MkSource {
    Stdin,
    Host(FoD),
    StoreCID(FoD),
    StorePath(FoD),
}

/// I specify which `Destination` to xfer to
#[derive(Copy, Clone, Debug)]
enum MkDest {
    Stdout,
    Host,
    StoreBare,
    StoreDest,
}

impl MkSource {
    fn setup(self, testcasedir: &Path) -> anyhow::Result<()> {
        fn populate_host_dir(p: PathBuf) -> anyhow::Result<PathBuf> {
            p.create_dir_anyhow()?;
            p.join("file.txt").write_anyhow("Hello World!")?;
            let subdir = p.join("subdir");
            subdir.create_dir_anyhow()?;
            subdir.join("a").write_anyhow("Hello World!")?;
            subdir.join("b").write_anyhow("Honeybee")?;
            subdir
                .join("c")
                .write_anyhow(consts::STORE_FILE_CONTENTS_2)?;
            Ok(p)
        }

        let predir = populate_host_dir(testcasedir.join("presetup_dir"))?;

        {
            let cidspace = run_pg_ok(
                &testcasedir,
                &["store", "xfer", predir.to_str_anyhow()?, "pg:"],
                "",
            )?;
            assert_eq!(cidspace.trim_end(), StoreCID(Dir).to_arg());
        }

        use MkSource::*;

        match self {
            Host(File) => {
                testcasedir
                    .join("srcfile")
                    .write_anyhow(consts::HOST_FILE_CONTENTS)?;
            }
            Host(Dir) => {
                populate_host_dir(testcasedir.join("srcdir"))?;
            }
            StoreCID(File) | StorePath(File) => {
                let cidspace = run_pg_ok(
                    &testcasedir,
                    &["store", "xfer", "-", "pg:"],
                    consts::STORE_FILE_CONTENTS,
                )?;
                assert_eq!(cidspace.trim_end(), StoreCID(File).to_arg());
            }
            _ => {}
        }

        Ok(())
    }

    fn to_arg(self) -> &'static str {
        use MkSource::*;

        match self {
            Stdin => "-",
            Host(File) => "./srcfile",
            Host(Dir) => "./srcdir",
            StoreCID(File) => consts::MKSOURCE_FILE_CID,
            StoreCID(Dir) => consts::MKSOURCE_DIR_CID,
            StorePath(File) => consts::MKSOURCE_FILE_STORE_PATH,
            StorePath(Dir) => consts::MKSOURCE_DIR_STORE_PATH,
        }
    }

    fn stdin(self) -> &'static str {
        match self {
            MkSource::Stdin => consts::STDIN_CONTENTS,
            _ => "",
        }
    }

    fn verify_outcome(
        self,
        mkdest: MkDest,
        testcasedir: &Path,
        status: ExitStatus,
        output: String,
    ) -> anyhow::Result<()> {
        if let Some(expected) = self.expected_output(mkdest) {
            status.exit_ok()?;
            assert_eq!(output.trim_end(), expected);
            self.verify_host_dest(mkdest, testcasedir)?;
        } else {
            assert!(!status.success());
        }

        Ok(())
    }

    fn expected_output(self, mkdest: MkDest) -> Option<&'static str> {
        // BUG: The error logic here ignores overwrite errors:
        match (self, mkdest) {
            // Any dir headed to stdout is an error:
            (MkSource::Host(Dir), MkDest::Stdout)
            | (MkSource::StoreCID(Dir), MkDest::Stdout)
            | (MkSource::StorePath(Dir), MkDest::Stdout) => None,

            // Anything headed to host produces empty output without error:
            (_, MkDest::Host) => Some(""),

            // echo
            (MkSource::Stdin, MkDest::Stdout) => Some(MkSource::Stdin.stdin()),

            // cat
            (MkSource::Host(File), MkDest::Stdout) => Some(consts::HOST_FILE_CONTENTS),
            (MkSource::StoreCID(File), MkDest::Stdout) => Some(consts::STORE_FILE_CONTENTS),
            (MkSource::StorePath(File), MkDest::Stdout) => Some(consts::STORE_FILE_CONTENTS_2),

            // All writes into the store output a CID:
            (MkSource::Stdin, MkDest::StoreBare) => Some(consts::STDIN_TO_STORE_BARE),
            (MkSource::Stdin, MkDest::StoreDest) => Some(consts::STDIN_TO_STORE_DEST),
            (MkSource::Host(File), MkDest::StoreBare) => Some(consts::HOST_FILE_TO_STORE_BARE),
            (MkSource::Host(Dir), MkDest::StoreBare) => Some(consts::MKSOURCE_DIR_CID),
            (MkSource::Host(File), MkDest::StoreDest) => Some(consts::HOST_FILE_TO_STORE_DEST),
            (MkSource::Host(Dir), MkDest::StoreDest) => Some(consts::HOST_DIR_TO_STORE_DEST),

            // Copying any cid to `pg:` is a no-op because it's a deduplicated store:
            (MkSource::StoreCID(_), MkDest::StoreBare) => Some(self.to_arg()),
            (MkSource::StoreCID(File), MkDest::StoreDest) => {
                Some(consts::STORE_CID_FILE_TO_STORE_DEST)
            }
            (MkSource::StoreCID(Dir), MkDest::StoreDest) => {
                Some(consts::STORE_CID_DIR_TO_STORE_DEST)
            }
            (MkSource::StorePath(File), MkDest::StoreBare) => {
                Some(consts::STORE_PATH_FILE_TO_STORE_BARE)
            }
            (MkSource::StorePath(Dir), MkDest::StoreBare) => {
                Some(consts::STORE_PATH_DIR_TO_STORE_BARE)
            }
            (MkSource::StorePath(File), MkDest::StoreDest) => {
                Some(consts::STORE_PATH_FILE_TO_STORE_DEST)
            }
            (MkSource::StorePath(Dir), MkDest::StoreDest) => {
                Some(consts::STORE_PATH_DIR_TO_STORE_DEST)
            }
        }
    }

    fn verify_host_dest(self, mkdest: MkDest, testcasedir: &Path) -> anyhow::Result<()> {
        if matches!(mkdest, MkDest::Host) {
            use MkSource::*;
            let destpath = testcasedir.join(mkdest.to_arg());

            let (expectedname, optcontents) = match self {
                Stdin => ("_stdin_for_comparison", Some(self.stdin())),
                StoreCID(File) => ("_cidfile_for_comparison", Some(consts::STORE_FILE_CONTENTS)),
                StorePath(File) => (
                    "_cidpathfile_for_comparison",
                    Some(consts::STORE_FILE_CONTENTS_2),
                ),

                Host(_) => (self.to_arg(), None),
                StoreCID(Dir) => ("presetup_dir", None),
                StorePath(Dir) => ("presetup_dir/subdir", None),
            };

            let expectedpath = testcasedir.join(expectedname);
            if let Some(contents) = optcontents {
                expectedpath.write_anyhow(contents)?;
            }

            check_paths_equal(&expectedpath, &destpath)?;
        }
        Ok(())
    }
}

impl MkDest {
    fn to_arg(self) -> &'static str {
        use MkDest::*;

        match self {
            Stdout => "-",
            Host => "dest",
            StoreBare => "pg:",
            StoreDest => consts::MKDEST_STORE_DEST,
        }
    }
}

#[test_case(MkSource::Stdin, MkDest::Stdout)]
#[test_case(MkSource::Stdin, MkDest::Host)]
#[test_case(MkSource::Stdin, MkDest::StoreBare)]
#[test_case(MkSource::Stdin, MkDest::StoreDest)]
#[test_case(MkSource::Host(File), MkDest::Stdout)]
#[test_case(MkSource::Host(Dir), MkDest::Stdout)]
#[test_case(MkSource::Host(File), MkDest::Host)]
#[test_case(MkSource::Host(Dir), MkDest::Host)]
#[test_case(MkSource::Host(File), MkDest::StoreBare)]
#[test_case(MkSource::Host(Dir), MkDest::StoreBare)]
#[test_case(MkSource::Host(File), MkDest::StoreDest)]
#[test_case(MkSource::Host(Dir), MkDest::StoreDest)]
#[test_case(MkSource::StoreCID(File), MkDest::Stdout)]
#[test_case(MkSource::StoreCID(Dir), MkDest::Stdout)]
#[test_case(MkSource::StoreCID(File), MkDest::Host)]
#[test_case(MkSource::StoreCID(Dir), MkDest::Host)]
#[test_case(MkSource::StoreCID(File), MkDest::StoreBare)]
#[test_case(MkSource::StoreCID(Dir), MkDest::StoreBare)]
#[test_case(MkSource::StoreCID(File), MkDest::StoreDest)]
#[test_case(MkSource::StoreCID(Dir), MkDest::StoreDest)]
#[test_case(MkSource::StorePath(File), MkDest::Stdout)]
#[test_case(MkSource::StorePath(Dir), MkDest::Stdout)]
#[test_case(MkSource::StorePath(File), MkDest::Host)]
#[test_case(MkSource::StorePath(Dir), MkDest::Host)]
#[test_case(MkSource::StorePath(File), MkDest::StoreBare)]
#[test_case(MkSource::StorePath(Dir), MkDest::StoreBare)]
#[test_case(MkSource::StorePath(File), MkDest::StoreDest)]
#[test_case(MkSource::StorePath(Dir), MkDest::StoreDest)]
fn xfer(mksource: MkSource, mkdest: MkDest) -> anyhow::Result<()> {
    let testcasedir = setup_test_case_dir(&format!("xfer_{mksource:?}_{mkdest:?}"))?;

    mksource.setup(&testcasedir)?;

    let (status, output) = run_pg(
        &testcasedir,
        &["store", "xfer", &mksource.to_arg(), &mkdest.to_arg()],
        &mksource.stdin(),
    )?;
    mksource.verify_outcome(mkdest, &testcasedir, status, output)
}

fn setup_test_case_dir(dataset: &str) -> anyhow::Result<PathBuf> {
    let testcasedir = PathBuf::from(get_test_case_dir(dataset));
    dbg!(&testcasedir);

    testcasedir.remove_dir_all_anyhow().or_else(|e| {
        use std::io::ErrorKind::NotFound;

        match e.downcast_ref::<std::io::Error>() {
            Some(e) if e.kind() == NotFound => Ok(()),
            _ => Err(e),
        }
    })?;

    testcasedir.create_dir_all_anyhow()?;

    Ok(PathBuf::from(testcasedir))
}

fn get_test_case_dir(dataset: &str) -> String {
    format!(
        "{}/cli_store_tests_data/{dataset}",
        env!("CARGO_TARGET_TMPDIR")
    )
}

fn run_pg_ok(testcasedir: &Path, args: &[&str], stdin: &str) -> anyhow::Result<String> {
    let (status, output) = run_pg(testcasedir, args, stdin)?;
    status.exit_ok()?;
    Ok(output)
}

fn run_pg(testcasedir: &Path, args: &[&str], stdin: &str) -> anyhow::Result<(ExitStatus, String)> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_pg"));
    cmd.args(args);
    cmd.env("XDG_DATA_HOME", testcasedir);
    cmd.current_dir(testcasedir);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = cmd.spawn()?;

    child.stdin.as_ref().unwrap().write_all(stdin.as_bytes())?;
    let cmdout = child.wait_with_output()?;

    println!("{}", String::from_utf8(cmdout.stderr)?);

    let outtext = String::from_utf8(cmdout.stdout)?;
    Ok((cmdout.status, outtext))
}

fn check_paths_equal(src: &Path, dst: &Path) -> anyhow::Result<()> {
    use anyhow::Context;

    check_paths_equal_inner(src, dst)
        .with_context(|| format!("{:?} != {:?}", src.display(), dst.display()))
}

fn check_paths_equal_inner(src: &Path, dst: &Path) -> anyhow::Result<()> {
    #[derive(Debug, PartialEq)]
    enum Ftype {
        File,
        Dir,
    }
    use Ftype::*;

    fn file_type(p: &Path) -> anyhow::Result<Ftype> {
        if p.is_file() {
            Ok(File)
        } else if p.is_dir() {
            Ok(Dir)
        } else {
            anyhow::bail!("unknown fs type: {:?}", p.display());
        }
    }

    match (file_type(src)?, file_type(dst)?) {
        (File, File) => {
            let srcvec = src.read_anyhow()?;
            let dstvec = dst.read_anyhow()?;
            if srcvec == dstvec {
                Ok(())
            } else {
                anyhow::bail!("files not equal");
            }
        }
        (Dir, Dir) => {
            use std::collections::BTreeSet;

            let mut visitedsrc = BTreeSet::default();

            for entres in src.read_dir_anyhow()? {
                let entry = entres?;
                let suffix = entry.path().strip_prefix_anyhow(src)?.to_path_buf();
                visitedsrc.insert(suffix);
            }

            for entres in dst.read_dir_anyhow()? {
                let dstpath = entres?.path();
                let suffix = dstpath.strip_prefix_anyhow(dst)?;
                let srcpath = src.join(suffix);
                if visitedsrc.remove(suffix) {
                    check_paths_equal_inner(&srcpath, &dstpath)?;
                } else {
                    anyhow::bail!("missing: {:?}", srcpath.display());
                }
            }

            Ok(())
        }
        (l, r) => {
            anyhow::bail!("unmatched fs types: {l:?} != {r:?}");
        }
    }
}

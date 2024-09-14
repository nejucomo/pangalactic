mod runner;
mod testdir;

use std::path::{Path, PathBuf};

use anyhow::Result;
use anyhow_std::PathAnyhow;
use test_case::test_case;

use self::runner::{Output, Runner};

#[test_case("")]
#[test_case("Hello World!")]
fn put_get_round_trip(input: &str) -> Result<()> {
    let testcasedir = testdir::setup(&format!(
        "put_get_round_trip-{}",
        input.replace(' ', "_").replace('!', "_")
    ))?;

    let runner = Runner::new(&testcasedir, ["util", "store"]);
    let rawlink = runner.pg(["put"], input)?.exit_ok()?;
    let link = rawlink.trim();

    let output = runner.pg(["get", link], "")?.exit_ok()?;
    assert_eq!(input, output);
    Ok(())
}

mod consts {
    pub const STDIN_CONTENTS: &'static str = "I am a stdin file.";
    pub const HOST_FILE_CONTENTS: &'static str = "I am a host file.";
    pub const STORE_FILE_CONTENTS: &'static str = "I am a store file.";
    pub const STORE_FILE_CONTENTS_2: &'static str = "I am also a store file.";

    pub const MKSOURCE_FILE_CID: &'static str = "pg:F:-GvvRcHHjkJrbg4eN1NJ3Q0bsCEjhXsKS5DzmVprckAS";

    // Note: We wish we could evaluate this in const stage to remove redundancy:
    // pub const MKSOURCE_DIR_CID: &'static str = MKSOURCE_DIR_STORE_PATH.split_once('/').unwrap().0;
    pub const MKSOURCE_DIR_CID: &'static str = "pg:D:xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW";
    pub const MKSOURCE_FILE_STORE_PATH: &'static str =
        "pg:D:xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW/subdir/c";
    pub const MKSOURCE_DIR_STORE_PATH: &'static str =
        "pg:D:xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW/subdir";

    pub const MKDEST_HOST_DEST: &'static str = "dest";
    pub const MKDEST_STORE_DEST: &'static str =
        "pg:D:xB2_Y8LhYxhm1J0xd8kMWmKJ6x14_214vIXZlRAU3xdW/subdir/dest";

    pub const STDIN_TO_STORE_BARE: &'static str =
        "pg:F:QtBvYWotoTIPRBUkniYjLhNjgt65hkYUzj91Ax3yyyES";
    pub const STDIN_TO_STORE_DEST: &'static str =
        "pg:D:E14qcnMPIy0hrURgpxygXDv628fgKtVeJtrNQB9Z_4RX/subdir/dest";
    pub const HOST_FILE_TO_STORE_BARE: &'static str =
        "pg:F:VIs1dAsBTGIiYh92Nqk2Eeq0C6WaJfrhvPQi9tnYTacR";
    pub const HOST_FILE_TO_STORE_DEST: &'static str =
        "pg:D:2FTxXVthHGe0DZxQXkIB27wGRd20H-F9-aUHcLXtamBX/subdir/dest";
    pub const HOST_DIR_TO_STORE_DEST: &'static str =
        "pg:D:GlR2nsXYF7oiQSgHAEQAb_TMBGkfQB7ZvNvwjMLSC3ZX/subdir/dest";
    pub const STORE_CID_FILE_TO_STORE_DEST: &'static str =
        "pg:D:AQSwJQ7qQsi58KESmz6izCd_DQDHv8-aUu3uwIxIUlRX/subdir/dest";
    pub const STORE_CID_DIR_TO_STORE_DEST: &'static str = HOST_DIR_TO_STORE_DEST;
    pub const STORE_PATH_FILE_TO_STORE_BARE: &'static str =
        "pg:F:9haKuYOiSb5F0GTpXwoLbu2zqM2OfR9b8z48R3vXiCwX";
    pub const STORE_PATH_DIR_TO_STORE_BARE: &'static str =
        "pg:D:JS-NoYzJP2xBPG-H4TEQuOyxOrsU4yUze5bV-9A2sHJu";
    pub const STORE_PATH_FILE_TO_STORE_DEST: &'static str =
        "pg:D:-UFyHlmmfl0BJLb__TznvYDCiOk2Fiad0Oo4cet5PUpX/subdir/dest";
    pub const STORE_PATH_DIR_TO_STORE_DEST: &'static str =
        "pg:D:Da29BWShEVjO74u6mYjxZumeuBsu_whlSxi_z1ZDNENX/subdir/dest";
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
    LinkPath(FoD),
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
    fn setup<'a>(self, runner: &Runner<'a>) -> Result<()> {
        fn populate_host_dir(p: PathBuf) -> Result<PathBuf> {
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

        let predir = populate_host_dir(runner.testcasedir.join("presetup_dir"))?;

        {
            let cidspace = runner
                .pg(["xfer", predir.to_str_anyhow()?, "pg:"], "")?
                .exit_ok()?;
            assert_eq!(cidspace.trim_end(), StoreCID(Dir).to_arg());
        }

        use MkSource::*;

        match self {
            Host(File) => {
                runner
                    .testcasedir
                    .join("srcfile")
                    .write_anyhow(consts::HOST_FILE_CONTENTS)?;
            }
            Host(Dir) => {
                populate_host_dir(runner.testcasedir.join("srcdir"))?;
            }
            StoreCID(File) | LinkPath(File) => {
                let cidspace = runner
                    .pg(["xfer", "-", "pg:"], consts::STORE_FILE_CONTENTS)?
                    .exit_ok()?;
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
            LinkPath(File) => consts::MKSOURCE_FILE_STORE_PATH,
            LinkPath(Dir) => consts::MKSOURCE_DIR_STORE_PATH,
        }
    }

    fn stdin(self) -> &'static str {
        match self {
            MkSource::Stdin => consts::STDIN_CONTENTS,
            _ => "",
        }
    }

    fn verify_outcome(self, mkdest: MkDest, testcasedir: &Path, output: Output) -> Result<()> {
        if let Some((constname, expected)) = self.expected_output(mkdest) {
            let actual = output.exit_ok()?;
            assert_eq!(
                actual.trim_end(),
                expected,
                "mismatched const {constname:?}"
            );
            self.verify_host_dest(mkdest, testcasedir)?;
        } else {
            assert!(!output.status.success());
        }

        Ok(())
    }

    fn expected_output(self, mkdest: MkDest) -> Option<(&'static str, &'static str)> {
        macro_rules! named_const {
            ( $constname:ident ) => {
                Some((stringify!($constname), consts::$constname))
            };
        }

        // BUG: The error logic here ignores overwrite errors:
        match (self, mkdest) {
            // Any dir headed to stdout is an error:
            (MkSource::Host(Dir), MkDest::Stdout)
            | (MkSource::StoreCID(Dir), MkDest::Stdout)
            | (MkSource::LinkPath(Dir), MkDest::Stdout) => None,

            // Any host dest outputs the host path:
            (_, MkDest::Host) => named_const!(MKDEST_HOST_DEST),

            // echo
            (MkSource::Stdin, MkDest::Stdout) => named_const!(STDIN_CONTENTS),

            // cat
            (MkSource::Host(File), MkDest::Stdout) => named_const!(HOST_FILE_CONTENTS),
            (MkSource::StoreCID(File), MkDest::Stdout) => named_const!(STORE_FILE_CONTENTS),
            (MkSource::LinkPath(File), MkDest::Stdout) => named_const!(STORE_FILE_CONTENTS_2),

            // All writes into the store output a CID:
            (MkSource::Stdin, MkDest::StoreBare) => named_const!(STDIN_TO_STORE_BARE),
            (MkSource::Stdin, MkDest::StoreDest) => named_const!(STDIN_TO_STORE_DEST),
            (MkSource::Host(File), MkDest::StoreBare) => named_const!(HOST_FILE_TO_STORE_BARE),
            (MkSource::Host(Dir), MkDest::StoreBare) => named_const!(MKSOURCE_DIR_CID),
            (MkSource::Host(File), MkDest::StoreDest) => named_const!(HOST_FILE_TO_STORE_DEST),
            (MkSource::Host(Dir), MkDest::StoreDest) => named_const!(HOST_DIR_TO_STORE_DEST),
            (MkSource::StoreCID(File), MkDest::StoreBare) => named_const!(MKSOURCE_FILE_CID),
            (MkSource::StoreCID(Dir), MkDest::StoreBare) => named_const!(MKSOURCE_DIR_CID),
            (MkSource::StoreCID(File), MkDest::StoreDest) => {
                named_const!(STORE_CID_FILE_TO_STORE_DEST)
            }
            (MkSource::StoreCID(Dir), MkDest::StoreDest) => {
                named_const!(STORE_CID_DIR_TO_STORE_DEST)
            }
            (MkSource::LinkPath(File), MkDest::StoreBare) => {
                named_const!(STORE_PATH_FILE_TO_STORE_BARE)
            }
            (MkSource::LinkPath(Dir), MkDest::StoreBare) => {
                named_const!(STORE_PATH_DIR_TO_STORE_BARE)
            }
            (MkSource::LinkPath(File), MkDest::StoreDest) => {
                named_const!(STORE_PATH_FILE_TO_STORE_DEST)
            }
            (MkSource::LinkPath(Dir), MkDest::StoreDest) => {
                named_const!(STORE_PATH_DIR_TO_STORE_DEST)
            }
        }
    }

    fn verify_host_dest(self, mkdest: MkDest, testcasedir: &Path) -> Result<()> {
        if matches!(mkdest, MkDest::Host) {
            use MkSource::*;
            let destpath = testcasedir.join(mkdest.to_arg());

            let (expectedname, optcontents) = match self {
                Stdin => ("_stdin_for_comparison", Some(self.stdin())),
                StoreCID(File) => ("_cidfile_for_comparison", Some(consts::STORE_FILE_CONTENTS)),
                LinkPath(File) => (
                    "_cidpathfile_for_comparison",
                    Some(consts::STORE_FILE_CONTENTS_2),
                ),

                Host(_) => (self.to_arg(), None),
                StoreCID(Dir) => ("presetup_dir", None),
                LinkPath(Dir) => ("presetup_dir/subdir", None),
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
            Host => consts::MKDEST_HOST_DEST,
            StoreBare => "pg:",
            StoreDest => consts::MKDEST_STORE_DEST,
        }
    }
}

#[test_case(MkSource::Stdin, false, MkDest::Stdout)]
#[test_case(MkSource::Stdin, true, MkDest::Stdout)]
#[test_case(MkSource::Stdin, false, MkDest::Host)]
#[test_case(MkSource::Stdin, true, MkDest::Host)]
#[test_case(MkSource::Stdin, false, MkDest::StoreBare)]
#[test_case(MkSource::Stdin, true, MkDest::StoreBare)]
#[test_case(MkSource::Stdin, false, MkDest::StoreDest)]
#[test_case(MkSource::Stdin, true, MkDest::StoreDest)]
#[test_case(MkSource::Host(File), false, MkDest::Stdout)]
#[test_case(MkSource::Host(File), true, MkDest::Stdout)]
#[test_case(MkSource::Host(Dir), false, MkDest::Stdout)]
#[test_case(MkSource::Host(Dir), true, MkDest::Stdout)]
#[test_case(MkSource::Host(File), false, MkDest::Host)]
#[test_case(MkSource::Host(File), true, MkDest::Host)]
#[test_case(MkSource::Host(Dir), false, MkDest::Host)]
#[test_case(MkSource::Host(Dir), true, MkDest::Host)]
#[test_case(MkSource::Host(File), false, MkDest::StoreBare)]
#[test_case(MkSource::Host(File), true, MkDest::StoreBare)]
#[test_case(MkSource::Host(Dir), false, MkDest::StoreBare)]
#[test_case(MkSource::Host(Dir), true, MkDest::StoreBare)]
#[test_case(MkSource::Host(File), false, MkDest::StoreDest)]
#[test_case(MkSource::Host(File), true, MkDest::StoreDest)]
#[test_case(MkSource::Host(Dir), false, MkDest::StoreDest)]
#[test_case(MkSource::Host(Dir), true, MkDest::StoreDest)]
#[test_case(MkSource::StoreCID(File), false, MkDest::Stdout)]
#[test_case(MkSource::StoreCID(File), true, MkDest::Stdout)]
#[test_case(MkSource::StoreCID(Dir), false, MkDest::Stdout)]
#[test_case(MkSource::StoreCID(Dir), true, MkDest::Stdout)]
#[test_case(MkSource::StoreCID(File), false, MkDest::Host)]
#[test_case(MkSource::StoreCID(File), true, MkDest::Host)]
#[test_case(MkSource::StoreCID(Dir), false, MkDest::Host)]
#[test_case(MkSource::StoreCID(Dir), true, MkDest::Host)]
#[test_case(MkSource::StoreCID(File), false, MkDest::StoreBare)]
#[test_case(MkSource::StoreCID(File), true, MkDest::StoreBare)]
#[test_case(MkSource::StoreCID(Dir), false, MkDest::StoreBare)]
#[test_case(MkSource::StoreCID(Dir), true, MkDest::StoreBare)]
#[test_case(MkSource::StoreCID(File), false, MkDest::StoreDest)]
#[test_case(MkSource::StoreCID(File), true, MkDest::StoreDest)]
#[test_case(MkSource::StoreCID(Dir), false, MkDest::StoreDest)]
#[test_case(MkSource::StoreCID(Dir), true, MkDest::StoreDest)]
#[test_case(MkSource::LinkPath(File), false, MkDest::Stdout)]
#[test_case(MkSource::LinkPath(File), true, MkDest::Stdout)]
#[test_case(MkSource::LinkPath(Dir), false, MkDest::Stdout)]
#[test_case(MkSource::LinkPath(Dir), true, MkDest::Stdout)]
#[test_case(MkSource::LinkPath(File), false, MkDest::Host)]
#[test_case(MkSource::LinkPath(File), true, MkDest::Host)]
#[test_case(MkSource::LinkPath(Dir), false, MkDest::Host)]
#[test_case(MkSource::LinkPath(Dir), true, MkDest::Host)]
#[test_case(MkSource::LinkPath(File), false, MkDest::StoreBare)]
#[test_case(MkSource::LinkPath(File), true, MkDest::StoreBare)]
#[test_case(MkSource::LinkPath(Dir), false, MkDest::StoreBare)]
#[test_case(MkSource::LinkPath(Dir), true, MkDest::StoreBare)]
#[test_case(MkSource::LinkPath(File), false, MkDest::StoreDest)]
#[test_case(MkSource::LinkPath(File), true, MkDest::StoreDest)]
#[test_case(MkSource::LinkPath(Dir), false, MkDest::StoreDest)]
#[test_case(MkSource::LinkPath(Dir), true, MkDest::StoreDest)]
fn xfer(mksource: MkSource, with_exclude: bool, mkdest: MkDest) -> Result<()> {
    let testcasedir = testdir::setup(&format!("xfer_{mksource:?}_{with_exclude:?}_{mkdest:?}"))?;

    let runner = Runner::new(&testcasedir, ["util", "store"]);
    mksource.setup(&runner)?;

    let mut args = vec!["xfer"];
    if with_exclude {
        args.extend(["--exclude", "xxx"]);
    }
    args.extend([&mksource.to_arg(), &mkdest.to_arg()]);

    let runout = runner.pg(args, &mksource.stdin())?;
    mksource.verify_outcome(mkdest, &runner.testcasedir, runout)
}

fn check_paths_equal(src: &Path, dst: &Path) -> Result<()> {
    use anyhow::Context;

    check_paths_equal_inner(src, dst)
        .with_context(|| format!("{:?} != {:?}", src.display(), dst.display()))
}

fn check_paths_equal_inner(src: &Path, dst: &Path) -> Result<()> {
    #[derive(Debug, PartialEq)]
    enum Ftype {
        File,
        Dir,
    }
    use Ftype::*;

    fn file_type(p: &Path) -> Result<Ftype> {
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

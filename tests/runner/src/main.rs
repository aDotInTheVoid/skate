use std::convert::TryFrom;
use std::fs;
use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};
use structopt::StructOpt;

const THIS_CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const TREEWALK_CMD_NAME: &str = "crun";

// TODO: Release mode

#[derive(StructOpt)]
struct Args {
    files: Vec<Utf8PathBuf>,
}

struct Conf {
    tree_walk_cmd: Utf8PathBuf,
    test_root_dir: Utf8PathBuf,
}

fn main() -> eyre::Result<()> {
    let opt = Args::from_args();

    let status = Command::new("cargo")
        .args(&["build", "--bins", "--workspace"])
        .spawn()?
        .wait()?;

    if !status.success() {
        eyre::bail!("Failed to build packages");
    }

    let target_dir = Utf8PathBuf::from(THIS_CRATE_ROOT)
        .join("..")
        .join("..")
        .join("target")
        .join("debug");

    let tree_walk_cmd = target_dir.join(TREEWALK_CMD_NAME);

    let (test_root_dir, paths) = if opt.files.is_empty() {
        let test_root_dir = Utf8PathBuf::from(THIS_CRATE_ROOT).join("..");

        let run_pass = test_root_dir.join("run-pass").join("**").join("*.sk");

        let paths: Vec<_> = glob::glob(
            run_pass
                .as_os_str()
                .to_str()
                .ok_or_else(|| eyre::eyre!("Can't convert {:?} to string", run_pass))?,
        )
        .unwrap()
        .collect::<Result<_, _>>()?;

        let paths: Vec<_> = paths
            .into_iter()
            .map(Utf8PathBuf::try_from)
            .collect::<Result<_, _>>()?;

        println!("Discoverd {} run-pass tests", paths.len());

        (test_root_dir, paths)
    } else {
        (Utf8PathBuf::from(""), opt.files)
    };

    let conf = Conf {
        test_root_dir,
        tree_walk_cmd,
    };

    let mut pass = 0;
    let mut fail = 0;

    for i in paths {
        let i = Utf8PathBuf::try_from(i)?;
        let relative_path = i.strip_prefix(&conf.test_root_dir)?;
        match run_pass_test(&i, &conf)? {
            TestResult::Success => {
                eprintln!("PASSED: {}", relative_path);
                pass += 1
            }
            TestResult::Failure(why) => {
                eprintln!("FAILED: {} {}", relative_path, why);
                fail += 1;
            }
        }
    }
    eprintln!("{} passed, {} failed", pass, fail);
    if fail != 0 {
        eyre::bail!("Failed Tests")
    }

    Ok(())
}

enum TestResult {
    Success,
    Failure(String),
}

fn run_pass_test(src: &Utf8Path, conf: &Conf) -> eyre::Result<TestResult> {
    let mut output_path = src.to_owned();
    output_path.set_extension("stdout");

    let expected_output = fs::read_to_string(output_path)?;

    let output = Command::new(&conf.tree_walk_cmd).arg(&src).output()?;
    if !output.status.success() {
        return Ok(TestResult::Failure(format!(
            "Exited with failure {:?}\n --- stderr ---\n{}\n--- stdout ---\n{}\n --- ",
            output.status,
            String::from_utf8(output.stderr)?,
            String::from_utf8(output.stdout)?
        )));
    }

    let got = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    Ok(if got == expected_output {
        TestResult::Success
    } else {
        TestResult::Failure(format!(
            "--- expected ---\n{}\n--- got --- \n{}\n--- stderr ---\n{}\n---",
            expected_output, got, stderr
        ))
    })
}

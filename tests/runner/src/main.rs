use std::convert::TryFrom;
use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};
use fs_err as fs;
// use structopt::StructOpt;

const THIS_CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
const TREEWALK_CMD_NAME: &str = "crun";

// TODO: Release mode

fn main() -> eyre::Result<()> {
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

    let test_root_dir = Utf8PathBuf::from(THIS_CRATE_ROOT).join("..");

    let mut pass = Vec::new();
    let mut fail = 0;

    let mut afail = 0;

    for (name, func, must_pass) in [
        ("run-pass", run_pass_test as TestFn, true),
        ("compile-fail", compile_fail_test as TestFn, true),
        ("run-fail", compile_fail_test as TestFn, false),
    ] {
        let glob_path = test_root_dir.join(name).join("**").join("*.sk");

        let paths: Vec<_> = glob::glob(glob_path.as_str())
            .unwrap()
            .collect::<Result<_, _>>()?;

        let paths: Vec<_> = paths
            .into_iter()
            .map(Utf8PathBuf::try_from)
            .collect::<Result<_, _>>()?;

        println!("Running {} tests in {}", paths.len(), name);

        for i in paths {
            let relative_path = i.strip_prefix(&test_root_dir)?.to_owned();

            // If tests are hanging, uncomment this
            // eprint!("{}", relative_path);
            // std::io::stdout().flush()?;
            match func(&i, &tree_walk_cmd, &test_root_dir)? {
                TestResult::Success => {
                    eprintln!("PASSED: {}", relative_path);
                    pass.push(relative_path);
                }
                TestResult::Failure(why) => {
                    eprintln!("FAILED: {} {}", relative_path, why);
                    if must_pass {
                        fail += 1;
                    } else {
                        afail += 1;
                    }
                }
            }
        }
    }

    eprintln!(
        "{} passed, {} failed, {} allowed fails",
        pass.len(),
        fail,
        afail
    );

    let mut buff = String::new();
    pass.sort_unstable();
    for i in pass {
        buff.push_str(i.as_str());
        buff.push('\n');
    }

    fs::write(Utf8PathBuf::from(THIS_CRATE_ROOT).join("results"), buff)?;

    if fail != 0 {
        eyre::bail!("Failed Tests")
    }

    Ok(())
}

enum TestResult {
    Success,
    Failure(String),
}

type TestFn =
    for<'r, 's, 't> fn(&'r Utf8Path, &'s Utf8Path, &'t Utf8Path) -> eyre::Result<TestResult>;

fn compile_fail_test(
    src: &Utf8Path,
    tree_walk_cmd: &Utf8Path,
    test_root: &Utf8Path,
) -> eyre::Result<TestResult> {
    let mut output_path = src.to_owned();
    output_path.set_extension("stderr");

    let expected_output = fs::read_to_string(output_path)?;

    let output = Command::new(tree_walk_cmd)
        .arg(&src)
        .env("NO_COLOR", "1")
        .output()?;
    if output.status.success() {
        return Ok(TestResult::Failure("Unexpected success".to_string()));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    let stderr = stderr.replace(test_root.as_str(), "$DD/tests");

    if stderr == expected_output {
        Ok(TestResult::Success)
    } else {
        Ok(TestResult::Failure(format!(
            "--- expected ---\n{}\n--- got ---\n{}\n--- stdout ---\n{}\n---",
            expected_output, stderr, stdout,
        )))
    }
}

fn run_pass_test(
    src: &Utf8Path,
    tree_walk_cmd: &Utf8Path,
    _: &Utf8Path,
) -> eyre::Result<TestResult> {
    let mut output_path = src.to_owned();
    output_path.set_extension("stdout");

    let expected_output = fs::read_to_string(output_path)?;

    let output = Command::new(tree_walk_cmd).arg(&src).output()?;
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

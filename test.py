#!/usr/bin/env python3
import glob
from os import path
import os
import subprocess
import pathlib
import argparse

# TODO: Show num success, num fail
# TODO: Always run all tests, and show which ones failed

parser = argparse.ArgumentParser()
parser.add_argument("-b", "--bless", help="Update the tests", action="store_true")
parser.add_argument(
    "-s", "--skip-build", help="Skip rebuilding the binary", action="store_true"
)
parser.add_argument(
    "-r", "--release", help="Build skate in release_mode", action="store_true"
)
parser.add_argument("--coverage", action="store_true")
args = parser.parse_args()

BASE_DIR = path.realpath(path.dirname(__file__))
os.chdir(BASE_DIR)
if not (args.skip_build or args.coverage):
    rflags = "--release" if args.release else ""
    os.system(f"cargo build {rflags}")
    os.system(f"cargo test {rflags}")

if args.coverage:
    if path.exists("./profile"):
        print("WARNING: profile dir exists")
    global_env = {"LLVM_PROFILE_FILE": "./cov/data/profile.%p.profraw"}
else:
    global_env = {}


# TODO: Allow release mode
SKATE_BINARY = (
    path.join(BASE_DIR, "target", "debug", "skate")
    if not args.coverage
    else path.join(BASE_DIR, "cov", "target", "debug", "skate")
)

TEST_DIR = path.join(BASE_DIR, "tests")

SK_GLOB = path.join("**", "*.sk")
mkglob = lambda dir: path.join(TEST_DIR, dir, SK_GLOB)

RUN_PASS_GLOB = mkglob("run-pass")
COMPILE_FAIL_GLOB = mkglob("compile-fail")
RUN_FAIL_GLOB = mkglob("run-fail")

EXIT_PASS = 0
EXIT_COMPILE_FAIL = 66
# TODO: change this, see comment in main
EXIT_RUN_FAIL = 1

passing = True


def fail():
    global passing
    passing = False


def ppath(path):
    return str(path).replace(BASE_DIR, ".")


def process(stream, file):
    stream = stream.decode()
    output = stream.replace(BASE_DIR, "$DD")
    prity_file = ppath(file)
    if args.bless:
        try:
            with open(file, "r") as f:
                if f.read() == output:
                    return
        except:
            # Ignore errors where the file doesnt exits
            pass

        with open(file, "w") as f:
            f.write(output)
        print(f"BLESSED {prity_file}")
    else:
        with open(file, "r") as f:
            got = f.read()
            if output == got:
                print(f"PASSED {prity_file}")
            else:
                # TODO: diff
                print(f"FAILED {prity_file}")
                print("--- expected ---")
                print(got)
                print("--- got ---")
                print(output)
                print("--- ---\n")


def run_pass(path):
    output = subprocess.run([SKATE_BINARY, path], capture_output=True, env=global_env)
    if output.returncode != 0:
        # TODO: Dry
        print(
            f"Running {ppath(SKATE_BINARY)} {ppath(path)} returned code {output.returncode}"
        )
        print("--- stderr ---")
        print(output.stderr.decode())
        print("--------------")
        print("")
        print("--- stdout ---")
        print(output.stdout.decode())
        print("--------------")
        fail()
    else:
        stdout_file = pathlib.Path(path).with_suffix(".stdout")
        process(output.stdout, stdout_file)


def compile_fail(path, errcode):
    output = subprocess.run(
        [SKATE_BINARY, path], capture_output=True, env={"NO_COLOR": "1"} | global_env
    )
    if output.returncode != errcode:
        print(
            f"Running {ppath(SKATE_BINARY)} {ppath(path)} returned code {output.returncode} (expected {errcode})"
        )
        print("--- stderr ---")
        print(output.stderr.decode())
        print("--------------")
        print("")
        print("--- stdout ---")
        print(output.stdout.decode())
        print("--------------")
        fail()
    else:
        stderr_file = pathlib.Path(path).with_suffix(".stderr")
        process(output.stderr, stderr_file)


for i in glob.glob(RUN_PASS_GLOB, recursive=True):
    run_pass(i)

for i in glob.glob(COMPILE_FAIL_GLOB, recursive=True):
    compile_fail(i, EXIT_COMPILE_FAIL)

for i in glob.glob(RUN_FAIL_GLOB, recursive=True):
    compile_fail(i, EXIT_RUN_FAIL)

if not passing:
    exit(1)

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
args = parser.parse_args()

BASE_DIR = path.realpath(path.dirname(__file__))
os.chdir(BASE_DIR)
if not args.skip_build:
    os.system("cargo build")

# TODO: Allow release mode
SKATE_BINARY = path.join(BASE_DIR, "target", "debug", "skate")

TEST_DIR = path.join(BASE_DIR, "tests")
RUN_PASS = path.join(TEST_DIR, "run-pass")
COMPILE_FAIL = path.join(TEST_DIR, "compile-fail")
SK_GLOB = path.join("**", "*.sk")
RUN_PASS_GLOB = path.join(RUN_PASS, SK_GLOB)
COMPILE_FAIL_GLOB = path.join(COMPILE_FAIL, SK_GLOB)

passing = True


def process(stream, file):
    stream = stream.decode()
    output = stream.replace(BASE_DIR, "$DD")
    prity_file = str(file).replace(BASE_DIR, ".")
    if args.bless:
        with open(file, "w") as f:
            f.write(output)
        print(f"BLESSED {prity_file}")
    else:
        with open(file, "r") as f:
            assert output == f.read()
            print(f"PASSED {prity_file}")


def run_pass(path):
    output = subprocess.run([SKATE_BINARY, path], capture_output=True)
    if output.returncode != 0:
        print(f"Running {SKATE_BINARY} {path} returned code {output.returncode}")
        print("--- stderr ---")
        print(output.stderr.decode())
        print("--------------")
        global passing
        passing = False

    stdout_file = pathlib.Path(path).with_suffix(".stdout")
    process(output.stdout, stdout_file)


def compile_fail(path):
    output = subprocess.run(
        [SKATE_BINARY, path], capture_output=True, env={"NO_COLOR": "1"}
    )
    stderr_file = pathlib.Path(path).with_suffix(".stderr")
    # TODO: Sort this out. decide something like exit(1) = program failed.
    # exit(2) = internal interpriter error
    assert output.returncode != 0
    process(output.stderr, stderr_file)


for i in glob.glob(RUN_PASS_GLOB, recursive=True):
    run_pass(i)

for i in glob.glob(COMPILE_FAIL_GLOB, recursive=True):
    compile_fail(i)

if not passing:
    exit(1)

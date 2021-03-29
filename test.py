#!/usr/bin/env python3
import glob
from os import path
import os
import subprocess
import pathlib

BASE_DIR = path.dirname(__file__)

os.chdir(BASE_DIR)
os.system("cargo build")

# TODO: Allow release mode
SKATE_BINARY = path.join(BASE_DIR, "target", "debug", "skate")

TEST_DIR = path.join(BASE_DIR, "tests")
RUN_PASS = path.join(TEST_DIR, "run-pass")
COMPILE_FAIL = path.join(TEST_DIR, "compile-fail")
SK_GLOB = path.join("**", "*.sk")
RUN_PASS_GLOB = path.join(RUN_PASS, SK_GLOB)
COMPILE_FAIL_GLOB = path.join(COMPILE_FAIL, SK_GLOB)


def run_pass(path):
    output = subprocess.run([SKATE_BINARY, path], check=True, capture_output=True)
    stdout_file = pathlib.Path(path).with_suffix(".stdout")
    with open(stdout_file, "w") as f:
        f.write(output.stdout.decode())


def compile_fail(path):
    output = subprocess.run([SKATE_BINARY, path], check=False, capture_output=True)
    stderr_file = pathlib.Path(path).with_suffix(".stderr")
    # TODO: Sort this out. decide something like exit(1) = program failed.
    # exit(2) = internal interpriter error
    assert output.returncode != 0
    with open(stderr_file, "w") as f:
        f.write(output.stderr.decode())


for i in glob.glob(RUN_PASS_GLOB, recursive=True):
    run_pass(i)

for i in glob.glob(COMPILE_FAIL_GLOB, recursive=True):
    compile_fail(i)
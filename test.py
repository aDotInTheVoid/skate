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
SK_GLOB = path.join("**", "*.sk")
RUN_PASS_GLOB = path.join(RUN_PASS, SK_GLOB)


def run_pass(path):
    output = subprocess.run([SKATE_BINARY, path], check=True, capture_output=True)
    stdout_file = pathlib.Path(path).with_suffix(".stdout")
    with open(stdout_file, "w") as f:
        f.write(output.stdout.decode())


for i in glob.glob(path.join(RUN_PASS_GLOB), recursive=True):
    run_pass(i)

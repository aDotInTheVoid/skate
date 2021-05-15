#!/bin/bash
set -euxo pipefail

just wasm
mdbook build

# mkdir public

mv book public
mv playground/static public/playground
rm public/playground/.gitignore
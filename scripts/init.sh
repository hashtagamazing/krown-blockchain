#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e

echo "*** Initializing WASM build environment"

specific_nightly_version="nightly-2023-10-04"

if [ -z $CI_PROJECT_NAME ] ; then
   rustup update stable
   rustup toolchain install $specific_nightly_version
fi

rustup target add wasm32-unknown-unknown --toolchain $specific_nightly_version

#!/usr/bin/env sh

clear

if [ -z "${EXAMPLE}" ];
then
	EXAMPLE="platformer"
fi


echo "import init from \"./${EXAMPLE}\";
init();" > "generated/load.js"

RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo build --target "wasm32-unknown-unknown" --example=$EXAMPLE --profile "release-wasm"
RUST_LOG="warn" wasm-bindgen --out-dir "generated" --web "target/wasm32-unknown-unknown/release-wasm/examples/${EXAMPLE}.wasm"
wasm-opt -Oz -o "generated/${EXAMPLE}_bg.wasm" "generated/${EXAMPLE}_bg.wasm"

# run:
# > ./wasm-build.sh
# > miniserve generated
# note:
# required:
# - community/miniserve + community/binaryen packages
# - nightly firefox with webgpu enabled
# note:
# do:
# > EXAMPLE='example' ./wasm-build.sh
# to pick an example
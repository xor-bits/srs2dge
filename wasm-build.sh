#!/usr/bin/env sh

# run:
# > ./wasm-build.sh
# > miniserve generated
# > xdg-open "localhost:8080/index.html"
# note:
# required:
# - community/miniserve + community/binaryen packages
# - nightly firefox with webgpu enabled
# note:
# do:
# > EXAMPLE='example' ./wasm-build.sh
# to pick an example

clear

if [ -z "${EXAMPLE}" ];
then
	EXAMPLE="platformer"
fi
if [ -z "${OUTDIR}" ];
then
	OUTDIR="generated"
fi

mkdir -p "${OUTDIR}/"

cp "generated/index.html" "${OUTDIR}/"
cp "generated/.gitignore" "${OUTDIR}/"
echo "import init from \"./${EXAMPLE}.js\";
init();" > "${OUTDIR}/load.js"

RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo build --target "wasm32-unknown-unknown" --example=$EXAMPLE --profile "release-wasm"
RUST_LOG="warn" wasm-bindgen --out-dir "${OUTDIR}" --web "target/wasm32-unknown-unknown/release-wasm/examples/${EXAMPLE}.wasm"
wasm-opt -Oz -o "${OUTDIR}/${EXAMPLE}_bg.wasm" "${OUTDIR}/${EXAMPLE}_bg.wasm"
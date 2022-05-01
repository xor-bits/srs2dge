clear
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown --example=main --profile release-wasm
RUST_LOG=warn wasm-bindgen --out-dir generated --web target/wasm32-unknown-unknown/release-wasm/examples/main.wasm
wasm-opt -Oz -o generated/main_bg.wasm generated/main_bg.wasm

# run:
# > ./wasm-build.sh
# > miniserve generated
# note:
# required:
# - community/miniserve + community/binaryen packages
# - nightly firefox with webgpu enabled
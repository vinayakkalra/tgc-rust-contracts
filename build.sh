#!/usr/bin/env bash
set -e

package="$1"
root="$(dirname "$0")/."
did_file="/tmp/a.did"

cargo build --manifest-path="$root/Cargo.toml" \
    --target wasm32-unknown-unknown \
    --release \
    --package "$package"

candid-extractor "$root/target/wasm32-unknown-unknown/release/$package.wasm" 2>/dev/null > $did_file || true

ic-wasm "$root/target/wasm32-unknown-unknown/release/$package.wasm" \
    -o "$root/target/wasm32-unknown-unknown/release/$package.wasm" \
    metadata candid:service -v public -f $did_file

ic-wasm "$root/target/wasm32-unknown-unknown/release/$package.wasm" \
    -o "$root/target/wasm32-unknown-unknown/release/$package-opt.wasm" \
    shrink

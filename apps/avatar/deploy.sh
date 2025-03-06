#!/bin/zsh

set -e

rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli
wasm-bindgen --out-dir ./wasm --target web target/wasm32-unknown-unknown/release/avatar.wasm
cp -r assets ./wasm/
AWS_PROFILE=yyuunn922 aws s3 sync wasm/ s3://choiyunseok-mono-project/wasm/avatar --delete
AWS_PROFILE=yyuunn922 aws cloudfront create-invalidation --distribution-id E1A34FQNC9LRPS --paths '/*'

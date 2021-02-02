#!/bin/bash
cd react-front-end
pkill -x npm
npm ci
npm run build
cd ../
cd actix
sh sshTunnel.sh
nolimit cargo build --release
pkill -x actix
cargo run --release

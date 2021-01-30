#!/bin/bash
cd react-front-end
npm ci
npm run build
cd ../
cd actix
sh sshTunnel.sh
nolimit cargo build --release
cargo run --release

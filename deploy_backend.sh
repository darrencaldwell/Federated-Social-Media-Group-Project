#!/bin/bash
cd backend
sh sshTunnel.sh
nolimit cargo build --release
pkill -x actix
cargo run --release >> log.log 2>&1

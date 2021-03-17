#!/bin/bash
cd backend
sh sshTunnel.sh
pkill -x actix
cargo run --release >> log.log 2>&1

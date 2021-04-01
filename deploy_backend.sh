#!/bin/bash
cd backend
pkill -x actix
ps -lef | grep ssh | grep 21463 | awk "{print \$4}" | xargs kill
sh sshTunnel.sh
echo "Starting..."
cargo run --release > log.log 2>&1

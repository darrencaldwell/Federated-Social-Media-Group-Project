#!/bin/bash
cd backend
pid_ssh=$(pgrep -f 'ssh.*-f')
pid_actix=$(pidof actix)
while kill -0 $pid_ssh; do
	sleep 1
done
while kill -0 $pid_actix; do
	sleep 1
done
sh sshTunnel.sh
cargo run --release >> log.log 2>&1

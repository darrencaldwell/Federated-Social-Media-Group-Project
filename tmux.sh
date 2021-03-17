#!/bin/bash
sh deploy_frontend.sh &

cd backend
nolimit cargo build --release
cd ../

session="deploy"
tmux has-session -t $session 2>/dev/null

if [ $? = 0 ]; then
	# kill if exists
	tmux kill-session -t $session
fi
tmux new-session -d -s 'deploy' './deploy_backend.sh; exec $SHELL'

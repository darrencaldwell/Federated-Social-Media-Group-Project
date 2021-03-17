#!/bin/bash
session="deploy"
tmux has-session -t $session 2>/dev/null

if [ $? = 0 ]; then
	# kill if exists
	tmux kill-session -t $session
fi

./deploy_frontend.sh &
tmux new-session -d -s 'deploy' './deploy_backend.sh; exec $SHELL'

#!/bin/bash
session="deploy"
tmux has-session -t $session 2>/dev/null

if [ $? = 0 ]; then
	# kill if exists
	tmux kill-session -t $session
fi

tmux new-session -d -s 'deploy' './deploy.sh; exec $SHELL'

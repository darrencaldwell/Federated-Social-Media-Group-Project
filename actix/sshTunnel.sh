#!/bin/bash
ssh -f "$1"@cs3099user-b5.host.cs.st-andrews.ac.uk -L 3306:localhost:3306 -N

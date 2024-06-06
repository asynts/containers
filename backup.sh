#!/bin/bash
set -e

~/dev/scripts/backup.rb \
    --name "jail" \
    --url "git@github.com:asynts/jail.git"

#!/bin/sh

# Kills the process running on the provided port
#
# clear-port 3000

if [ -n "$1" ]; then
  port_num="$(lsof -ti4TCP:"$1")"
  if [ $? -eq 0 ]; then
    kill "$port_num"
  fi
else
  echo >&2 Usage: clear-port port-number
  exit 1
fi

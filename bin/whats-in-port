#!/bin/sh

# List process running on provided port
#
# whats-in-port 3000
#
# output:
# COMMAND   PID   USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
# ruby    25583   root   11u  IPv4 0xee20607697a79bf7      0t0  TCP *:irdmi (LISTEN)

if [ -n "$1" ]; then
  lsof -ni4TCP:"$1"
else
  echo >&2 Usage: whats-in-port port-number
  exit 1
fi

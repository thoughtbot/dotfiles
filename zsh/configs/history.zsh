#!/usr/bin/env zsh

setopt hist_ignore_all_dups hist_ignore_space inc_append_history share_history

HISTFILE=~/.zhistory
HISTSIZE=8192
SAVEHIST=8192

export ERL_AFLAGS="-kernel shell_history enabled"

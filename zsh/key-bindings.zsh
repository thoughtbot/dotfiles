# vi mode
bindkey -v

# use incremental search
bindkey "^R" history-incremental-search-backward

bindkey "^P" history-search-backward
bindkey "^Y" accept-and-hold
bindkey "^N" insert-last-word
bindkey -s "^T" "^[Isudo ^[A" # "t" for "toughguy"

# Alias Editing
alias ae='vi ~/.zsh/aliases.zsh' #alias edit
alias ar='source ~/.zsh/aliases.zsh'  #alias reload
alias zr='source ~/.zshrc' #reload zsh

# Unix
alias la='ls -la'
alias ln='ln -v'
alias mkdir='mkdir -p'
alias ...='../..'

# PS
alias psa="ps aux"
alias psag="ps aux | ag "

# git
alias g="git"
alias ga="git add"
alias gs='git status -s'
alias gall='git add -A'
alias gb='git branch'
alias gba='git branch -a'
alias gc='git commit'
alias gcm='git commit -m'
alias gcl='git clone'
alias gco='git checkout'
alias gnb='git checkout -b'
alias gr='git rebase'
alias gri='git rebase -i'
alias gra='git rebase --abort'
alias grc='git rebase --continue'
alias gl='git log -10'
alias gf='git fetch'
alias gd='git diff'
alias gcp='git cherry-pick'
alias gcd='cd $(git rev-parse --show-toplevel)'
alias amend='git commit --amend'
alias amendne='git commit --amend --no-edit'
alias standup='git standup'
alias gpr='hub pull-request'

# Bundler
alias b="bundle"
alias be="bundle exec"
alias bi="bundle install"

# Tests and Specs
alias s="rspec"

# Rails
alias c="rails console"

# Open ports
alias open_ports="lsof -i -P | grep -i 'listen'"

# tmux
alias tat='tmux attach -t'
alias tns='tmux new -s'

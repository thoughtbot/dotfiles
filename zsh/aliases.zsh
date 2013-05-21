# Alias Editing
alias ae='vi ~/.zsh/aliases.zsh' #alias edit
alias ar='source ~/.zsh/aliases.zsh'  #alias reload
alias zr='source ~/.zshrc' #reload zsh

# Unix
alias tlf="tail -f"
alias ln='ln -v'
alias mkdir='mkdir -p'
alias ...='../..'
alias l='ls'
alias ll='ls -al'
alias lh='ls -Alh'
alias -g G='| grep'
alias -g M='| less'
alias -g L='| wc -l'
alias -g ONE="| awk '{ print \$1}'"

# PS
alias psa="ps aux"
alias psag="ps aux | ag "
alias psg="ps aux | grep "

# git
alias g="git"
alias gs='git status -s'
alias gst='git stash'
alias gsh='git show'
alias gshow='git show'
alias gi='vi .gitignore'
alias ga='git add'
alias gall='git add -A'
alias gb='git branch'
alias gba='git branch -a'
alias gc='git commit'
alias ga='git commit -a'
alias gcm='git commit -m'
alias gcam='git commit -am'
alias gcl='git clone'
alias gco='git checkout'
alias gnb='git checkout -b'
alias gr='git rebase'
alias gra='git rebase --abort'
alias grc='git rebase --continue'
alias gl='git log -10'
alias gf='git fetch'
alias gd='git diff'
alias gpl='git pull'
alias gplr='git pull --rebase'
alias gps='git push'
alias grs='git reset'
alias grsh='git reset --hard'
alias gcp='git cherry-pick'
alias gcd='cd $(git rev-parse --show-toplevel)'
alias amend='git commit --amend'
alias amendne='git commit --amend --no-edit'
alias standup='git standup'

# Bundler
alias b="bundle"
alias be="bundle exec"
alias bi="bundle install --binstubs=bin/stubs"

# Tests and Specs
alias s="rspec"

# Rubygems
alias gi="gem install"
alias giv="gem install -v"

# Rails
alias migrate="rake db:migrate db:test:prepare"
alias c="rails console"

# Open ports
alias open_ports="lsof -i -P | grep -i 'listen'"

# tmux
alias tat='tmux attach -t'
alias tns='tmux new -s'

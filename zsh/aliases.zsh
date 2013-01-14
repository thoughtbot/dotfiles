# Alias Editing
alias ae='vi ~/.zsh/aliases.zsh' #alias edit
alias ar='source ~/.zsh/aliases.zsh'  #alias reload
alias zr='source ~/.zshrc' #reload zsh

# vimrc editing
alias ve='vi ~/.vimrc'

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
alias psg="ps aux | grep "

# git
alias g="git"
alias gs='git status -s'
alias gst='git stash'
alias gsh='git show'
alias gshow='git show'
alias gi='vi .gitignore'
alias gall='git add -A'
alias gc='git ci'
alias gcm='git ci -m'
alias gcam='git ci -am'
alias gcl='git clone'
alias gco='git co'
alias gcob='git co -b'
alias gr='git rebase'
alias gra='git rebase --abort'
alias grc='git rebase --continue'
alias gl='git l -10'
alias gf='git fetch'
alias gd='git diff'
alias gb='git b'
alias gpl='git pull'
alias gplr='git pull --rebase'
alias gps='git push'
alias grs='git reset'
alias grsh='git reset --hard'
alias gcpk='git cherry-pick'
alias gcd='cd $(git rev-parse --show-toplevel)'

# Bundler
alias b="bundle"
alias be="bundle exec"
alias bi="bundle install --binstubs"

# Tests and Specs
alias s="rspec"

# Rubygems
alias gi="gem install"
alias giv="gem install -v"

# Rails
alias migrate="rake db:migrate db:test:prepare"

# Heroku staging
alias staging='heroku run console --remote staging'
alias staging-name='echo `basename $PWD`-staging'
alias staging-process='watch heroku ps --remote staging'
alias staging-releases='heroku releases --remote staging'
alias staging-tail='heroku logs --tail --remote staging'

# Heroku production
alias production='heroku run console --remote production'
alias production-name='echo `basename $PWD`-production'
alias production-process='watch heroku ps --remote production'
alias production-releases='heroku releases --remote production'
alias production-tail='heroku logs --tail --remote production'

# Heroku databases
alias db-pull-staging='heroku db:pull --remote staging --confirm `staging-name`'
alias db-pull-production='heroku db:pull --remote production --confirm `production-name`'
alias db-copy-production-to-staging='heroku pgbackups:restore DATABASE `heroku pgbackups:url --app production-name` --app `staging-name` --confirm `staging-name`'
alias db-backup-production='heroku pgbackups:capture --remote production'

# Network
alias whats-my-ip="curl -s checkip.dyndns.org | grep -Eo '[0-9\.]+'"

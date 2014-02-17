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
alias e="$EDITOR"
alias v="$VISUAL"
alias cl="clear"

# git
alias gci="git pull --rebase && rake && git push"
alias gc="git commit"
alias gst="git status"
alias gp="git push"
alias ga="git add"
alias gb="git branch"
alias gco="git checkout"
alias gcob="git checkout -b"

# Bundler
alias b="bundle"

# Tests and Specs
alias t="ruby -I test"
alias cuc="bundle exec cucumber"

# Rubygems
alias gi="gem install"
alias giv="gem install -v"

# Rails
alias migrate="rake db:migrate db:rollback && rake db:migrate db:test:prepare"
alias m="migrate"
alias rk="rake"
alias s="rspec"
alias z="zeus"

# Include custom aliases
[[ -f ~/.aliases.local ]] && source ~/.aliases.local

# Browser: alias 'open' to 'launchy'
if [ `uname -s` = "Linux" ]; then
  if which launchy &>/dev/null; then
    alias open='launchy'
  fi
fi

# Pipe my public key to clipboard
case `uname -s` in
  Darwin )
    alias pubkey="more ~/.ssh/id_rsa.pub | pbcopy | echo '=> Public key copied to pasteboard.'" ;;
  Linux )
    if which xclip &>/dev/null; then
      alias pubkey="xclip -sel clip < ~/.ssh/id_rsa.pub && echo '=> Public key copied to pasteboard.'"
    else
      echo "xclip is not installed. Please install it and try again."
    fi ;;
esac

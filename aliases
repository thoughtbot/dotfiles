# Unix
alias ll="ls -al"
alias ln="ln -v"
alias mkdir="mkdir -p"
alias e="$EDITOR"
alias v="$VISUAL"

# Bundler
alias b="bundle"

# Rails
alias migrate="bin/rails db:migrate db:rollback && bin/rails db:migrate db:test:prepare"
alias s="rspec"

# Pretty print the path
alias path='echo $PATH | tr -s ":" "\n"'

# Include custom aliases
if [[ -f ~/.aliases.local ]]; then
  source ~/.aliases.local
fi

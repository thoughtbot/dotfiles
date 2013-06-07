path=($HOME/.dotfiles/bin /usr/local/bin /usr/local/share/npm/bin $path)
if which rbenv > /dev/null; then eval "$(rbenv init -)"; fi
path=(.git/safe/../../bin .git/safe/../../bin/stubs $path)

path=($HOME/.dotfiles/bin /usr/local/bin /usr/local/share/npm/bin $path)

if [[ -e /usr/local/share/chruby ]]; then
  source /usr/local/share/chruby/chruby.sh
  source /usr/local/share/chruby/auto.sh
elif which rbenv > /dev/null; then
  eval "$(rbenv init - --no-rehash)";
fi

path=(.git/safe/../../bin .git/safe/../../bin/stubs $path)
path=(./git/safe/../../node_modules/.bin $path)

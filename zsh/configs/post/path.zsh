# ensure dotfiles bin directory is loaded first
PATH="$HOME/.bin:/usr/local/sbin:$PATH"

# load ASDF, falling back to rbenv if not available
if [ -d "$HOME/.asdf" ]; then
  . $HOME/.asdf/asdf.sh
elif command -v rbenv >/dev/null; then
  if [ -z $SILENCE_RBENV_DEPRECATION ]; then
    echo "The thoughtbot dotfiles have deprecated the use of rbenv in favor"\
         "of asdf (https://github.com/asdf-vm/asdf) and will be removed on or"\
         "after December 8, 2017.  Migrate to asdf or export"\
         "SILENCE_RBENV_DEPRECATION=1 in your local dotfiles to silence this"\
         "deprecation."
  fi

  eval "$(rbenv init - --no-rehash)"
fi

# mkdir .git/safe in the root of repositories you trust
PATH=".git/safe/../../bin:$PATH"

export -U PATH

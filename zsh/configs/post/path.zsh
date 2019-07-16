# ensure dotfiles bin directory is loaded first
PATH="$HOME/.bin:/usr/local/sbin:$PATH"

# load ASDF, either from home dir or brew install
if [ -f "$HOME/.asdf/asdf.sh" ]; then
  . "$HOME/.asdf/asdf.sh"
elif [ -f "/usr/local/opt/asdf/asdf.sh" ]; then
  . "/usr/local/opt/asdf/asdf.sh"
elif which brew >/dev/null && ASDF_PREFIX=$(brew --prefix asdf); then
  . "$ASDF_PREFIX/asdf.sh"
fi

# mkdir .git/safe in the root of repositories you trust
PATH=".git/safe/../../bin:$PATH"

export -U PATH

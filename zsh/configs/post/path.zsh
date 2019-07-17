# ensure dotfiles bin directory is loaded first
PATH="$HOME/.bin:/usr/local/sbin:$PATH"

# Try loading ASDF from the regular home dir location
if [ -f "$HOME/.asdf/asdf.sh" ]; then
  . "$HOME/.asdf/asdf.sh"
# It's not in the home dir, let's try the default Homebrew location
elif [ -f "/usr/local/opt/asdf/asdf.sh" ]; then
  . "/usr/local/opt/asdf/asdf.sh"
# Not there either! Last try, let's see if Homebrew can tell us where it is
elif which brew >/dev/null &&
  ASDF_PREFIX=$(brew --prefix asdf) &&
  [ -f "$ASDF_PREFIX/asdf.sh" ]; then
  . "$ASDF_PREFIX/asdf.sh"
fi

# mkdir .git/safe in the root of repositories you trust
PATH=".git/safe/../../bin:$PATH"

export -U PATH

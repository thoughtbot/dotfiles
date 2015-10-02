# ensure dotfiles bin directory is loaded first
export PATH="$HOME/.bin:/usr/local/sbin:$PATH"

# load rbenv if available
if which rbenv &>/dev/null ; then
  eval "$(rbenv init - --no-rehash)"
fi

# mkdir .git/safe in the root of repositories you trust
export PATH=".git/safe/../../bin:$PATH"

# Local config
[[ -f ~/.zprofile.local ]] && source ~/.zprofile.local

# tmux loads zsh as a login shell, which may result in duplicate $PATH entries
# from ~/.zprofile. This removes duplicates from $PATH.
typeset -U PATH

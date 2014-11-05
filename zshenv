# use vim as the visual editor
export VISUAL=vim
export EDITOR=$VISUAL

DOTFILES=~/.dotfiles
ZSH=$DOTFILES/zsh

# Xcode specific paths need to be declared first
DEVELOPER_PATH=`xcode-select --print-path`
export DEVELOPER_PATH

DEVELOPER_DIR=$DEVELOPER_PATH
export DEVELOPER_DIR

IPHONEOS_DEVELOPER_PATH=$DEVELOPER_PATH/Platforms/iPhoneOS.platform/Developer
export IPHONEOS_DEVELOPER_PATH

# system path and environment variables need to be loaded before everything else
export PATH=/usr/local/bin:/usr/local/sbin:$DEVELOPER_PATH/usr/bin:$IPHONEOS_DEVELOPER_PATH/usr/bin:$PATH

MANPATH=/usr/local/share/man:/usr/share/man
export MANPATH

CODE=~/Development
export CODE

# ensure dotfiles bin directory is loaded first
export PATH="$HOME/.bin:$PATH"

# load rbenv if available
if which rbenv &>/dev/null ; then
  eval "$(rbenv init - --no-rehash)"
fi

# mkdir .git/safe in the root of repositories you trust
export PATH=".git/safe/../../bin:$PATH"

# Local config
[[ -f ~/.zshenv.local ]] && source ~/.zshenv.local

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

# load our own completion functions
fpath=($ZSH/completion $fpath)

# all of our zsh files
typeset -U config_files
config_files=($DOTFILES/**/*.zsh)

# load the path files
for file in ${(M)config_files:#*/path.zsh}
do
  source $file
done

# load everything but the path and completion files
for file in ${${config_files:#*/path.zsh}:#*/completion.zsh}
do
  source $file
done

# initialize autocomplete here, otherwise functions won't be loaded
autoload -U compinit
compinit

for function in $ZSH/functions/*
do
  source $function
done

# Load every completion after autocomplete loads
for file in ${(M)config_files:#*/completion.zsh}
do
  source $file
done

unset config_files

# history settings
setopt histignoredups
SAVEHIST=4096
HISTSIZE=4096

# awesome cd movements from zshkit
setopt autocd autopushd pushdminus pushdsilent pushdtohome cdablevars
DIRSTACKSIZE=5

# Turn off auto correct command line spelling
unsetopt correct correctall

# Enable extended globbing
setopt extendedglob

# Allow [ or ] whereever you want
unsetopt nomatch

# vi mode
bindkey -v
bindkey "^F" vi-cmd-mode
bindkey jj vi-cmd-mode

# handy keybindings
bindkey "^A" beginning-of-line
bindkey "^E" end-of-line
bindkey "^R" history-incremental-search-backward
bindkey "^P" history-search-backward
bindkey "^Y" accept-and-hold
bindkey "^N" insert-last-word
bindkey -s "^T" "^[Isudo ^[A" # "t" for "toughguy"

# use vim as the visual editor
export VISUAL=vim
export EDITOR=$VISUAL

# look for ey config in project dirs
export EYRC=./.eyrc

# mkdir .git/safe in the root of repositories you trust
export PATH=".git/safe/../../bin:$PATH"

# aliases
[[ -f ~/.aliases ]] && source ~/.aliases

# Local config
[[ -f ~/.zshrc.local ]] && source ~/.zshrc.local

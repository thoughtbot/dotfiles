# Source oh-my-zsh if it is installed.
if [[ -d $HOME/.oh-my-zsh ]]; then
  # Path to your oh-my-zsh configuration.
  ZSH=$HOME/.oh-my-zsh

  # Set name of the theme to load.
  ZSH_THEME="skwp"

  plugins=(zsh-syntax-highlighting vi-mode history-substring-search)

  source $ZSH/oh-my-zsh.sh
fi

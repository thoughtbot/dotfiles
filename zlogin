# oh-my-zsh config
[[ -f ~/.ohmyzshrc ]] && source ~/.ohmyzshrc

# makes color constants available
autoload -U colors
colors

# enable colored output from ls, etc
export CLICOLOR=1

# expand functions in the prompt
setopt promptsubst

# load thoughtbot/dotfiles scripts
export PATH="$HOME/.bin:$PATH"

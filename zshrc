# ==============================================================================
# ZSH Configuration
# ==============================================================================
# This file is the main zsh configuration entry point.
# It loads modular configs from ~/.zsh/configs/ in order: pre -> main -> post
# ==============================================================================

# ------------------------------------------------------------------------------
# Custom Functions
# ------------------------------------------------------------------------------
# Load all custom executable functions from ~/.zsh/functions/
for function in ~/.zsh/functions/*; do
  source $function
done

# ------------------------------------------------------------------------------
# Modular Config Loading
# ------------------------------------------------------------------------------
# Load config files from ~/.zsh/configs/pre, ~/.zsh/configs, and ~/.zsh/configs/post
# in that order. This allows for proper dependency management.
_load_settings() {
  _dir="$1"
  if [ -d "$_dir" ]; then
    if [ -d "$_dir/pre" ]; then
      for config in "$_dir"/pre/**/*~*.zwc(N-.); do
        . $config
      done
    fi

    for config in "$_dir"/**/*(N-.); do
      case "$config" in
        "$_dir"/(pre|post)/*|*.zwc)
          :
          ;;
        *)
          . $config
          ;;
      esac
    done

    if [ -d "$_dir/post" ]; then
      for config in "$_dir"/post/**/*~*.zwc(N-.); do
        . $config
      done
    fi
  fi
}
_load_settings "$HOME/.zsh/configs"

# ------------------------------------------------------------------------------
# Local Overrides
# ------------------------------------------------------------------------------
# Machine-specific config that shouldn't be in version control
[[ -f ~/.zshrc.local ]] && source ~/.zshrc.local

# Shell aliases
[[ -f ~/.aliases ]] && source ~/.aliases

# ------------------------------------------------------------------------------
# FZF - Fuzzy Finder
# ------------------------------------------------------------------------------
[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh

# ------------------------------------------------------------------------------
# Tab Completions (tabtab)
# ------------------------------------------------------------------------------
[[ -f ~/.config/tabtab/__tabtab.zsh ]] && . ~/.config/tabtab/__tabtab.zsh || true

# ------------------------------------------------------------------------------
# Zsh Plugins
# ------------------------------------------------------------------------------
# z - Jump to frequently used directories
source ~/.zsh/zsh-z/zsh-z.plugin.zsh

# zsh-autosuggestions - Fish-like autosuggestions
source ~/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh

# zsh-syntax-highlighting - Syntax highlighting for commands (load last)
source ~/.zsh/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh

# ------------------------------------------------------------------------------
# ASDF Version Manager - Language Runtimes
# ------------------------------------------------------------------------------
# Java - Set JAVA_HOME automatically based on asdf version
source ~/.asdf/plugins/java/set-java-home.zsh

# Golang - Set Go environment variables
. ~/.asdf/plugins/golang/set-env.zsh

# ------------------------------------------------------------------------------
# Android SDK
# ------------------------------------------------------------------------------
export ANDROID_HOME=$HOME/Library/Android/sdk
export PATH=$PATH:$ANDROID_HOME/emulator
export PATH=$PATH:$ANDROID_HOME/tools
export PATH=$PATH:$ANDROID_HOME/tools/bin
export PATH=$PATH:$ANDROID_HOME/platform-tools

# ------------------------------------------------------------------------------
# Bun - JavaScript Runtime
# ------------------------------------------------------------------------------
# Bun completions
[ -s "$HOME/.bun/_bun" ] && source "$HOME/.bun/_bun"

# Bun path
export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"
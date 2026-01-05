# Enable Powerlevel10k instant prompt. Should stay close to the top of ~/.zshrc.
# Initialization code that may require console input (password prompts, [y/n]
# confirmations, etc.) must go above this block; everything else may go below.
if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

export LANG=en_US.UTF-8

export PATH="${HOME}/.local/bin:$PATH"

eval "$(~/.local/bin/mise activate zsh)"

test -e "${HOME}/.iterm2_shell_integration.zsh" && source "${HOME}/.iterm2_shell_integration.zsh"

export MATLAB_RUNTIME="/Applications/MATLAB/MATLAB_Runtime/v97"
export DYLD_LIBRARY_PATH="${MATLAB_RUNTIME}/runtime/maci64:${MATLAB_RUNTIME}/sys/os/maci64:${MATLAB_RUNTIME}/bin/maci64"

# Added by LM Studio CLI tool (lms)
export PATH="$PATH:${HOME}/.cache/lm-studio/bin"
export PATH="${HOME}/.mint/bin:$PATH"
export PATH="${HOME}/.bun/bin:$PATH"

export PATH="${HOME}/Develop/google-cloud-sdk/bin:$PATH"

### Alibaba cloud mirror for homebrew
# export HOMEBREW_API_DOMAIN=https://mirrors.aliyun.com/homebrew-bottles/api
# export HOMEBREW_BOTTLE_DOMAIN=https://mirrors.aliyun.com/homebrew-bottles
# export HOMEBREW_BREW_GIT_REMOTE=https://mirrors.aliyun.com/homebrew/brew.git

### Added by Zinit's installer
if [[ ! -f $HOME/.local/share/zinit/zinit.git/zinit.zsh ]]; then
    print -P "%F{33} %F{220}Installing %F{33}ZDHARMA-CONTINUUM%F{220} Initiative Plugin Manager (%F{33}zdharma-continuum/zinit%F{220})…%f"
    command mkdir -p "$HOME/.local/share/zinit" && command chmod g-rwX "$HOME/.local/share/zinit"
    command git clone https://github.com/zdharma-continuum/zinit "$HOME/.local/share/zinit/zinit.git" && \
        print -P "%F{33} %F{34}Installation successful.%f%b" || \
        print -P "%F{160} The clone has failed.%f%b"
fi

source "$HOME/.local/share/zinit/zinit.git/zinit.zsh"
autoload -Uz _zinit
(( ${+_comps} )) && _comps[zinit]=_zinit

# Load a few important annexes, without Turbo
# (this is currently required for annexes)
zinit light-mode for \
    zdharma-continuum/zinit-annex-as-monitor \
    zdharma-continuum/zinit-annex-bin-gem-node \
    zdharma-continuum/zinit-annex-patch-dl \
    zdharma-continuum/zinit-annex-rust

### End of Zinit's installer chunk
#
# Load powerlevel10k theme
zinit ice depth"1" # git clone depth
zinit light romkatv/powerlevel10k

# Binary release in archive, from GitHub-releases page.
# After automatic unpacking it provides program "fzf".
zi ice from"gh-r" as"program"
zi light junegunn/fzf

# Load completion-related plugins with wait"0"
zinit lucid wait"0" for \
  lincheney/fzf-tab-completion \
  alexiszamanidis/zsh-git-fzf \
  laggardkernel/git-ignore

zicompinit
zinit cdreplay -q

# Set up fzf key bindings (Ctrl-T/Ctrl-R/Alt-C) and fuzzy completion
command -v fzf >/dev/null 2>&1 && source <(fzf --zsh)

zinit lucid wait"1" for \
  zsh-users/zsh-syntax-highlighting

# The next line updates PATH for the Google Cloud SDK.
if [ -f '${HOME}/Develop/google-cloud-sdk/path.zsh.inc' ]; then . '/Users/jessearmand/Develop/google-cloud-sdk/path.zsh.inc'; fi

# The next line enables shell command completion for gcloud.
if [ -f '${HOME}/Develop/google-cloud-sdk/completion.zsh.inc' ]; then . '/Users/jessearmand/Develop/google-cloud-sdk/completion.zsh.inc'; fi

function gi() { curl -sLw "\n" https://www.toptal.com/developers/gitignore/api/$@ ;}

eval "$(uv generate-shell-completion zsh)"
eval "$(uvx --generate-shell-completion zsh)"

eval "$(/opt/homebrew/bin/brew shellenv)"

compdef _gnu_generic zed

# bun completions
[ -s "/opt/homebrew/share/zsh/site-functions/_bun" ] && source "/opt/homebrew/share/zsh/site-functions/_bun"

# To customize prompt, run `p10k configure` or edit ~/.p10k.zsh.
[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh


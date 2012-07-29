# Simple theme with RVM prompt
function preexec() {
    typeset -gi CALCTIME=1
    typeset -gi CMDSTARTTIME=SECONDS
}
function precmd() {
    if (( CALCTIME )) ; then
        typeset -gi ETIME=SECONDS-CMDSTARTTIME
    fi
    typeset -gi CALCTIME=0
}
function safe-rvm-prompt() {
  if [[ -d ~/.rvm/ ]]; then
    rvm-prompt
  fi
}

PROMPT='%{$fg[blue]%}%~%{$fg_bold[yellow]%}$(git_prompt_info)%{$reset_color%}%{$fg[blue]%}➤ %{$reset_color%}'
RPROMPT='%{$fg[blue]%}$(safe-rvm-prompt i v p g)  [${ETIME}s] %{$reset_color%}'

ZSH_THEME_GIT_PROMPT_PREFIX=" "
ZSH_THEME_GIT_PROMPT_SUFFIX=" "
ZSH_THEME_GIT_PROMPT_DIRTY=" ✗"
ZSH_THEME_GIT_PROMPT_CLEAN=" ✔"

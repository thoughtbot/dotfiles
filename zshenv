local _old_path="$PATH"

# Local config
[[ -f ~/dotfiles-local/.zshenv.local ]] && source ~/dotfiles-local/.zshenv.local

if [[ $PATH != $_old_path ]]; then
  # `colors` isn't initialized yet, so define a few manually
  typeset -AHg fg fg_bold
  if [ -t 2 ]; then
    fg[red]=$'\e[31m'
    fg_bold[white]=$'\e[1;37m'
    reset_color=$'\e[m'
  else
    fg[red]=""
    fg_bold[white]=""
    reset_color=""
  fi

  cat <<MSG >&2
${fg[red]}Warning:${reset_color} your \`~/dotfiles-local/zshenv.local' configuration seems to edit PATH entries.
Please move that configuration to \`~/dotfiles-local/zshrc.local' like so:
  ${fg_bold[white]}cat ~/dotfiles-local/zshenv.local >> ~/dotfiles-local/zshrc.local && rm ~/dotfiles-local/zshenv.local${reset_color}

(called from ${(%):-%N:%i})

MSG
fi

unset _old_path

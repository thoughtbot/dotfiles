local _old_path="$PATH"

# Local config
[[ -f ~/.zshenv.local ]] && source ~/.zshenv.local

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
${fg[red]}Warning:${reset_color} your \`~/.zshenv.local' configuration seems to edit PATH entries.
Please move that configuration to \`.zshrc.local' like so:
  ${fg_bold[white]}cat ~/.zshenv.local >> ~/.zshrc.local && rm ~/.zshenv.local${reset_color}

(called from ${(%):-%N:%i})

MSG
fi

unset _old_path

# load proprietary scripts into path
if [ -d "$HOME/.proprietary" ]; then
  # add all sudirectories containing a directory named 'bin' to PATH
  # EXAMPLE: '~/.proprietary/derp/bin' and '~/.proprietary/bin' would be added,
  #          but '~/.proprietary/foo/bar/bin' would not.
  PATH="$(find -L $HOME/.proprietary -maxdepth 2 -mindepth 1 -name 'bin' -type d | tr '\n' ':')$PATH"
fi

# load rbenv if available
if command -v rbenv &>/dev/null ; then
  eval "$(rbenv init - --no-rehash)"
fi

# mkdir .git/safe in the root of repositories you trust
PATH=".git/safe/../../bin:$PATH"

export PATH

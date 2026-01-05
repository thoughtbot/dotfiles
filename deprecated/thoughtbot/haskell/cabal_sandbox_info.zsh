function cabal_sandbox_info() {
  cabal_files=(*.cabal(N))
  if [ $#cabal_files -gt 0 ]; then
    if [ -f cabal.sandbox.config ]; then
      echo "%{$fg[green]%}sandboxed%{$reset_color%}"
    else
      echo "%{$fg[red]%}not sandboxed%{$reset_color%}"
    fi
  fi
}

RPROMPT="\$(cabal_sandbox_info) $RPROMPT"

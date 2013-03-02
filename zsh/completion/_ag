#compdef ag

if (( CURRENT == 2 )); then
  if [[ -a tmp/tags ]]; then
    compadd $(cut -f 1 tmp/tags | uniq)
  fi
else;
  _files
fi

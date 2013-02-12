rake() {
  if [ -S .zeus.sock ]; then
    zeus rake "$@"
  else
    command rake "$@"
  fi
}

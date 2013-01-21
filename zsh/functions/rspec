rspec() {
  if [ -S .zeus.sock ]; then
    zeus rspec "$@"
  else
    command rspec "$@"
  fi
}

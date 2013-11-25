function wordswith() {
  grep "${1}" /usr/share/dict/words | less
}

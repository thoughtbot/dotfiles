# Speed up git completion
# http://talkings.org/post/5236392664/zsh-and-slow-git-completion
__git_files () {
  _wanted files expl 'local files' _files
}

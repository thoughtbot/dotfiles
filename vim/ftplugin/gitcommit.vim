" Automatically wrap at 72 characters and spell check commit messages
autocmd BufNewFile,BufRead PULLREQ_EDITMSG set syntax=gitcommit
setlocal textwidth=72
setlocal spell

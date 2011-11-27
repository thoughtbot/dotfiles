function! UpdateCtags()
  if filereadable("tmp/tags") && strpart(expand("%"), 0, len(getcwd())) == expand("%")
    execute "silent! !ctags -f tmp/tags -a " . expand("%:p")
  endif
endfunction

augroup ctags
  autocmd!
  autocmd BufWritePost * call UpdateCtags()
augroup END

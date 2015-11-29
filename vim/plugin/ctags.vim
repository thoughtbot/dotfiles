" Exclude Javascript files in :Rtags via rails.vim due to warnings when parsing
let g:Tlist_Ctags_Cmd="ctags --exclude='*.js'"

" Index ctags from any project, including those outside Rails
function! ReindexCtags()
  let l:ctags_hook = '$(git rev-parse --show-toplevel)/.git/hooks/ctags'

  if exists(l:ctags_hook)
    exec '!'. l:ctags_hook
  else
    exec "!ctags -R ."
  endif
endfunction

nmap <Leader>ct :call ReindexCtags()<CR>

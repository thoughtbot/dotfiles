" Duplicate a selection
" Visual mode: D
vmap D y'>p

" Disable shift+k
map K <Nop>

" Get off my lawn
nnoremap <Left> :echoe "Use h"<CR>
nnoremap <Right> :echoe "Use l"<CR>
nnoremap <Up> :echoe "Use k"<CR>
nnoremap <Down> :echoe "Use j"<CR>

" j and k move through wrapped lines
nmap j gj
nmap k gk

command! Q q " Bind :Q to :q

" Edit another file in the same directory as the current file
" uses expression to extract path from current file's path
map <Leader>e :e <C-R>=expand("%:p:h") . '/'<CR>
map <Leader>s :split <C-R>=expand("%:p:h") . '/'<CR>
map <Leader>v :vsplit <C-R>=expand("%:p:h") . '/'<CR>
map <Leader>te :tabe <C-R>=expand("%:p:h") . '/'<CR>

" Rename current file (thanks Gary Bernhardt)
function! RenameFile()
  let old_name = expand('%')
  let new_name = input('New file name: ', expand('%'), 'file')
  if new_name != '' && new_name != old_name
    exec ':saveas ' . new_name
    exec ':silent !rm ' . old_name
    redraw!
  endif
endfunction
map <leader>n :call RenameFile()<cr>

" Reload snippets
map <leader>rr :call ReloadAllSnippets()<cr>

" Reindent file and return to current line
map <leader>i mmgg=G`m<cr>

nmap <leader>9 :%s/:\([^ ]*\)\(\s*\)=>/\1:/g<cr>

" Quicker window movement
noremap <C-j> <C-w>j
noremap <C-k> <C-w>k
noremap <C-h> <C-w>h
noremap <C-l> <C-w>l

" Leaders
map <Leader>bi :!bundle install<cr>
map <Leader>co ggVG"*y

" Git
map <Leader>gs :Gstatus<CR>

" Duplicate a selection
" Visual mode: D
vmap D y'>p

" Disabling
"""""""""""

" Disable shift+k
map K <Nop>

" Bind :W to :w
command! W w

" Bind :Q to :q
command! Q q

" Movement
""""""""""

" Get off my lawn
nnoremap <Left> :echoe "Use h"<CR>
nnoremap <Right> :echoe "Use l"<CR>
nnoremap <Up> :echoe "Use k"<CR>
nnoremap <Down> :echoe "Use j"<CR>

" j and k move through wrapped lines
nmap j gj
nmap k gk

" File Handling
"""""""""""""""

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

" Formatting
""""""""""""

" Reindent file and return to current line
map <leader>i mmgg=G`m<cr>
" Rehash with 1.9 style hash syntax
nmap <leader>rh :%s/:\([^ ]*\)\(\s*\)=>/\1:/g<cr>

" Other
"""""""

map <Leader>gs :Gstatus<CR>
map <Leader>bi :!bundle install<cr>

" run the current file in ruby
map <Leader>r :w\|:!ruby %<cr>

" delete comments
nmap <leader>c :%s/^\s*#.*$//g<CR>:%s/\(\n\)\n\+/\1/g<CR>:nohl<CR>gg

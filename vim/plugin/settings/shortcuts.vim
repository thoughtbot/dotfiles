" Duplicate a selection
" Visual mode: D
vmap D y'>p

" Disable shift+k
map K <Nop>

" Because escape is too far away
imap jj <Esc>

" Get off my lawn
nnoremap <Left> :echoe "Use h"<CR>
nnoremap <Right> :echoe "Use l"<CR>
nnoremap <Up> :echoe "Use k"<CR>
nnoremap <Down> :echoe "Use j"<CR>

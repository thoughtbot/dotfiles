let g:ctrlp_user_command = 'ag %s -l --nocolor -g ""'

" <Leader>d searches the current file's path using ctrlp
nmap <Leader>d :CtrlP <C-R>=expand("%:h")<CR><CR>

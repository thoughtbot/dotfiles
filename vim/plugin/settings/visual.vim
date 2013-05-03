" Softtabs, 2 spaces
set tabstop=2
set shiftwidth=2
set expandtab
set laststatus=2  " Always display the status line

set ruler         " show the cursor position all the time
set showcmd       " display incomplete commands

" Line Numbers
set relativenumber
au InsertEnter * :set nu
au InsertLeave * :set rnu

function! NumberToggle()
  if(&relativenumber == 1)
    set number
  else
    set relativenumber
  endif
endfunc

nnoremap <C-n> :call NumberToggle()<cr>

" Scrolling
set scrolloff=8   "Start scrolling when we're 8 lines away from margins
set sidescrolloff=15
set sidescroll=1

set shiftround    " When at 3 spaces and I hit >>, go to 4, not 5.

" Display extra whitespace
set list listchars=tab:»·,trail:·

au BufRead,BufNewFile *.md,*.txt setlocal textwidth=80

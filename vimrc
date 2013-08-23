set nocompatible  " Use Vim settings, rather then Vi settings, must be first

set history=1000
set autoread      " Reload files changed outside
set visualbell    " No sounds

" Turn of swapfiles
set nobackup
set noswapfile
set nowritebackup

" Search
set incsearch     " do incremental searching
set hlsearch      " Highlight searches by default
set ignorecase    " Ignore case in search

" Hide search highlighting
map <Leader>h :set invhls <CR>

" Backspace
set backspace=indent,eol,start
set guifont=Monaco:h13

let mapleader=","

filetype plugin indent on

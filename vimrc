set nocompatible  " Use Vim settings, rather then Vi settings, must be first
call pathogen#infect()

set history=1000
set laststatus=2  " Always display the status line

" Turn of swapfiles
set nobackup
set noswapfile
set nowritebackup

" Search
set incsearch     " do incremental searching
set hlsearch      " Highlight searches by default

set ruler         " show the cursor position all the time
set showcmd       " display incomplete commands
set autoread      " Reload files changed outside
set visualbell    " No sounds
set ignorecase    " Ignore case in search

" Scrolling
set scrolloff=8   "Start scrolling when we're 8 lines away from margins
set sidescrolloff=15
set sidescroll=1

set shiftround    " When at 3 spaces and I hit >>, go to 4, not 5.
set grepprg=ack   " Use ack instead of grep

" Backspace
set backspace=indent,eol,start

" Line Numbers
set number

set guifont=Monaco:h13

let mapleader=","

augroup vimrcEx
  au!

  " When editing a file, always jump to the last known cursor position.
  " Don't do it when the position is invalid or when inside an event handler
  " (happens when dropping a file on gvim).
  autocmd BufReadPost *
    \ if line("'\"") > 0 && line("'\"") <= line("$") |
    \   exe "normal g`\"" |
    \ endif
augroup END

" Tags
let g:Tlist_Ctags_Cmd="ctags --exclude='*.js'"

filetype plugin indent on

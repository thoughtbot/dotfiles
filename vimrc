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

" Scrolling
set scrolloff=8         "Start scrolling when we're 8 lines away from margins
set sidescrolloff=15
set sidescroll=1

" Line Numbers
set number

set guifont=Monaco:h13

let mapleader = "\\"
imap jj <Esc>

augroup vimrcEx
  au!

  " For all text files set 'textwidth' to 78 characters.
  autocmd FileType text setlocal textwidth=78

  " When editing a file, always jump to the last known cursor position.
  " Don't do it when the position is invalid or when inside an event handler
  " (happens when dropping a file on gvim).
  autocmd BufReadPost *
    \ if line("'\"") > 0 && line("'\"") <= line("$") |
    \   exe "normal g`\"" |
    \ endif
augroup END

" Use Ack instead of Grep when available
if executable("ack")
  set grepprg=ack\ -H\ --nogroup\ --nocolor
endif

" Tags
let g:Tlist_Ctags_Cmd="ctags --exclude='*.js'"

" Get off my lawn
nnoremap <Left> :echoe "Use h"<CR>
nnoremap <Right> :echoe "Use l"<CR>
nnoremap <Up> :echoe "Use k"<CR>
nnoremap <Down> :echoe "Use j"<CR>

filetype plugin on

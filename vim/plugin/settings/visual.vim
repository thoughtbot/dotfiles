" Softtabs, 2 spaces
set tabstop=2
set shiftwidth=2
set expandtab
set laststatus=2  " Always display the status line

set ruler         " show the cursor position all the time
set showcmd       " display incomplete commands

" Line Numbers
set number

" Scrolling
set scrolloff=8   "Start scrolling when we're 8 lines away from margins
set sidescrolloff=15
set sidescroll=1

set shiftround    " When at 3 spaces and I hit >>, go to 4, not 5.

" Open new splits the the right and bottom. More natural than the default
set splitbelow
set splitright

" Display extra whitespace
set list listchars=tab:»·,trail:·

" automatically rebalance windows on vim resize
autocmd VimResized * :wincmd =

au BufRead,BufNewFile *.md,*.txt setlocal textwidth=80

autocmd BufWinEnter,WinEnter * setlocal colorcolumn=80
autocmd BufWinLeave,WinLeave * setlocal colorcolumn=0

au WinLeave * set nocursorcolumn
au WinEnter * set cursorcolumn
set cursorcolumn

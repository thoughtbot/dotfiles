autocmd BufWinEnter,WinEnter * setlocal colorcolumn=100
autocmd BufWinLeave,WinLeave * setlocal colorcolumn=0

au WinLeave * set nocursorcolumn
au WinEnter * set cursorcolumn
set cursorcolumn

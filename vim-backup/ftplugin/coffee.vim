" Language:    CoffeeScript
" Maintainer:  Mick Koch <kchmck@gmail.com>
" URL:         http://github.com/kchmck/vim-coffee-script
" License:     WTFPL

if exists("b:did_ftplugin")
  finish
endif

let b:did_ftplugin = 1

setlocal formatoptions-=t formatoptions+=croql
setlocal comments=:#
setlocal commentstring=#\ %s

setlocal errorformat=Error:\ In\ %f\\,\ %m\ on\ line\ %l,
                    \Error:\ In\ %f\\,\ Parse\ error\ on\ line\ %l:\ %m,
                    \SyntaxError:\ In\ %f\\,\ %m,
                    \%-G%.%#

" Extra options passed to CoffeeMake
if !exists("coffee_make_options")
  let coffee_make_options = ""
endif

" Update `makeprg` for the current filename. This is needed to support filenames
" with spaces and quotes while also supporting generic `make`.
function! s:SetMakePrg()
  let &l:makeprg = "coffee -c " . g:coffee_make_options . ' $* '
  \              . fnameescape(expand('%'))
endfunction

" Set `makeprg` initially.
call s:SetMakePrg()
" Set `makeprg` on rename.
autocmd BufFilePost,BufWritePost,FileWritePost <buffer> call s:SetMakePrg()

" Reset the global variables used by CoffeeCompile.
function! s:CoffeeCompileResetVars()
  " Position in the source buffer
  let s:coffee_compile_src_buf = -1
  let s:coffee_compile_src_pos = []

  " Position in the CoffeeCompile buffer
  let s:coffee_compile_buf = -1
  let s:coffee_compile_win = -1
  let s:coffee_compile_pos = []

  " If CoffeeCompile is watching a buffer
  let s:coffee_compile_watch = 0
endfunction

" Save the cursor position when moving to and from the CoffeeCompile buffer.
function! s:CoffeeCompileSavePos()
  let buf = bufnr('%')
  let pos = getpos('.')

  if buf == s:coffee_compile_buf
    let s:coffee_compile_pos = pos
  else
    let s:coffee_compile_src_buf = buf
    let s:coffee_compile_src_pos = pos
  endif
endfunction

" Restore the cursor to the source buffer.
function! s:CoffeeCompileRestorePos()
  let win = bufwinnr(s:coffee_compile_src_buf)

  if win != -1
    exec win 'wincmd w'
    call setpos('.', s:coffee_compile_src_pos)
  endif
endfunction

" Close the CoffeeCompile buffer and clean things up.
function! s:CoffeeCompileClose()
  silent! autocmd! CoffeeCompileAuPos
  silent! autocmd! CoffeeCompileAuWatch

  call s:CoffeeCompileRestorePos()
  call s:CoffeeCompileResetVars()
endfunction

" Update the CoffeeCompile buffer given some input lines.
function! s:CoffeeCompileUpdate(startline, endline)
  let input = join(getline(a:startline, a:endline), "\n")

  " Coffee doesn't like empty input.
  if !len(input)
    return
  endif

  " Compile input.
  let output = system('coffee -scb 2>&1', input)

  " Move to the CoffeeCompile buffer.
  exec s:coffee_compile_win 'wincmd w'

  " Replace buffer contents with new output and delete the last empty line.
  setlocal modifiable
    exec '% delete _'
    put! =output
    exec '$ delete _'
  setlocal nomodifiable

  " Highlight as JavaScript if there is no compile error.
  if v:shell_error
    setlocal filetype=
  else
    setlocal filetype=javascript
  endif

  " Restore the cursor in the compiled output.
  call setpos('.', s:coffee_compile_pos)
endfunction

" Update the CoffeeCompile buffer with the whole source buffer and restore the
" cursor.
function! s:CoffeeCompileWatchUpdate()
  call s:CoffeeCompileSavePos()
  call s:CoffeeCompileUpdate(1, '$')
  call s:CoffeeCompileRestorePos()
endfunction

" Peek at compiled CoffeeScript in a scratch buffer. We handle ranges like this
" to prevent the cursor from being moved (and its position saved) before the
" function is called.
function! s:CoffeeCompile(startline, endline, args)
  " Don't compile the CoffeeCompile buffer.
  if bufnr('%') == s:coffee_compile_buf
    return
  endif

  " Parse arguments.
  let watch = a:args =~ '\<watch\>'
  let unwatch = a:args =~ '\<unwatch\>'
  let vert = a:args =~ '\<vert\%[ical]\>'
  let size = str2nr(matchstr(a:args, '\<\d\+\>'))

  " Remove any watch listeners.
  silent! autocmd! CoffeeCompileAuWatch

  " If just unwatching, don't compile.
  if unwatch
    let s:coffee_compile_watch = 0
    return
  endif

  if watch
    let s:coffee_compile_watch = 1
  endif

  call s:CoffeeCompileSavePos()

  " Build the CoffeeCompile buffer if it doesn't exist.
  if s:coffee_compile_buf == -1
    let src_win = bufwinnr(s:coffee_compile_src_buf)

    " Create the new window and resize it.
    if vert
      let width = size ? size : winwidth(src_win) / 2

      vertical new
      exec 'vertical resize' width
    else
      " Try to guess the compiled output's height.
      let height = size ? size : min([winheight(src_win) / 2,
      \                               a:endline - a:startline + 2])

      botright new
      exec 'resize' height
    endif

    " Set up scratch buffer.
    setlocal bufhidden=wipe buftype=nofile
    setlocal nobuflisted nomodifiable noswapfile nowrap

    autocmd BufWipeout <buffer> call s:CoffeeCompileClose()
    nnoremap <buffer> <silent> q :hide<CR>

    " Save the cursor position on each buffer switch.
    augroup CoffeeCompileAuPos
      autocmd BufEnter,BufLeave * call s:CoffeeCompileSavePos()
    augroup END

    let s:coffee_compile_buf = bufnr('%')
    let s:coffee_compile_win = bufwinnr(s:coffee_compile_buf)
  endif

  " Go back to the source buffer and do the initial compile.
  call s:CoffeeCompileRestorePos()

  if s:coffee_compile_watch
    call s:CoffeeCompileWatchUpdate()

    augroup CoffeeCompileAuWatch
      autocmd InsertLeave <buffer> call s:CoffeeCompileWatchUpdate()
    augroup END
  else
    call s:CoffeeCompileUpdate(a:startline, a:endline)
  endif
endfunction

" Complete arguments for the CoffeeCompile command.
function! s:CoffeeCompileComplete(arg, cmdline, cursor)
  let args = ['unwatch', 'vertical', 'watch']

  if !len(a:arg)
    return args
  endif

  let match = '^' . a:arg

  for arg in args
    if arg =~ match
      return [arg]
    endif
  endfor
endfunction

" Don't let new windows overwrite the CoffeeCompile variables.
if !exists("s:coffee_compile_buf")
  call s:CoffeeCompileResetVars()
endif

" Peek at compiled CoffeeScript.
command! -range=% -bar -nargs=* -complete=customlist,s:CoffeeCompileComplete
\        CoffeeCompile call s:CoffeeCompile(<line1>, <line2>, <q-args>)
" Compile the current file.
command! -bang -bar -nargs=* CoffeeMake make<bang> <args>
" Run some CoffeeScript.
command! -range=% -bar CoffeeRun <line1>,<line2>:w !coffee -s

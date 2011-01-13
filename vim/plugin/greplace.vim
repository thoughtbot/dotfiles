" File: greplace.vim
" Script to search and replace pattern across multiple files
" Author: Yegappan Lakshmanan (yegappan AT yahoo DOT com)
" Version: 1.0 Beta1
" Last Modified: March 3, 2007
"
if exists("loaded_greplace")
    finish
endif
let loaded_greplace = 1

" Requires Vim 7.0 and above
if v:version < 700
    finish
endif

" Line continuation used here
let s:cpo_save = &cpo
set cpo&vim

if &isfname =~ '['
    let s:gRepl_bufname = '[Global\ Replace]'
else
    let s:gRepl_bufname = '\[Global\ Replace\]'
endif

" Character to use to quote patterns
if !exists("Greplace_Shell_Quote_Char")
    if has("win32") || has("win16") || has("win95")
        let Greplace_Shell_Quote_Char = ''
    else
        let Greplace_Shell_Quote_Char = "'"
    endif
endif

let s:save_qf_list = {}

function! s:warn_msg(msg)
    echohl WarningMsg
    echomsg a:msg
    echohl None
endfunction

highlight GReplaceText term=reverse cterm=reverse gui=reverse

function! s:gReplace()
    if empty(s:save_qf_list)
        return
    endif

    let change_all = v:cmdbang

    let changeset = {}

    " Parse the replace buffer contents and get a List of changed lines
    let lines = getbufline('%', 1, '$')
    for l in lines
        if l !~ '[^:]\+:\d\+:.*'
            continue
        endif

        let match_l = matchlist(l, '\([^:]\+\):\(\d\+\):\(.*\)')
        let fname = match_l[1]
        let lnum = match_l[2]
        let text = match_l[3]

        let key = fname . ':' . lnum
        if s:save_qf_list[key].text == text
            " This line is not changed
            continue
        endif

        let fname = s:save_qf_list[key].fname
        if !has_key(changeset, fname)
            let changeset[fname] = {}
        endif

        let changeset[fname][lnum] = text
    endfor

    if empty(changeset)
        " The replace buffer is not changed by the user
        call s:warn_msg('Error: No changes in the replace buffer')
        return
    endif

    " Make the changes made by the user to the buffers
    for f in keys(changeset)
        let f_l = changeset[f]
        if !filereadable(f)
            continue
        endif
        silent! exe 'hide edit ' . f

        let change_buf_all = 0   " Accept all the changes in this buffer

        for lnum in keys(f_l)
            exe lnum

            let cur_ltext = getline(lnum)
            let new_ltext = f_l[lnum]

            let s_idx =0
            while cur_ltext[s_idx] == new_ltext[s_idx]
                let s_idx += 1
            endwhile

            let e_idx1 = strlen(cur_ltext) - 1
            let e_idx2 = strlen(new_ltext) - 1
            while e_idx1 >= 0 && cur_ltext[e_idx1] == new_ltext[e_idx2]
                let e_idx1 -= 1
                let e_idx2 -= 1
            endwhile

            let e_idx1 += 2

            if (s_idx + 1) == e_idx1 
                " If there is nothing to highlight, then highlight the
                " last character
                let e_idx1 += 1
            endif

            let hl_pat = '/\%'.lnum.'l\%>'.s_idx.'v.*\%<'.e_idx1.'v/'
            exe '2match GReplaceText ' . hl_pat
            redraw!

            try
                let change_line = 0

                if !change_all && !change_buf_all
                    let new_text_frag = strpart(new_ltext, s_idx,
                                \ e_idx2 - s_idx + 1)

                    echo "Replace with '" . new_text_frag . "' (y/n/a/b/q)?"
                    let ans = 'x'
                    while ans !~? '[ynab]'
                        let ans = nr2char(getchar())
                        if ans == 'q' || ans == "\<Esc>"      " Quit
                            return
                        endif
                    endwhile
                    if ans == 'a'       " Accept all
                        let change_all = 1
                    endif
                    if ans == 'b'       " Accept changes in the current buffer
                        let change_buf_all = 1
                    endif
                    if ans == 'y'       " Yes
                        let change_line = 1
                    endif
                endif

                if change_all || change_buf_all || change_line
                    call setline(lnum, f_l[lnum])
                endif
            finally
                2match none
            endtry
        endfor
    endfor
endfunction

function! s:gReplace_show_matches()
    let qf = getqflist()
    if empty(qf)
        call s:warn_msg('Error: Quickfix list is empty')
        return
    endif

    let new_qf = {}

    " Populate the buffer with the current quickfix list
    let lines = []
    for l in qf
        if l.valid && l.lnum > 0 && l.bufnr > 0
            let fname = fnamemodify(bufname(l.bufnr), ':.')
            let buf_text = fname . ':' . l.lnum . ':' . l.text
            let k = fname . ':' . l.lnum
            let new_qf[k] = {}
            let new_qf[k].fname = fnamemodify(bufname(l.bufnr), ':p')
            let new_qf[k].text = l.text
        else
            let buf_text = l.text
        endif

        call add(lines, buf_text)
    endfor

    if empty(lines)
        " No valid matching lines
        return
    endif

    let w = bufwinnr(s:gRepl_bufname)
    if w == -1
        " Create a new window
        silent! exe 'new ' . s:gRepl_bufname
    else
        exe w . 'wincmd w'

        " Discard the contents of the buffer
        %d _
    endif

    call append(0, '#')
    call append(1, '# Modify the contents of this buffer and then')
    call append(2, '# use the ":Greplace" command to merge the changes.')
    call append(3, '#')
    call append(4, lines)

    call cursor(5, 1)
    setlocal buftype=nofile
    setlocal bufhidden=wipe
    setlocal nomodified

    command! -buffer -nargs=0 -bang Greplace call s:gReplace()

    let s:save_qf_list = new_qf
endfunction

" gSearch
" Search for a pattern in a group of files using ':grep'
function! s:gSearch(type, ...)
    let grep_opt  = ''
    let pattern   = ''
    let filenames = ''

    " Parse the arguments
    " grep command-line flags are specified using the "-flag" format
    " the next argument is assumed to be the pattern
    " and the next arguments are assumed to be filenames or file patterns
    let argcnt = 1
    while argcnt <= a:0
        if &grepprg =~ 'findstr' && a:{argcnt} =~ '^/'
            let grep_opt = grep_opt . ' ' . a:{argcnt}
        elseif a:{argcnt} =~ '^-'
            let grep_opt = grep_opt . ' ' . a:{argcnt}
        elseif pattern == ''
            let pattern = a:{argcnt}
        else
            let filenames = filenames . ' ' . a:{argcnt}
        endif
        let argcnt += 1
    endwhile

    " If search pattern is not specified on command-line, ask for it
    if pattern == ''
        let pattern = input('Search pattern: ', expand('<cword>'))
        if pattern == ''
            return
        endif

        " Quote the supplied pattern
        let pattern = g:Greplace_Shell_Quote_Char . pattern .
                    \ g:Greplace_Shell_Quote_Char
    endif

    if a:type == 'grep'
        if filenames == ''
            let filenames = input('Search in files: ', '*', 'file')
        endif
    elseif a:type == 'args'
        " Search in all the filenames in the argument list
        let arg_cnt = argc()

        if arg_cnt == 0
            call s:warn_msg('Error: Argument list is empty')
            return
        endif

        let filenames = ''
        for i in range(0, arg_cnt - 1)
            let filenames .= ' ' . argv(i)
        endfor
    else
        " Get a list of all the buffer names
        let filenames = ''
        for i in range(1, bufnr('$'))
            let bname = bufname(i)
            if bufexists(i) && buflisted(i) && filereadable(bname) &&
                        \ getbufvar(i, '&buftype') == ''
                let filenames .= ' ' . bufname(i)
            endif
        endfor
    endif

    if filenames == ''
        call s:warn_msg('Error: No valid file names')
        return
    endif

    " Use ! after grep, so that Vim doesn't automatically jump to the
    " first match
    let grep_cmd = 'grep! ' . grep_opt . ' ' . pattern . ' ' . filenames

    " Run the grep and get the matches
    exe grep_cmd

    call s:gReplace_show_matches()
endfunction

command! -nargs=0 Gqfopen call s:gReplace_show_matches()
command! -nargs=* -complete=file Gsearch call s:gSearch('grep', <f-args>)
command! -nargs=* Gargsearch call s:gSearch('args', <f-args>)
command! -nargs=* Gbuffersearch call s:gSearch('buffer', <f-args>)

" restore 'cpo'
let &cpo = s:cpo_save
unlet s:cpo_save


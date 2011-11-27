" endwise.vim - EndWise
" Author:       Tim Pope <vimNOSPAM@tpope.info>
" Version:      1.0

" Distributable under the same terms as Vim itself (see :help license)

" Exit quickly when:
" - this plugin was already loaded (or disabled)
" - when 'compatible' is set
if (exists("g:loaded_endwise") && g:loaded_endwise) || &cp
    finish
endif
let g:loaded_endwise = 1


let s:cpo_save = &cpo
set cpo&vim

augroup endwise " {{{1
    au!
    autocmd FileType ruby
                \ let b:endwise_addition = '\=submatch(0)=="{" ? "}" : "end"' |
                \ let b:endwise_words = 'module,class,def,if,unless,case,while,until,begin,do' |
                \ let b:endwise_pattern = '^\s*\zs\%(module\|class\|def\|if\|unless\|case\|while\|until\|for\|\|begin\)\>\%(.*[^.:@$]\<end\>\)\@!\|\<do\ze\%(\s*|.*|\)\=\s*$' |
                \ let b:endwise_syngroups = 'rubyModule,rubyClass,rubyDefine,rubyControl,rubyConditional,rubyRepeat'
    autocmd FileType vb,vbnet,aspvbs
                \ let b:endwise_addition = 'End &' |
                \ let b:endwise_words = 'Function,Sub,Class,Module,Enum,Namespace' |
                \ let b:endwise_pattern = '\%(\<End\>.*\)\@<!\<&\>' |
                \ let b:endwise_syngroups = 'vbStatement,vbnetStorage,vbnetProcedure,vbnet.*Words,AspVBSStatement'
    autocmd FileType vim
                \ let b:endwise_addition = 'end&' |
                \ let b:endwise_words = 'fu\%[nction],wh\%[ile],if,for,try' |
                \ let b:endwise_syngroups = 'vimFuncKey,vimNotFunc,vimCommand'
augroup END " }}}1

" Maps {{{1


if maparg("<Plug>DiscretionaryEnd") == ""
    inoremap <silent> <SID>DiscretionaryEnd <C-R>=<SID>crend(0)<CR>
    inoremap <silent> <SID>AlwaysEnd        <C-R>=<SID>crend(1)<CR>
    imap    <script> <Plug>DiscretionaryEnd <SID>DiscretionaryEnd
    imap    <script> <Plug>AlwaysEnd        <SID>AlwaysEnd
endif
if maparg('<CR>','i') =~# '<C-R>=.*crend(.)<CR>\|<\%(Plug\|SID\)>.*End'
    " Already mapped
elseif maparg('<CR>','i') =~ '<CR>'
    exe "imap <script> <C-X><CR> ".maparg('<CR>','i')."<SID>AlwaysEnd"
    exe "imap <script> <CR>      ".maparg('<CR>','i')."<SID>DiscretionaryEnd"
else
    imap <C-X><CR> <CR><Plug>AlwaysEnd
    imap <CR>      <CR><Plug>DiscretionaryEnd
endif

if maparg('<M-o>','i') == ''
    inoremap <M-o> <C-O>o
endif

" }}}1

" Code {{{1

function! s:mysearchpair(beginpat,endpat,synpat)
    let g:endwise_syntaxes = ""
    let s:lastline = line('.')
    call s:synname()
    let line = searchpair(a:beginpat,'',a:endpat,'Wn','<SID>synname() !~# "^'.substitute(a:synpat,'\\','\\\\','g').'$"',line('.')+50)
    return line
endfunction

function! s:crend(always)
    let n = ""
    if !exists("b:endwise_addition") || !exists("b:endwise_words") || !exists("b:endwise_syngroups")
        return n
    end
    let synpat  = '\%('.substitute(b:endwise_syngroups,',','\\|','g').'\)'
    let wordchoice = '\%('.substitute(b:endwise_words,',','\\|','g').'\)'
    if exists("b:endwise_pattern")
        let beginpat = substitute(b:endwise_pattern,'&',substitute(wordchoice,'\\','\\&','g'),'g')
    else
        let beginpat = '\<'.wordchoice.'\>'
    endif
    let lnum = line('.') - 1
    let space = matchstr(getline(lnum),'^\s*')
    let col  = match(getline(lnum),beginpat) + 1
    let word  = matchstr(getline(lnum),beginpat)
    let endpat = substitute(word,'.*',b:endwise_addition,'')
    let y = n.endpat."\<C-O>O"
    let endpat = '\<'.substitute(wordchoice,'.*',b:endwise_addition,'').'\>'
    if a:always
        return y
    elseif col <= 0 || synIDattr(synID(lnum,col,1),'name') !~ '^'.synpat.'$'
        return n
    elseif getline('.') !~ '^\s*#\=$'
        return n
    endif
    let line = s:mysearchpair(beginpat,endpat,synpat)
    " even is false if no end was found, or if the end found was less
    " indented than the current line
    let even = strlen(matchstr(getline(line),'^\s*')) >= strlen(space)
    if line == 0
        let even = 0
    endif
    let g:endwise_debug = line . "(" . even . ")"
    if !even && line == line('.') + 1
        return y
    endif
    if even
        return n
    endif
    return y
endfunction

function! s:synname()
    " Checking this helps to force things to stay in sync
    while s:lastline < line('.')
        let s = synIDattr(synID(s:lastline,indent(s:lastline)+1,1),'name')
        let s:lastline = nextnonblank(s:lastline + 1)
    endwhile

    let s = synIDattr(synID(line('.'),col('.'),1),'name')
    let g:endwise_syntaxes = g:endwise_syntaxes . line('.').','.col('.')."=".s."\n"
    let s:lastline = line('.')
    return s
endfunction

" }}}1

let &cpo = s:cpo_save

" vim:set ft=vim ff=unix ts=8 sw=4 sts=4:

" rails.vim - Detect a rails application
" Author:       Tim Pope <vimNOSPAM@tpope.info>
" GetLatestVimScripts: 1567 1 :AutoInstall: rails.vim
" URL:          http://rails.vim.tpope.net/

" Install this file as plugin/rails.vim.  See doc/rails.txt for details. (Grab
" it from the URL above if you don't have it.)  To access it from Vim, see
" :help add-local-help (hint: :helptags ~/.vim/doc) Afterwards, you should be
" able to do :help rails

" ============================================================================

" Exit quickly when:
" - this plugin was already loaded (or disabled)
" - when 'compatible' is set
if &cp || (exists("g:loaded_rails") && g:loaded_rails) && !(exists("g:rails_debug") && g:rails_debug)
  finish
endif
let g:loaded_rails = 1

" Utility Functions {{{1

function! s:error(str)
  echohl ErrorMsg
  echomsg a:str
  echohl None
  let v:errmsg = a:str
endfunction

function! s:autoload(...)
  if !exists("g:autoloaded_rails") && v:version >= 700
    runtime! autoload/rails.vim
  endif
  if exists("g:autoloaded_rails")
    if a:0
      exe a:1
    endif
    return 1
  endif
  if !exists("g:rails_no_autoload_warning")
    let g:rails_no_autoload_warning = 1
    if v:version >= 700
      call s:error("Disabling rails.vim: autoload/rails.vim is missing")
    else
      call s:error("Disabling rails.vim: Vim version 7 or higher required")
    endif
  endif
  return ""
endfunction

" }}}1
" Configuration {{{

function! s:SetOptDefault(opt,val)
  if !exists("g:".a:opt)
    let g:{a:opt} = a:val
  endif
endfunction

call s:SetOptDefault("rails_statusline",1)
call s:SetOptDefault("rails_syntax",1)
call s:SetOptDefault("rails_mappings",1)
call s:SetOptDefault("rails_abbreviations",1)
call s:SetOptDefault("rails_ctags_arguments","--exclude=\"*.js\"")
call s:SetOptDefault("rails_expensive",1)
call s:SetOptDefault("rails_dbext",g:rails_expensive)
call s:SetOptDefault("rails_default_file","README")
call s:SetOptDefault("rails_default_database","")
call s:SetOptDefault("rails_root_url",'http://localhost:3000/')
call s:SetOptDefault("rails_modelines",0)
call s:SetOptDefault("rails_menu",1)
call s:SetOptDefault("rails_gnu_screen",1)
call s:SetOptDefault("rails_history_size",5)
call s:SetOptDefault("rails_generators","controller\nintegration_test\nmailer\nmigration\nmodel\nobserver\nplugin\nresource\nscaffold\nsession_migration")
if g:rails_dbext
  if exists("g:loaded_dbext") && executable("sqlite3") && ! executable("sqlite")
    " Since dbext can't find it by itself
    call s:SetOptDefault("dbext_default_SQLITE_bin","sqlite3")
  endif
endif

" }}}1
" Detection {{{1

function! s:escvar(r)
  let r = fnamemodify(a:r,':~')
  let r = substitute(r,'\W','\="_".char2nr(submatch(0))."_"','g')
  let r = substitute(r,'^\d','_&','')
  return r
endfunction

function! s:Detect(filename)
  let fn = substitute(fnamemodify(a:filename,":p"),'\c^file://','','')
  let sep = matchstr(fn,'^[^\\/]\{3,\}\zs[\\/]')
  if sep != ""
    let fn = getcwd().sep.fn
  endif
  if fn =~ '[\/]config[\/]environment\.rb$'
    return s:BufInit(strpart(fn,0,strlen(fn)-22))
  endif
  if isdirectory(fn)
    let fn = fnamemodify(fn,':s?[\/]$??')
  else
    let fn = fnamemodify(fn,':s?\(.*\)[\/][^\/]*$?\1?')
  endif
  let ofn = ""
  let nfn = fn
  while nfn != ofn && nfn != ""
    if exists("s:_".s:escvar(nfn))
      return s:BufInit(nfn)
    endif
    let ofn = nfn
    let nfn = fnamemodify(nfn,':h')
  endwhile
  let ofn = ""
  while fn != ofn
    if filereadable(fn . "/config/environment.rb")
      return s:BufInit(fn)
    endif
    let ofn = fn
    let fn = fnamemodify(ofn,':s?\(.*\)[\/]\(app\|config\|db\|doc\|features\|lib\|log\|public\|script\|spec\|stories\|test\|tmp\|vendor\)\($\|[\/].*$\)?\1?')
  endwhile
  return 0
endfunction

function! s:BufInit(path)
  let s:_{s:escvar(a:path)} = 1
  if s:autoload()
    return RailsBufInit(a:path)
  endif
endfunction

" }}}1
" Initialization {{{1

augroup railsPluginDetect
  autocmd!
  autocmd BufNewFile,BufRead * call s:Detect(expand("<afile>:p"))
  autocmd VimEnter * if expand("<amatch>") == "" && !exists("b:rails_root") | call s:Detect(getcwd()) | endif | if exists("b:rails_root") | silent doau User BufEnterRails | endif
  autocmd FileType netrw if !exists("b:rails_root") | call s:Detect(expand("<afile>:p")) | endif | if exists("b:rails_root") | silent doau User BufEnterRails | endif
  autocmd BufEnter * if exists("b:rails_root")|silent doau User BufEnterRails|endif
  autocmd BufLeave * if exists("b:rails_root")|silent doau User BufLeaveRails|endif
  autocmd Syntax railslog if s:autoload()|call rails#log_syntax()|endif
augroup END

command! -bar -bang -nargs=* -complete=dir Rails :if s:autoload()|call rails#new_app_command(<bang>0,<f-args>)|endif

" }}}1
" Menus {{{1

if !(g:rails_menu && has("menu"))
  finish
endif

function! s:sub(str,pat,rep)
  return substitute(a:str,'\v\C'.a:pat,a:rep,'')
endfunction

function! s:gsub(str,pat,rep)
  return substitute(a:str,'\v\C'.a:pat,a:rep,'g')
endfunction

function! s:menucmd(priority)
  return 'anoremenu <script> '.(exists("$CREAM") ? 87 : '').s:gsub(g:rails_installed_menu,'[^.]','').'.'.a:priority.' '
endfunction

function! s:CreateMenus() abort
  if exists("g:rails_installed_menu") && g:rails_installed_menu != ""
    exe "aunmenu ".s:gsub(g:rails_installed_menu,'\&','')
    unlet g:rails_installed_menu
  endif
  if has("menu") && (exists("g:did_install_default_menus") || exists("$CREAM")) && g:rails_menu
    if g:rails_menu > 1
      let g:rails_installed_menu = '&Rails'
    else
      let g:rails_installed_menu = '&Plugin.&Rails'
    endif
    let dots = s:gsub(g:rails_installed_menu,'[^.]','')
    let menucmd = s:menucmd(200)
    if exists("$CREAM")
      exe menucmd.g:rails_installed_menu.'.-PSep- :'
      exe menucmd.g:rails_installed_menu.'.&Related\ file\	:R\ /\ Alt+] :R<CR>'
      exe menucmd.g:rails_installed_menu.'.&Alternate\ file\	:A\ /\ Alt+[ :A<CR>'
      exe menucmd.g:rails_installed_menu.'.&File\ under\ cursor\	Ctrl+Enter :Rfind<CR>'
    else
      exe menucmd.g:rails_installed_menu.'.-PSep- :'
      exe menucmd.g:rails_installed_menu.'.&Related\ file\	:R\ /\ ]f :R<CR>'
      exe menucmd.g:rails_installed_menu.'.&Alternate\ file\	:A\ /\ [f :A<CR>'
      exe menucmd.g:rails_installed_menu.'.&File\ under\ cursor\	gf :Rfind<CR>'
    endif
    exe menucmd.g:rails_installed_menu.'.&Other\ files.Application\ &Controller :find app/controllers/application.rb<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.Application\ &Helper :find app/helpers/application_helper.rb<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.Application\ &Javascript :find public/javascripts/application.js<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.Application\ &Layout :Rlayout application<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.Application\ &README :find doc/README_FOR_APP<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.&Environment :find config/environment.rb<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.&Database\ Configuration :find config/database.yml<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.Database\ &Schema :Rmigration 0<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.R&outes :find config/routes.rb<CR>'
    exe menucmd.g:rails_installed_menu.'.&Other\ files.&Test\ Helper :find test/test_helper.rb<CR>'
    exe menucmd.g:rails_installed_menu.'.-FSep- :'
    exe menucmd.g:rails_installed_menu.'.Ra&ke\	:Rake :Rake<CR>'
    let menucmd = substitute(menucmd,'200 $','500 ','')
    exe menucmd.g:rails_installed_menu.'.&Server\	:Rserver.&Start\	:Rserver :Rserver<CR>'
    exe menucmd.g:rails_installed_menu.'.&Server\	:Rserver.&Force\ start\	:Rserver! :Rserver!<CR>'
    exe menucmd.g:rails_installed_menu.'.&Server\	:Rserver.&Kill\	:Rserver!\ - :Rserver! -<CR>'
    exe substitute(menucmd,'<script>','<script> <silent>','').g:rails_installed_menu.'.&Evaluate\ Ruby\.\.\.\	:Rp :call <SID>menuprompt("Rp","Code to execute and output: ")<CR>'
    exe menucmd.g:rails_installed_menu.'.&Console\	:Rscript :Rscript console<CR>'
    exe menucmd.g:rails_installed_menu.'.&Preview\	:Rpreview :Rpreview<CR>'
    exe menucmd.g:rails_installed_menu.'.&Log\ file\	:Rlog :Rlog<CR>'
    exe substitute(s:sub(menucmd,'anoremenu','vnoremenu'),'<script>','<script> <silent>','').g:rails_installed_menu.'.E&xtract\ as\ partial\	:Rextract :call <SID>menuprompt("'."'".'<,'."'".'>Rextract","Partial name (e.g., template or /controller/template): ")<CR>'
    exe menucmd.g:rails_installed_menu.'.&Migration\ writer\	:Rinvert :Rinvert<CR>'
    exe menucmd.'         '.g:rails_installed_menu.'.-HSep- :'
    exe substitute(menucmd,'<script>','<script> <silent>','').g:rails_installed_menu.'.&Help\	:help\ rails :if <SID>autoload()<Bar>exe RailsHelpCommand("")<Bar>endif<CR>'
    exe substitute(menucmd,'<script>','<script> <silent>','').g:rails_installed_menu.'.Abo&ut\	 :if <SID>autoload()<Bar>exe RailsHelpCommand("about")<Bar>endif<CR>'
    let g:rails_did_menus = 1
    call s:ProjectMenu()
    call s:menuBufLeave()
    if exists("b:rails_root")
      call s:menuBufEnter()
    endif
  endif
endfunction

function! s:ProjectMenu()
  if exists("g:rails_did_menus") && g:rails_history_size > 0
    if !exists("g:RAILS_HISTORY")
      let g:RAILS_HISTORY = ""
    endif
    let history = g:RAILS_HISTORY
    let menu = s:gsub(g:rails_installed_menu,'\&','')
    silent! exe "aunmenu <script> ".menu.".Projects"
    let dots = s:gsub(menu,'[^.]','')
    exe 'anoremenu <script> <silent> '.(exists("$CREAM") ? '87' : '').dots.'.100 '.menu.'.Pro&jects.&New\.\.\.\	:Rails :call <SID>menuprompt("Rails","New application path and additional arguments: ")<CR>'
    exe 'anoremenu <script> '.menu.'.Pro&jects.-FSep- :'
    while history =~ '\n'
      let proj = matchstr(history,'^.\{-\}\ze\n')
      let history = s:sub(history,'^.{-}\n','')
      exe 'anoremenu <script> '.menu.'.Pro&jects.'.s:gsub(proj,'[.\\ ]','\\&').' :e '.s:gsub(proj."/".g:rails_default_file,'[ !%#]','\\&')."<CR>"
    endwhile
  endif
endfunction

function! s:menuBufEnter()
  if exists("g:rails_installed_menu") && g:rails_installed_menu != ""
    let menu = s:gsub(g:rails_installed_menu,'\&','')
    exe 'amenu enable '.menu.'.*'
    if RailsFileType() !~ '^view\>'
      exe 'vmenu disable '.menu.'.Extract\ as\ partial'
    endif
    if RailsFileType() !~ '^\%(db-\)\=migration$' || RailsFilePath() =~ '\<db/schema\.rb$'
      exe 'amenu disable '.menu.'.Migration\ writer'
    endif
    call s:ProjectMenu()
    silent! exe 'aunmenu       '.menu.'.Rake\ tasks'
    silent! exe 'aunmenu       '.menu.'.Generate'
    silent! exe 'aunmenu       '.menu.'.Destroy'
    if rails#app().cache.needs('rake_tasks') || empty(rails#app().rake_tasks())
      exe substitute(s:menucmd(300),'<script>','<script> <silent>','').g:rails_installed_menu.'.Rake\ &tasks\	:Rake.Fill\ this\ menu :call rails#app().rake_tasks()<Bar>call <SID>menuBufLeave()<Bar>call <SID>menuBufEnter()<CR>'
    else
      let i = 0
      while i < len(rails#app().rake_tasks())
        let task = rails#app().rake_tasks()[i]
        exe s:menucmd(300).g:rails_installed_menu.'.Rake\ &tasks\	:Rake.'.s:sub(task,':',':.').' :Rake '.task.'<CR>'
        let i += 1
      endwhile
    endif
    let i = 0
    let menucmd = substitute(s:menucmd(400),'<script>','<script> <silent>','').g:rails_installed_menu
    while i < len(rails#app().generators())
      let generator = rails#app().generators()[i]
      exe menucmd.'.&Generate\	:Rgen.'.s:gsub(generator,'_','\\ ').' :call <SID>menuprompt("Rgenerate '.generator.'","Arguments for script/generate '.generator.': ")<CR>'
      exe menucmd.'.&Destroy\	:Rdestroy.'.s:gsub(generator,'_','\\ ').' :call <SID>menuprompt("Rdestroy '.generator.'","Arguments for script/destroy '.generator.': ")<CR>'
      let i += 1
    endwhile
  endif
endfunction

function! s:menuBufLeave()
  if exists("g:rails_installed_menu") && g:rails_installed_menu != ""
    let menu = s:gsub(g:rails_installed_menu,'\&','')
    exe 'amenu disable '.menu.'.*'
    exe 'amenu enable  '.menu.'.Help\	'
    exe 'amenu enable  '.menu.'.About\	'
    exe 'amenu enable  '.menu.'.Projects'
    silent! exe 'aunmenu       '.menu.'.Rake\ tasks'
    silent! exe 'aunmenu       '.menu.'.Generate'
    silent! exe 'aunmenu       '.menu.'.Destroy'
    exe s:menucmd(300).g:rails_installed_menu.'.Rake\ tasks\	:Rake.-TSep- :'
    exe s:menucmd(400).g:rails_installed_menu.'.&Generate\	:Rgen.-GSep- :'
    exe s:menucmd(400).g:rails_installed_menu.'.&Destroy\	:Rdestroy.-DSep- :'
  endif
endfunction

function! s:menuprompt(vimcmd,prompt)
  let res = inputdialog(a:prompt,'','!!!')
  if res == '!!!'
    return ""
  endif
  exe a:vimcmd." ".res
endfunction

call s:CreateMenus()

augroup railsPluginMenu
  autocmd!
  autocmd User BufEnterRails call s:menuBufEnter()
  autocmd User BufLeaveRails call s:menuBufLeave()
  " g:RAILS_HISTORY hasn't been set when s:InitPlugin() is called.
  autocmd VimEnter *         call s:ProjectMenu()
augroup END

" }}}1
" vim:set sw=2 sts=2:

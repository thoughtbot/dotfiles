 set nocompatible
 filetype off

" Vundle setup
set rtp+=~/.vim/bundle/vundle/
call vundle#rc()
Bundle 'gmarik/vundle'
filetype plugin indent on

" Essential
Bundle 'altercalation/vim-colors-solarized.git'
Bundle 'tpope/vim-unimpaired.git'

" Plugins that help with ctags integration
Bundle 'tpope/vim-fugitive.git'
Bundle 'tpope/vim-bundler.git'
Bundle 'vim-ruby/vim-ruby'
Bundle 'tpope/vim-rake.git'


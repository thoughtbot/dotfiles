thoughtbot dotfiles
===================

Install
-------

[Fork this repo](https://github.com/jsteiner/dotfiles/fork_select) on Github.

Clone your fork (replace `your-github-name` with your Github name).

    git clone git@github.com:your-github-name/dotfiles.git
    cd dotfiles

Download submodules

    git submodule init
    git submodule update

Run the installer.

    ./install.sh

It creates symlinks for all dotfiles in your home directory. You can safely run
this file multiple times to update.

Install Xcode and the Xcode Command Line Tools.

Install [homebrew](http://mxcl.github.com/homebrew/).

    brew install git vim zsh the_silver_searcher fasd
    brew install tmux reattach-to-user-namespace
    brew install rbenv ruby-build

Install
[tmux-vim-select-pane](https://github.com/derekprior/tmux-vim-select-pane)

Install [Solarized for
iTerm2](https://github.com/altercation/solarized/tree/master/iterm2-colors-solarized)

Remap `Caps Lock` to `Control`. Thank me later.

Set scrollback buffer in iTerm2 to 0. Use tmux!

Why fork?
---------

Your master branch is meant for your customizations. Use the `upstream` branch
to get updates.

Set up upstream
---------------

Do this once:

    git remote add upstream git@github.com:jsteiner/dotfiles.git
    git fetch upstream
    git checkout -b upstream upstream/master

Update upstream
---------------

Make changes in files that are not in my dotfiles.

For example, to customize your `zsh` config, make your changes in `~/.zshenv`:

    # RVM
    [[ -s '/Users/croaky/.rvm/scripts/rvm' ]] && source '/Users/croaky/.rvm/scripts/rvm'

    # recommended by brew doctor
    export PATH="/usr/local/bin:/usr/local/sbin:$PATH"

Commit those kinds of things in your master branch.

Then, each time you want to update my changes.

    git checkout upstream
    git pull
    git checkout master
    git rebase upstream

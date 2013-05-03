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

Install [homebrew](http://mxcl.github.com/homebrew/).

    brew install the_silver_searcher tmux fasd

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

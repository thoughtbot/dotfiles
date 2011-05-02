thoughtbot dotfiles
===================

Flow:

* Fork this repo.
* Clone your fork.
* Install.
* Track thoughtbot/dotfiles.
* Customize in master.
* Update.

Install
-------

From your cloned directory:

    ./install.sh

This will create symlinks for all config files in your home directory. You can
safely run this file multiple times to update.

Track thoughtbot/dotfiles
-------------------------

One time:

    git remote add upstream git@github.com:thoughtbot/dotfiles.git
    git fetch upstream
    git checkout -b upstream upstream/master

Update
------

Each time you want to update:

    git checkout upstream
    git pull
    git checkout master
    git rebase upstream

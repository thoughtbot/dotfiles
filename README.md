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

    git remote add thoughtbot git@github.com:thoughtbot/dotfiles.git
    git fetch thoughtbot
    git checkout -b thoughtbot thoughtbot/master

Update
------

    git checkout thoughtbot
    git pull --rebase
    git checkout master
    git merge thoughtbot

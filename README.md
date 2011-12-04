thoughtbot dotfiles
===================

Install
-------

First, [fork this repo](https://github.com/thoughtbot/dotfiles#fork_box) on Github.

Then, clone your Github fork (replace "your-github-name" with your Github name) onto your laptop and install it:

    git clone git@github.com:your-github-name/dotfiles.git
    cd dotfiles
    ./install.sh

This will create symlinks for all config files in your home directory. You can
safely run this file multiple times to update.

There is configuration for `zsh` so switch your shell from the default `bash` to `zsh` on OS X:

    chsh -s /bin/zsh

Why fork?
---------

dotfiles are fairly personal. You should be able to modify your dotfiles, and save them in version control in your fork.

However, the thoughtbot folks are often tweaking these dotfiles and you want to be able to get those updates.

So, your master branch is meant for your customizations and use the `upstream` branch to get thoughtbot's updates.

Set up the upstream branch
--------------------------

You only have to do this once:

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

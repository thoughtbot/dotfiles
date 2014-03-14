# dotfiles

## Install

Set `zsh` as your login shell:

chsh -s $(which zsh)

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

Rename `/etc/zshenv` to `/etc/zprofile`.

    $ sudo mv /etc/zshenv /etc/zprofile

By default, OS X's zsh config resets `PATH` for every zsh instance, which can
cause problems with Vim. More info [here].

## Dependencies

Install Xcode and the Xcode Command Line Tools.

Install [homebrew](http://mxcl.github.com/homebrew/).

Run `brew bundle` to install all packages in the `Brewfile`

[tmux-vim-select-pane](https://github.com/derekprior/tmux-vim-select-pane)

Remap `Caps Lock` to `Control`. Thank me later.

Configure iTerm:

* Install [Solarized for
iTerm2](https://github.com/altercation/solarized/tree/master/iterm2-colors-solarized)
* Under Profiles > Terminal
  * Set `Scrollback Lines` to 0 if you are using tmux.
  * Set `Report Terminal Type` to xterm-256color
* Under Profiles > Text
  * Disable `Draw bold text in bright colors`

## Vundle

Set up vundle: `git clone https://github.com/gmarik/vundle.git
~/.vim/bundle/vundle`

Then run `:BundleInstall` from within vim.

## Make your own customizations

Put your customizations in dotfiles appended with `.local`:

* `~/.aliases.local`
* `~/.gitconfig.local`
* `~/.tmux.conf.local`
* `~/.vimrc.local`
* `~/.vimrc.bundles.local`
* `~/.zshrc.local`

For example, your `~/.aliases.local` might look like this:

    # Productivity
    alias todo='$EDITOR ~/.todo'

Your `~/.gitconfig.local` might look like this:

    [alias]
      l = log --pretty=colored
    [pretty]
      colored = format:%Cred%h%Creset %s %Cgreen(%cr) %C(bold blue)%an%Creset
    [user]
      name = Dan Croak
      email = dan@thoughtbot.com

Your `~/.zshrc.local` might look like this:

    # recommended by brew doctor
    export PATH="/usr/local/bin:/usr/local/sbin:$PATH"

Your `~/.vimrc.bundles.local` might look like this:

    Bundle 'Lokaltog/vim-powerline'
    Bundle 'stephenmckinney/vim-solarized-powerline'

## Git Safe

`.git/safe/../../bin` has been added to the path.
Do `mkdir .git/safe` in the root of repositories that you trust, for binstubs
in those repos to be added to your path.

## Why fork?

Your master branch is meant for your customizations. Use the `upstream` branch
to get updates.

## Set up upstream

Do this once:

    git remote add upstream git@github.com:jsteiner/dotfiles.git
    git fetch upstream
    git checkout -b upstream upstream/master

## Update upstream

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

[here]: https://github.com/b4winckler/macvim/wiki/Troubleshooting#wiki-rename-the-etczshenv-file-to-etczprofile

# dotfiles

## Install

Install and use [iTerm2].

Set `zsh` as your login shell:

    chsh -s $(which zsh)

Change to your home directory and clone this repo, then change to the new dotfiles directory.

    cd && git clone https://github.com/jsteiner/dotfiles.git
    cd dotfiles

Download submodules

    git submodule init
    git submodule update

Install [homebrew](http://mxcl.github.com/homebrew/).

Run `brew bundle` to install all packages in the `Brewfile`

    brew bundle

Install:

    rcup -d ~/dotfiles -x README.md -x Brewfile

This will create symlinks for config files in your home directory. The
`-x` options, which exclude the `README.md` and `Brewfile` files, are
needed during installation but can be skipped after the first time.

Rename `/etc/zshenv` to `/etc/zprofile`.

    sudo mv /etc/zshenv /etc/zprofile

By default, OS X's zsh config resets `PATH` for every zsh instance, which can
cause problems with Vim. More info [here].

Set up vundle:

    git clone https://github.com/gmarik/vundle.git ~/.vim/bundle/vundle
    vim +BundleInstall +qa

Remap `Caps Lock` to `Control`. Thank me later.

    System Preferences -> Keyboard -> Keyboard Tab -> Modifier Keys

Configure iTerm:

* Install [Solarized for iTerm2]
* Under Profiles > Terminal
  * Set `Scrollback Lines` to 0 if you are using tmux.
  * Set `Report Terminal Type` to xterm-256color
* Under Profiles > Text
  * Disable `Draw bold text in bright colors`

## Updating

Pull down changes:

    git pull origin master

Use [rcm] to symlink new dotfiles:

    rcup

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

`.git/safe/../../bin` has been added to the path. Run `mkdir .git/safe` in the
root of repositories that you trust, for binstubs in those repos to be added to
your path.

## What's included?

[vim](http://www.vim.org/) configuration:

* [Ctrl-P](https://github.com/kien/ctrlp.vim) for fuzzy file/buffer/tag finding.
* [Rails.vim](https://github.com/tpope/vim-rails) for enhanced navigation of
  Rails file structure.
* Run [RSpec](https://www.relishapp.com/rspec) specs from vim in tmux.
* Set `<leader>` to `,` (comma).
* Use [Ag](https://github.com/ggreer/the_silver_searcher) instead of Grep when
  available.
* Use Solarize color scheme, use `F5` to toggle between light and dark.
* Use [Vundle](https://github.com/gmarik/vundle) to manage plugins.

[tmux](http://robots.thoughtbot.com/a-tmux-crash-course)
configuration:

* Improve color resolution.
* Set prefix to `Ctrl+a` (like GNU screen).
* Seamlessly move between vim and tmux panes with `Ctrl+h,j,k,l`

Shell aliases and scripts:

* `j` for quickly navigating around the filesystem. See [fasd]
* `b` for `bundle`
* `g` for `git`
* `c` for `rails console`
* `tat` for `tmux attach -t`
* `tns` for `tmux new -s`

[git](http://git-scm.com/) configuration:

* Use concise formatting of git status with `gs`
* Use consise formatting of git log with `gl`

[Ruby](https://www.ruby-lang.org/en/) configuration:

* Add trusted binstubs to the PATH.
* Support for `rbenv` and `chruby`

[here]: https://github.com/b4winckler/macvim/wiki/Troubleshooting#wiki-rename-the-etczshenv-file-to-etczprofile
[Solarized for iTerm2]: https://github.com/altercation/solarized/tree/master/iterm2-colors-solarized
[iTerm2]: http://www.iterm2.com
[rcm]: http://thoughtbot.github.io/rcm/rcm.7.html
[fasd]: https://github.com/clvv/fasd

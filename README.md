thoughtbot dotfiles
===================

Requirements
------------

Set zsh as your login shell.

    chsh -s /bin/zsh

Install
-------

Clone onto your laptop:

    git clone git://github.com/thoughtbot/dotfiles.git

(Or, [fork and keep your fork
updated](http://robots.thoughtbot.com/post/5133345960)).

Install:

    cd dotfiles
    ./install.sh

This will create symlinks for config files in your home directory. If you
include the line "DO NOT EDIT BELOW THIS LINE" anywhere in a config file, it
will copy that file over instead of symlinking it, and it will leave
everything above that line in your local config intact.

You can safely run `./install.sh` multiple times to update.

Make your own customizations
----------------------------

Put your customizations at the top of files, separated by "DO NOT EDIT BELOW
THIS LINE."

For example, the top of your `~/.gitconfig` might look like this:

    [user]
      name = Joe Ferris
      email = jferris@thoughtbot.com

    # DO NOT EDIT BELOW THIS LINE

    [push]
      default = current

The top of your `~/.zlogin` might look like this:

    # Productivity
    alias todo='$EDITOR ~/.todo'

    # DO NOT EDIT BELOW THIS LINE

    # recommended by brew doctor
    export PATH="/usr/local/bin:/usr/local/sbin:$PATH"

What's in it?
-------------

[vim](http://www.vim.org/) configuration:

* [Ctrl-P](https://github.com/kien/ctrlp.vim) for fuzzy file/buffer/tag finding.
* [Rails.vim](https://github.com/tpope/vim-rails) for enhanced navigation of
  Rails file structure via `gf` and `:A` (alternate), `:Rextract` partials,
  `:Rinvert` migrations, etc.
* Run [RSpec](https://www.relishapp.com/rspec) specs from vim.
* Set `<leader>` to a single space.
* Switch between the last two files with space-space.
* Syntax highlighting for CoffeeScript, Textile, Cucumber, Haml, Markdown, and
  HTML.
* Use [Ag](https://github.com/ggreer/the_silver_searcher) instead of Grep when
  available.
* Use [Exuberant Ctags](http://ctags.sourceforge.net/) for tab completion.
* Use [GitHub color scheme](https://github.com/croaky/vim-colors-github).
* Use [Vundle](https://github.com/gmarik/vundle) to manage plugins.

[tmux](http://robots.thoughtbot.com/post/2641409235/a-tmux-crash-course)
configuration:

* Improve color resolution.
* Remove administrative debris (session name, hostname, time) in status bar.
* Set prefix to `Ctrl+a` (like GNU screen).
* Soften status bar color from harsh green to light gray.

[git](http://git-scm.com/) configuration:

* Adds a `create-branch` alias to create feature branches.
* Adds a `delete-branch` alias to delete feature branches.
* Adds a `merge-branch` alias to merge feature branches into master.

Shell aliases and scripts:

* `b` for `bundle`.
* `g` with no arguments is `git status` and with arguments acts like `git`.
* `git-churn` to show churn for the files changed in the branch.
* `load-backup-into development` or `load-backup-into staging` to load latest
  production database backup into development/staging.
* `m` for `rake db:migrate && rake db:rollback && rake db:migrate && rake db:test:prepare`.
* `mcd` to make a directory and change into it.
* `production backup`, `production migrate`, `production tail`, `watch
  production ps`, etc. to interact with production Heroku environment. This
  script also acts as a pass-through so you can do anything with it that you can
  do with `heroku _______ -r production`.
* `rake` is `zeus rake` if using [Zeus](https://github.com/burke/zeus) on the
  project in current directory.
* `replace foo bar **/*.rb` to find and replace within a given list of files.
* `rk` for `rake`.
* `rspec` is `zeus rspec` if using Zeus on the project in current directory.
* `staging` version of `production` script.
* `tat` to attach to tmux session named the same as the current directory.
* `v` for `$VISUAL`.

Credits
-------

Thank you, [contributors](/thoughtbot/dotfiles/graphs/contributors)!

![thoughtbot](http://thoughtbot.com/images/tm/logo.png)

Dotfiles is maintained by [thoughtbot, inc](http://thoughtbot.com/community)
The names and logos for thoughtbot are trademarks of thoughtbot, inc.

Dotfiles is © 2009-2013 thoughtbot, inc. It is free software and may be
redistributed under the terms specified in the MIT-LICENSE file.

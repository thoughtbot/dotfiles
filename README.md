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

Credits
-------

Thank you, [contributors](/thoughtbot/dotfiles/graphs/contributors)!

![thoughtbot](http://thoughtbot.com/images/tm/logo.png)

Dotfiles is maintained by [thoughtbot, inc](http://thoughtbot.com/community)
The names and logos for thoughtbot are trademarks of thoughtbot, inc.

Dotfiles is © 2009-2013 thoughtbot, inc. It is free software and may be
redistributed under the terms specified in the MIT-LICENSE file.

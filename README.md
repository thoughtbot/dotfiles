dotfiles
========

This repo tracks the small set of dotfiles I actually use.

Layout
------

Files in this repo are stored at the repo root *without* a leading dot.
They map to `$HOME` by prefixing with `.`.

Examples:

- `zshrc` -> `~/.zshrc`
- `gitconfig` -> `~/.gitconfig`
- `p10k.zsh` -> `~/.p10k.zsh`

Managed files
-------------

- `zshrc`
- `zshenv`
- `p10k.zsh`
- `tmux.conf`
- `gitconfig`
- `gitignore`
- `gitmessage`
- `ripgreprc`

Ignored / managed elsewhere
--------------------------

- Most of `~/.config/**` (app-specific), including `~/.config/mise/config.toml`
- LazyVim (`~/.config/nvim`)
- Coding agents (`~/.claude*`, `~/.codex*`, `~/.config/opencode`)

Private config
--------------

This repo does not store personal identity or secrets.

`gitconfig` includes `~/.gitconfig.private` for machine/user-specific settings.
Create it locally (and keep it private):

    touch ~/.gitconfig.private
    chmod 600 ~/.gitconfig.private

Example contents:

    [user]
      name = Your Name
      email = you@example.com
      signingkey = YOUR_KEY_ID

Linking (optional)
-----------------

No symlinks are created automatically. If you want symlinks, use:

    ./script/link-dotfiles --dry-run
    ./script/link-dotfiles --force

Deprecated
----------

Old thoughtbot-era dotfiles and unused configs are archived under `deprecated/`.

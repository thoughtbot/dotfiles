Los dotfiles de thoughtbot
==========================

![prompt](http://images.thoughtbot.com/thoughtbot-dotfiles-prompt.png)

Requerimientos
--------------

Establece zsh como tu shell de inicio de sesión:

    chsh -s $(which zsh)

Instalar
--------

Clona en tu laptop:

    git clone git://github.com/thoughtbot/dotfiles.git ~/dotfiles

(o [haz un fork y mantenlo actualizado](http://robots.thoughtbot.com/keeping-a-github-fork-updated)).

Instala [rcm](https://github.com/thoughtbot/rcm):

    brew tap thoughtbot/formulae
    brew install rcm

Instala los dotfiles:

    env RCRC=$HOME/dotfiles/rcrc rcup

Después de la instalación inicial, puedes ejecutarlo sin establecer la variable `RCRC`
(`rcup` establecerá un enlace simbólico (symlink) del repo `rcrc` hacia `~/.rcrc` para futuras
ejecuciones de `rcup`). [Ve el ejemplo](https://github.com/thoughtbot/dotfiles/blob/master/rcrc).

Este comando creará enlaces simbólicos (symlinks) para los archivos de configuración en tu
directorio principal.

Establecer la variable de entorno le dice a `rcup` que use las opciones de
configuración preestablecidas:

* Excluye los archivos `README.md`, `README-ES.md` y `LICENSE`, que son parte
  del repositorio `dotfiles`, pero no necesitan enlazarse simbólicamente.
* Le da precedencia a las modificaciones personales que por defecto están en
  `~/dotfiles-local`
* Por favor configura el archivo `rcrc` en caso de que quieras hacer
  modificaciones personales en un directorio distinto.


Actualizar
----------

De vez en cuando deberías descargar las actualizaciones de estos dotfiles, y ejectuar

    rcup

para ligar cualquier nuevo archivo e instalar los nuevos plugins de vim. **Nota** _Debes_ ejecutar
`rcup` después de descargar para asegurarte que todos los archivos de los plugins
estén instalados adecuadamente. Puedes ejecutar `rcup` con seguridad muchas veces
para actualizar pronto y muy seguido!


Haz tus propias modificaciones
------------------------------

Crea un directorio para tus modificaciones personales:

    mkdir ~/dotfiles-local

Pon tus modificaciones en `~/dotfiles-local` anexado con `.local`:

* `~/dotfiles-local/aliases.local`
* `~/dotfiles-local/git_template.local/*`
* `~/dotfiles-local/gitconfig.local`
* `~/dotfiles-local/psqlrc.local` (proveemos `.psqlrc.local` en blanco para prevenir que `psql`
  arroje un error, pero debes sobreescribir el archivo con tu propia copia)
* `~/dotfiles-local/tmux.conf.local`
* `~/dotfiles-local/vimrc.local`
* `~/dotfiles-local/vimrc.bundles.local`
* `~/dotfiles-local/zshrc.local`
* `~/dotfiles-local/zsh/configs/*`

Por ejemplo, tu `~/dotfiles-local/aliases.local` tal vez se vea así:

    # Productivity
    alias todo='$EDITOR ~/.todo'

Tu `~/dotfiles-local/gitconfig.local` tal vez se vea así:

    [alias]
      l = log --pretty=colored
    [pretty]
      colored = format:%Cred%h%Creset %s %Cgreen(%cr) %C(bold blue)%an%Creset
    [user]
      name = Dan Croak
      email = dan@thoughtbot.com

Tu `~/dotfiles-local/vimrc.local` tal vez se vea así:

    " Color scheme
    colorscheme github
    highlight NonText guibg=#060606
    highlight Folded  guibg=#0A0A0A guifg=#9090D0

Si prefieres prevenir la instalación de un plugin predeterminado de vim en `.vimrc.bundles`,
puedes ignorarlo sacándolo con `UnPlug` en tu `~/.vimrc.bundles.local`.

    " Don't install vim-scripts/tComment
    UnPlug 'tComment'

`UnPlug` puede ser usado para instalar tu propio fork de un plugin o para instalar
un plugin compartido con opciones personalizadas distintas.

    " Only load vim-coffee-script if a Coffeescript buffer is created
    UnPlug 'vim-coffee-script'
    Plug 'kchmck/vim-coffee-script', { 'for': 'coffee' }

    " Use a personal fork of vim-run-interactive
    UnPlug 'vim-run-interactive'
    Plug '$HOME/plugins/vim-run-interactive'

Para extender tus `git` hooks, crea scripts ejecutables en
`~/dotfiles-local/git_template.local/hooks/*` files.

Tu `~/dotfiles-local/zshrc.local` tal vez se vea así:

    # load pyenv if available
    if command -v pyenv &>/dev/null ; then
      eval "$(pyenv init -)"
    fi

Tu `~/dotfiles-local/vimrc.bundles.local` tal vez se vea así:

    Plug 'Lokaltog/vim-powerline'
    Plug 'stephenmckinney/vim-solarized-powerline'

Configuraciones de zsh
----------------------

Configuraciones adicionales para zsh pueden ir en el directorio `~/dotfiles-local/zsh/configs`. Este
tiene dos subdirectorios especiales: `pre` para archivos que deben ser cargados primero y `post`
para archivos que deben cargarse al final.

Por ejemplo, `~/dotfiles-local/zsh/configs/pre/virtualenv` hace uso de varias características
de shell que tal vez se vean afectadas por tu configuración, por lo tanto cárgalo primero:

    # Load the virtualenv wrapper
    . /usr/local/bin/virtualenvwrapper.sh

Establecer una vinculación clave puede ocurrir en `~/dotfiles-local/zsh/configs/keys`:

    # Grep anywhere with ^G
    bindkey -s '^G' ' | grep '

Algunos cambios, como `chpwd`, deben ocurrir en `~/dotfiles-local/zsh/configs/post/chpwd`:

    # Show the entries in a directory whenever you cd in
    function chpwd {
      ls
    }

Este directorio está a la mano para combinar dotfiles de múltiples equipos; un equipo
puede agregar el archivo `virtualenv`, otro el archivo `keys` y un tercero el archivo `chpwd`.

El archivo `~/dotfiles-local/zshrc.local` se carga después de `~/dotfiles-local/zsh/configs`.

Configuraciones de vim
----------------------

Similar al directorio de configuración para zsh descrito arriba, vim
automáticamente descarga los archivos en el directorio `~/dotfiles-local/vim/plugin`. Sin embargo, este no
tiene el mismo soporte para los subdirectorios `pre` ni `post` que tiene nuestro `zshrc`.

Este es un ejemplo `~/dotfiles-local/vim/plugin/c.vim`. Se carga cada vez que inicia vim,
sin importar de nombre del archivo:

    # Indent C programs according to BSD style(9)
    set cinoptions=:0,t0,+4,(4
    autocmd BufNewFile,BufRead *.[ch] setlocal sw=0 ts=8 noet

¿Qué viene incluido?
-----------------

Configuración [vim](http://www.vim.org/):

* [Ctrl-P](https://github.com/ctrlpvim/ctrlp.vim) para hallazgo difuso de archivos/buffer/tags.
* [Rails.vim](https://github.com/tpope/vim-rails) para una mejor navegación de la estructura
de archivos de Rails via `gf` y `:A` (alterno), `:Rextract` parciales,`:Rinvert` migraciones, etc.
* Ejecuta muchos tipos de pruebas [desde vim]([https://github.com/janko-m/vim-test)
* Establece `<leader>` a un sólo espacio.
* Navega entre los últimos dos archivos con espacio-espacio
* Resaltado de sintaxis para Markdown, HTML, JavaScript, Ruby, Go, Elixir, y más.
* Usa [Ag](https://github.com/ggreer/the_silver_searcher) en lugar de Grep cuando esté disponible.
* Map `<leader>ct` para re-indexar [Exuberant Ctags](http://ctags.sourceforge.net/).
* Usa [vim-mkdir](https://github.com/pbrisbin/vim-mkdir) para crear automáticamente directorios
  no existentes antes de escribir el buffer.
* Usa [vim-plug](https://github.com/junegunn/vim-plug) para administrar plugins.

[tmux](http://robots.thoughtbot.com/a-tmux-crash-course)
configuración:

* Mejora la resolición del color.
* Eliminar desechos administrativos(bombre de sesión, nombre de host, tiempo) en la barra de estatus.
* Establece el prefijo a `Ctrl+s`
* Suaviza el color de la barra de estatus de un verde chillante a un gris claro.

Configuración para [git](http://git-scm.com/):

* Agrega el alias `create-branch` para crear branches.
* Agrega el alias `delete-branch` para borrar branches.
* Agrega el alias `merge-branch` para fusionar los branches en master.
* Agrega el alias `up` para buscar y rebasar `origin/master` en el branch.
  Usa `git up -i` para rebases interactivos.
* Agrega el hook `post-{checkout,commit,merge}` para re-indexar tus ctags.
* Agrega `pre-commit` y `prepare-commit-msg` stubs que delegan hacia tu
  configuración local.
* Agrega el alias `trust-bin` para anexar el `bin/` de un proyecto al `$PATH`.

Configuración de [Ruby](https://www.ruby-lang.org/en/):

* Agrega binstubs confiables al `PATH`.
* Descarga el administrador de versiones ASDF.

Alias de Shell y scripts:

* `b` para `bundle`.
* `g` sin argumentos es `git status` y con argumentos funciona como `git`.
* `migrate` para `rake db:migrate && rake db:rollback && rake db:migrate`.
* `mcd` para crear un directorio e ir a él.
* `replace foo bar **/*.rb` para buscar y reemplazar en una lista dada de archivos.
* `tat` para adjuntar a una sesión de tmux llamada igual que el directorio actual.
* `v` para `$VISUAL`.

Gracias
-------

Gracias [Contribuyentes](https://github.com/thoughtbot/dotfiles/contributors)!
Además, gracias a Corey Haines, Gary Bernhardt, y otros por compartir sus dotfiles
y otros scripts de shell que derivaron en la inspiración para los artículos
en este proyecto.

Licencia
--------

dotfiles está protegida por copyright © 2009-2017 thoughtbot. Es un software gratis, y tal vez
redistribuido bajo los términos especificados en el archivo de la [`LICENCIA`]
[`LICENCIA`]: /LICENSE

Acerca de thoughtbot
--------------------

![thoughtbot](http://presskit.thoughtbot.com/images/thoughtbot-logo-for-readmes.svg)

dotfiles es mantenido y creado por thoughtbot, inc.
Los nombres y los logos de thoughtbot son marca registrada de thoughtbot, inc.

Amamos el código de fuente abiarta!
Ve [nuestros otros proyectos][community].
Estamos [disponibles para ser contratados][hire].

[community]: https://thoughtbot.com/community?utm_source=github
[hire]: https://thoughtbot.com/hire-us?utm_source=github

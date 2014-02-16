export PYTHONPATH=/usr/local/lib/python2.7/site-packages:$PYTHONPATH
export PIP_VIRTUALENV_BASE=$WORKON_HOME
export PIP_RESPECT_VIRTUALENV=true

source "/usr/local/bin/virtualenvwrapper.sh"

[[ -s $HOME/.pythonz/etc/bashrc ]] && source $HOME/.pythonz/etc/bashrc


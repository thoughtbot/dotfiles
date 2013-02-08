# completion
autoload -U compinit
compinit

# automatically enter directories without cd
setopt auto_cd

# use vim as an editor
export EDITOR=vim

# Put secret configuration settings in ~/.secrets
if [[ -a ~/.secrets ]] then
  source ~/.secrets
fi

# expand functions in the prompt
setopt prompt_subst

# prompt
export PS1='[${SSH_CONNECTION+"%n@%m:"}%~] '

# ignore duplicate history entries
setopt histignoredups

# keep TONS of history
export HISTSIZE=4096

# look for ey config in project dirs
export EYRC=./.eyrc

# automatically pushd
setopt auto_pushd
export dirstacksize=5

# awesome cd movements from zshkit
setopt AUTOCD
setopt AUTOPUSHD PUSHDMINUS PUSHDSILENT PUSHDTOHOME
setopt cdablevars

# Enable extended globbing
setopt EXTENDED_GLOB

# Configuration
for config_file (~/.zsh/*.zsh) source $config_file

PATH=/usr/local/bin:$PATH

### Added by the Heroku Toolbelt
export PATH="/usr/local/heroku/bin:$PATH"

# Add npm utils (coffeescript)
export PATH="/usr/local/share/npm/bin:$PATH"

# Find bin stubs in /bin and bin/stubs using T. Pope's git safe method
export PATH=".git/safe/../../bin:.git/safe/../../bin/stubs:$PATH"

export PATH="$PATH:/Users/jsteiner/Desktop/AWS-ElasticBeanstalk-CLI-2.3/eb/macosx/python2.7/"

if which rbenv > /dev/null; then eval "$(rbenv init -)"; fi

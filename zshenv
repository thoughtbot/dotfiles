PATH=/usr/local/bin:$PATH

### Added by the Heroku Toolbelt
export PATH="/usr/local/heroku/bin:$PATH"

# Add npm utils (coffeescript)
export PATH="/usr/local/share/npm/bin:$PATH"

export PATH="$PATH:/Users/jsteiner/Desktop/AWS-ElasticBeanstalk-CLI-2.3/eb/macosx/python2.7/"

# Add Amazon AWS tools
export AWS_CREDENTIAL_FILE="/usr/local/aws/aws-credentials"
export AWS_RDS_HOME="/usr/local/aws/rds"
export JAVA_HOME=`/usr/libexec/java_home -v 1.6`
export PATH="$PATH:${AWS_RDS_HOME}/bin"

export RBENV_ROOT=$HOME/.rbenv
[[ -d $RBENV_ROOT/shims ]] && eval "$(rbenv init -)"

# Find bin stubs in /bin and bin/stubs using T. Pope's git safe method
export PATH=".git/safe/../../bin:.git/safe/../../bin/stubs:$HOME/.bin:$PATH"

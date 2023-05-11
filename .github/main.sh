#!/bin/bash

gem install slack-ruby-client

echo "We're in a script"
echo $@
env
ruby slack-pr-notification.rb

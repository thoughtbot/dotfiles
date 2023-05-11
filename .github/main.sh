#!/bin/bash

gem install slack-ruby-client

echo "We're in a script"
echo $@
env
ruby ./.github/slack-pr-notification.rb

#!/bin/bash

gem install slack-ruby-client

echo "We're in a script"
env
ruby ./.github/slack-pr-notification.rb

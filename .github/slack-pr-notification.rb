#!/usr/bin/env ruby

# gem install slack-ruby-client

require 'slack-ruby-client'
require 'json'

# load json object from file
json = JSON.parse(File.read(ENV['GITHUB_EVENT_PATH']))
puts json

Slack.configure do |config|
  config.token = ENV['SLACK_GITHUB_BOT_TOKEN']
end

pr_author_email = ENV['pull_request_author_email']

client = Slack::Web::Client.new()
resp = client.users_lookupByEmail(email: pr_author_email)

if resp.ok?
  user = resp.user
  resp = client.users_conversations(types: 'im')
  channel = resp.channels.select { |c| c.user == user.id }.first
  messages = client.conversations_history(channel: channel.id).messages

  message = messages.select { |m| m.text == 'Hello World2' }.first
  puts message

  if message
    client.chat_postMessage(channel: user.id, text: 'An update', as_user: true, thread_ts: message.ts)
  else
    client.chat_postMessage(channel: user.id, text: 'Hello World2', as_user: true)
  end
end

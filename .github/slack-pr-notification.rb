#!/usr/bin/env ruby

require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'slack-ruby-client'
end

require 'slack-ruby-client'

# configure slack with the right
Slack.configure do |config|
  config.token = ENV['SLACK_GITHUB_BOT_TOKEN']
end
client = Slack::Web::Client.new()

def find_slack_user_from_email(client, email)
  begin
    resp = client.users_lookupByEmail(email: email)
    if resp.ok?
      resp.user
    end
  rescue => e
    puts e
  end
  nil
end

slack_user = find_slack_user_from_email(client, ENV['pull_request_author_email'])
if !slack_user
  puts "No slack user found for #{ENV['pull_request_author_email']}"
  exit 0
end

# load json object from file
event = JSON.parse(File.read(ENV['GITHUB_EVENT_PATH']))
puts event

action = event['action']
if action == 'edited' and action['changes']['title']
  old_title = action['changes']['title'][]

  # find the conversation with the bot
  resp = client.users_conversations(types: 'im')
  channel = resp.channels.select { |c| c.user == user.id }.first
  if channel != nil
    # try to find if there is already an existing message
    messages = client.conversations_history(channel: channel.id).messages
    message = messages.select { |m| m.text == old_title }.first
    if message
      client.chat_postMessage(channel: user.id, text: 'An update', as_user: true, thread_ts: message.ts)
    else
      client.chat_postMessage(channel: user.id, text: event['pull_request']['title'], as_user: true)
    end
  else
    client.chat_postMessage(channel: user.id, text: event['pull_request']['title'], as_user: true)
  end
end

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

# find the slack user from the email
def find_slack_user_from_email(client, email)
  begin
    resp = client.users_lookupByEmail(email: email)
    if resp.ok?
      return resp.user
    end
  rescue => e
    puts e
  end
  nil
end

# function that creates the slack message
def create_pr_message(pull_request)
  [{
     "type": "section",
     "text": {
       "type": "mrkdwn",
       "text": "Hey there 👋. You created the :github: PR *#{pull_request['title']}*.\nYou can keep track of its status here and new comments will get added as thread messages"
     }
   }]
end


slack_user = find_slack_user_from_email(client, ENV['pull_request_author_email'])
if !slack_user
  puts "No slack user found for #{ENV['pull_request_author_email']}"
  exit 0
end

# load json object from file
event = JSON.parse(File.read(ENV['GITHUB_EVENT_PATH']))
# puts event

action = event['action']
if action == 'edited' and event['changes']['title']
  old_title = event['changes']['title']['from']

  # find the conversation with the bot
  resp = client.users_conversations(types: 'im')
  channel = resp.channels.select { |c| c.user == slack_user.id }.first
  if channel != nil
    # try to find if there is already an existing message
    messages = client.conversations_history(channel: channel.id).messages
    message = messages.select { |m| m.text == old_title }.first
    if message
      client.chat_postMessage(channel: slack_user.id, text: 'An update', as_user: true, thread_ts: message.ts)
    else
      client.chat_postMessage(channel: slack_user.id, blocks: create_pr_message(event['pull_request']), as_user: true, metadata: JSON.dump({
        "event_type": "task_created",
        "event_payload": {
          "id": "TK-2132",
        }}))
    end
  else
    client.chat_postMessage(channel: slack_user.id, blocks: create_pr_message(event['pull_request']), as_user: true)
  end
end

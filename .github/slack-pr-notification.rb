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

slack_user = find_slack_user_from_email(client, ENV['pull_request_author_email'])
if !slack_user
  puts "No slack user found for #{ENV['pull_request_author_email']}"
  exit 0
end

# function that creates the slack message
def create_pr_message(pull_request)
  [{
     "type": "section",
     "text": {
       "type": "mrkdwn",
       "text": "Hey there 👋. You created the :github: <#{pull_request['html_url']}|PR ##{pull_request['number']}> *#{pull_request['title']}*.\nYou can keep track of its status here and new comments will get added as thread messages"
     }
   }]
end

def find_existing_pr_message(client, slack_user, pr_id)
  resp = client.users_conversations(types: 'im')
  channel = resp.channels.select { |c| c.user == slack_user.id }.first
  if channel != nil
    # try to find if there is already an existing message
    messages = client.conversations_history(channel: channel.id).messages
    message = messages.select { |m| m.metadata && m.metadata['event_type'] == "pr_created_#{pr_id}" }.first
    return [channel, message]
  end
  nil
end

# load json object from file
event = JSON.parse(File.read(ENV['GITHUB_EVENT_PATH']))
pull_request = event['pull_request']
pr_id = pull_request['id']
action = event['action']
title_updated = false

if action == 'edited' || action == 'closed'
  title_updated = action == 'edited' && event['changes']['title']
  channel, message = find_existing_pr_message(client, slack_user, pr_id)
end

if action == 'opened' || title_updated
  if message && channel
    client.chat_update(channel: channel.id, ts: message['ts'], blocks: create_pr_message(pull_request), as_user: true)
  else
    client.chat_postMessage(channel: slack_user.id, blocks: create_pr_message(pull_request), as_user: true, metadata: JSON.dump({
      "event_type": "pr_created_#{pr_id}",
      "event_payload": {
        "pr_id": "#{pr_id}",
      }}))
  end
end

if action == 'closed'
  merged = event['pull_request']['merged']
  client.reactions_add(channel: channel.id, timestamp: message['ts'], name: merged ? 'merged' : 'pr-closed')
end
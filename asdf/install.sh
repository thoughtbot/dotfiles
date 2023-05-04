asdf plugin add nodejs
asdf plugin add ruby
asdf plugin add postgres
asdf plugin add elixir
asdf plugin add rabbitmq
asdf plugin add redis
asdf plugin add mongodb

asdf install nodejs latest
asdf install ruby latest
asdf install postgres latest
asdf install elixir latest
asdf install rabbitmq latest
asdf install redis latest
asdf install mongodb latest

asdf global nodejs $(asdf latest nodejs)
asdf global ruby $(asdf latest ruby)
asdf global postgres $(asdf latest postgres)
asdf global elixir $(asdf latest elixir)
asdf global rabbitmq $(asdf latest rabbitmq)
asdf global redis $(asdf latest redis)
asdf global mongodb $(asdf latest mongodb)

sudo cp ./asdf/org.postgresql.postgres.plist /Library/LaunchDaemons/
sudo cp ./asdf/org.redis.redis.plist /Library/LaunchDaemons/

sudo chown root:wheel /Library/LaunchDaemons/org.redis.redis.plist
sudo chmod 644 /Library/LaunchDaemons/org.redis.redis.plist

sudo chown root:wheel /Library/LaunchDaemons/org.postgresql.postgres.plist
sudo chmod 644 /Library/LaunchDaemons/org.postgresql.postgres.plist

sudo launchctl load /Library/LaunchDaemons/org.postgresql.postgres.plist
sudo launchctl load /Library/LaunchDaemons/org.redis.redis.plist


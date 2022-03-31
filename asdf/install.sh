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

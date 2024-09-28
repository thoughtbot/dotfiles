asdf plugin add nodejs
asdf plugin add ruby
asdf plugin add elixir

asdf install nodejs latest
asdf install ruby latest
asdf install elixir latest

asdf global nodejs $(asdf latest nodejs)
asdf global ruby $(asdf latest ruby)
asdf global elixir $(asdf latest elixir)

#!/bin/bash

ln -s $PWD ~/.dotfiles

brew install rcm
env RCRC=$PWD/rcrc rcup

brew bundle --file ./macos/Brewfile
bash ./macos/install.sh

bash ./vscode/install.sh
touch ~/Library/Application\ Support/Code/User/settings.json
cp -f ./vscode/settings.json ~/Library/Application\ Support/Code/User/settings.json

#!/bin/bash

brew install rcm
env RCRC=$PWD/rcrc rcup
rcup

brew bundle --file ./macos/Brewfile
bash ./macos/install.sh

bash ./vscode/install.sh
touch ~/Library/Application\ Support/Code/User/settings.json
cp -f ./vscode/settings.json ~/Library/Application\ Support/Code/User/settings.json

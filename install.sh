#!/bin/bash

bash ./macos/install.sh
bash ./asdf/install.sh
bash ./puma/install.sh

ln -s $PWD ~/.dotfiles
env RCRC=$PWD/rcrc rcup

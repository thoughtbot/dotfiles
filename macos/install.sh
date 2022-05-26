#!/usr/bin/env bash

brew bundle --file ./macos/Brewfile
bash ./macos/setup.sh
bash ./macos/gpg.sh

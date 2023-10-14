#!/bin/bash

pkglist=(
    adamwalzer.scss-lint
    aki77.rails-routes
    alexkrechik.cucumberautocomplete
    bradlc.vscode-tailwindcss
    bung87.rails
    bung87.vscode-gemfile
    ckolkman.vscode-postgres
    dbaeumer.vscode-eslint
    donjayamanne.githistory
    DotJoshJohnson.xml
    GitHub.github-vscode-theme
    eamodio.gitlens
    eg2.vscode-npm-script
    GitHub.copilot
    GrapeCity.gc-excelviewer
    JakeBecker.elixir-ls
    mechatroner.rainbow-csv
    ms-azuretools.vscode-docker
    ms-python.python
    ms-python.vscode-pylance
    ms-toolsai.jupyter
    ms-toolsai.jupyter-keymap
    ms-toolsai.jupyter-renderers
    ms-vscode-remote.remote-containers
    msaraiva.surface
    phoenixframework.phoenix
    redhat.vscode-commons
    redhat.vscode-yaml
    shanehofstetter.rails-i18n
    sianglim.slim
    tomoki1207.pdf
    wingrunr21.vscode-ruby
    yzhang.markdown-all-in-one
    sorbet.sorbet-vscode-extension
    esbenp.prettier-vscode
    aaron-bond.better-comments
    streetsidesoftware.code-spell-checker
    Shopify.ruby-lsp
)

for i in ${pkglist[@]}; do
  code --install-extension $i
done

touch ~/Library/Application\ Support/Cursor/User/settings.json
cp -f cursor/settings.json ~/Library/Application\ Support/Cursor/User/settings.json


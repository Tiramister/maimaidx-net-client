#!/bin/sh
pip3 install -U Commitizen
yarn global add @vue/cli
echo 'export PATH="$(yarn global bin):$PATH"' >> $HOME/.bashrc

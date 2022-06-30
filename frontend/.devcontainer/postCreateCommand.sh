#!/bin/sh
yarn global add @vue/cli
echo 'export PATH="$(yarn global bin):$PATH"' >> $HOME/.bashrc

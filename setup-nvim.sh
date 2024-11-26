#!/bin/zsh
# vim:syntax=bash

mkdir -p /tmp/nvim
cd /tmp/nvim
curl -fsSLO https://github.com/neovim/neovim/releases/download/v0.9.5/nvim-linux64.tar.gz
curl -fsSLO https://github.com/neovim/neovim/releases/download/v0.9.5/nvim-linux64.tar.gz.sha256sum
sha256sum -c nvim-linux64.tar.gz.sha256sum

tar -xzvf nvim-linux64.tar.gz -C /usr/local/ --strip-components=1

rm -rf /tmp/nvim

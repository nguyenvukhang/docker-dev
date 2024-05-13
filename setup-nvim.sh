#!/bin/zsh
# vim:syntax=bash

mkdir -p /tmp/nvim
cd /tmp/nvim
curl -fsSLO https://github.com/neovim/neovim/releases/download/v0.9.5/nvim-linux64.tar.gz
curl -fsSLO https://github.com/neovim/neovim/releases/download/v0.9.5/nvim-linux64.tar.gz.sha256sum
sha256sum -c nvim-linux64.tar.gz.sha256sum

tar -xzvf nvim-linux64.tar.gz
cd nvim-linux64
mkdir -p /usr/local/bin /usr/local/lib /usr/local/share /usr/local/man/man1
mv bin/nvim /usr/local/bin/nvim
mv lib/nvim /usr/local/lib/nvim
mv share/nvim /usr/local/share/nvim
mv man/man1/nvim.1 /usr/local/man/man1/nvim.1

rm -rf /tmp/nvim

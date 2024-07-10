#!/bin/zsh
# vim:syntax=bash

mkdir -p /tmp/node
cd /tmp/node

curl -fsSL https://raw.githubusercontent.com/tj/n/master/bin/n -o n-installer
bash n-installer lts

rm -rf /tmp/node

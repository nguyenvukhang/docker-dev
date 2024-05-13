#!/bin/zsh
# vim:syntax=bash

GO_TARGZ='go1.22.3.linux-amd64.tar.gz'

mkdir -p /tmp/go
cd /tmp/go
curl -fsSLO https://go.dev/dl/$GO_TARGZ
rm -rf /usr/local/go
tar -C /usr/local -xzf $GO_TARGZ

# update path
echo 'export PATH=/usr/local/go/bin:$PATH' >>~/.zshrc

rm -rf /tmp/go

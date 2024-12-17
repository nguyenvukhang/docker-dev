FROM ubuntu:22.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  zsh \
  git \
  curl \
  ca-certificates \
  gcc \
  g++ \
  python3 \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/\* /tmp/\* /var/tmp/\*

RUN useradd --create-home --shell /bin/zsh hq

WORKDIR /tmp
RUN curl -fLO https://github.com/bazelbuild/bazelisk/releases/download/v1.25.0/bazelisk-amd64.deb
RUN dpkg -i bazelisk-amd64.deb

USER hq
WORKDIR /home/hq

RUN touch .zshrc

# https://catalog.ngc.nvidia.com/orgs/nvidia/containers/cuda/tags
ARG TAG=cuda:12.4.1-devel-ubuntu22.04
FROM nvcr.io/nvidia/${TAG}

# to help with openssh setup
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Singapore

RUN apt-get update && apt-get install -y --no-install-recommends \
  sudo zsh vim git curl tar ripgrep fd-find fzf unzip \
  gnupg2 ca-certificates openssh-server less \
  tmux black clang-format \
  && apt-get clean && rm -rf /var/lib/apt/lists/\* /tmp/\* /var/tmp/\*
  # maven openjdk-17-jdk openjdk-17-jre \

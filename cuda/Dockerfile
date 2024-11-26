# https://catalog.ngc.nvidia.com/orgs/nvidia/containers/cuda/tags
ARG TAG=12.5.1-devel-ubuntu22.04
FROM nvcr.io/nvidia/cuda:${TAG}

# to help with openssh setup
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Singapore

RUN apt-get update && apt-get install -y --no-install-recommends \
  gnupg2 ca-certificates openssh-server \
  sudo zsh vim git tmux \
  curl tar unzip less \
  ripgrep fd-find fzf \
  black clang-format \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/\* /tmp/\* /var/tmp/\*

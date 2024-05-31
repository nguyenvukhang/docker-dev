FROM nvcr.io/nvidia/cuda:12.2.2-devel-ubuntu22.04
# https://catalog.ngc.nvidia.com/orgs/nvidia/containers/cuda/tags

# to help with openssh setup
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Singapore

RUN apt-get update && apt-get install -y --no-install-recommends \
  sudo zsh vim git curl tar ripgrep fd-find fzf unzip \
  openjdk-17-jdk openjdk-17-jre \
  gnupg2 ca-certificates openssh-server \
  maven tmux black clang-format \
  && apt-get clean && rm -rf /var/lib/apt/lists/\* /tmp/\* /var/tmp/\*

# FROM nvcr.io/nvidia/cuda:11.4.2-devel-ubuntu20.04
FROM nvcr.io/nvidia/cuda:12.2.2-devel-ubuntu22.04
# https://catalog.ngc.nvidia.com/orgs/nvidia/containers/cuda/tags

# to help with openssh setup
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Singapore

RUN apt-get update && apt-get install -y --no-install-recommends \
  sudo zsh vim git curl tar ripgrep fd-find fzf unzip \
  openjdk-17-jdk openjdk-17-jre \
  gnupg2 ca-certificates openssh-server \
  && apt-get clean && rm -rf /var/lib/apt/lists/\* /tmp/\* /var/tmp/\*

# RUN curl -fsSLO https://github.com/neovim/neovim/releases/download/v0.9.5/nvim-linux64.tar.gz \
#   && curl -fsSLO https://github.com/neovim/neovim/releases/download/v0.9.5/nvim-linux64.tar.gz.sha256sum \
#   && sha256sum -c nvim-linux64.tar.gz.sha256sum

# Download the micromamba (conda) binary. Setup is not done yet.
RUN mkdir -p /tmp/setup \
  && curl -o /tmp/setup/m -fL https://micro.mamba.pm/api/micromamba/linux-64/latest \
  && tar -xvjf /tmp/setup/m bin/micromamba \
  && rm -f /tmp/setup/m

ARG USERNAME=appliedai
ARG PASSWORD=appliedai
COPY setup-userspace.sh setup-nvim.sh setup-go.sh /

# install things while there is still superuser permissions
RUN /setup-nvim.sh
RUN /setup-go.sh

# add new user and give it sudo priviledges (set its shell to zsh)
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN useradd -ms /bin/zsh $USERNAME && echo "$USERNAME:$PASSWORD" | chpasswd && adduser $USERNAME sudo
RUN chown -R $USERNAME /home/$USERNAME

# switch from root to the regular user (loses superuser permissions)
USER $USERNAME
SHELL ["/bin/zsh", "-c"]
# setup user-space stuff
RUN /setup-userspace.sh

# back to root user
USER root

# clear setup files
RUN rm /setup-userspace.sh /setup-nvim.sh /setup-go.sh

# start ssh server
RUN mkdir -p /run/sshd && chmod 0755 /run/sshd
CMD ["/usr/sbin/sshd", "-D"]

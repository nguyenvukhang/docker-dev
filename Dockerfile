ARG TAG=12.6.2-devel-ubuntu22.04
FROM ghcr.io/nguyenvukhang/cuda:${TAG}

ARG USERNAME=appliedai
ARG PASSWORD=appliedai

# Download the micromamba (conda) binary. Setup is not done yet.
RUN mkdir -p /tmp/setup \
  && curl -o /tmp/setup/m -fL https://micro.mamba.pm/api/micromamba/linux-64/latest \
  && tar -xvjf /tmp/setup/m bin/micromamba \
  && rm -f /tmp/setup/m

COPY setup-userspace.sh setup-nvim.sh setup-go.sh setup-docker.sh setup-node.sh /tmp/setup

# install things while there is still superuser permissions
RUN chsh -s /bin/zsh
RUN /tmp/setup/setup-nvim.sh
RUN /tmp/setup/setup-go.sh
RUN /tmp/setup/setup-node.sh
RUN /tmp/setup/setup-docker.sh

# add new user and give it sudo privileges (set its shell to zsh)
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN useradd -ms /bin/zsh $USERNAME && echo "$USERNAME:$PASSWORD" \
  | chpasswd && adduser $USERNAME sudo && adduser $USERNAME docker
RUN chown -R $USERNAME /home/$USERNAME

# switch from root to the regular user (loses superuser permissions)
USER $USERNAME
SHELL ["/bin/zsh", "-c"]
COPY --chown=appliedai .zshrc /home/$USERNAME
# setup user-space stuff
RUN /tmp/setup/setup-userspace.sh

# back to root user
USER root

# clear setup files
RUN rm -rf /tmp/setup
# RUN echo 'export JAVA_HOME=/usr/lib/jvm/java-1.17.0-openjdk-amd64' >>/etc/zsh/zshenv

# start ssh server
RUN mkdir -p /run/sshd && chmod 0755 /run/sshd
CMD ["/usr/sbin/sshd", "-D"]

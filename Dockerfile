ARG TAG=cuda12.4.1-ubuntu22.04
FROM ghcr.io/nguyenvukhang/apt-base-${TAG}

# Download the micromamba (conda) binary. Setup is not done yet.
RUN mkdir -p /tmp/setup \
  && curl -o /tmp/setup/m -fL https://micro.mamba.pm/api/micromamba/linux-64/latest \
  && tar -xvjf /tmp/setup/m bin/micromamba \
  && rm -f /tmp/setup/m

ARG USERNAME=appliedai
ARG PASSWORD=appliedai
COPY setup-userspace.sh setup-nvim.sh setup-go.sh /

# install things while there is still superuser permissions
RUN chsh -s /bin/zsh
RUN /setup-nvim.sh
RUN /setup-go.sh
RUN /setup-docker.sh

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
RUN echo 'export JAVA_HOME=/usr/lib/jvm/java-1.17.0-openjdk-amd64' >>/etc/zsh/zshenv

# start ssh server
RUN mkdir -p /run/sshd && chmod 0755 /run/sshd
CMD ["/usr/sbin/sshd", "-D"]

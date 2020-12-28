# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.154.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# Required for desktop notifications
# See: https://github.com/mviereck/x11docker/wiki/How-to-connect-container-to-DBus-from-host#dbus-user-session-daemon
ENV DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install --no-install-recommends dbus libdbus-1-dev libnotify-bin

# Replace sh with bash
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

# Switch user -----------------------------------------------------------------
USER vscode

# Install Node ----------------------------------------------------------------
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | bash \
  && source $HOME/.nvm/nvm.sh \
  && nvm install 14 \
  && echo 'export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"' >> $HOME/.zshrc \
  && echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm' >> $HOME/.zshrc \
  && npm install -g commitizen


# Configure ZSH Shell ---------------------------------------------------------

# Configure oh-my-zsh
RUN sed -i 's/plugins=(.*)/plugins=(git rust cargo npm)/g' ~/.zshrc
# Configure powerlevel10k
ENV POWERLEVEL9K_TRANSIENT_PROMPT=always
# Install powerlevel10k
RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ~/powerlevel10k \
  && echo 'source ~/powerlevel10k/powerlevel10k.zsh-theme' >>~/.zshrc

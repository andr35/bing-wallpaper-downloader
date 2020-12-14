# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.154.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends your-list-here

# Replace sh with bash
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

# Install Node
USER vscode
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | bash \
  && source $HOME/.nvm/nvm.sh \
  && nvm install 14 \
  && echo 'export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"' >> $HOME/.zshrc \
  && echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm' >> $HOME/.zshrc \
  && npm install -g commitizen

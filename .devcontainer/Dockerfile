# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.224.2/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

RUN rustup toolchain install nightly --allow-downgrade -c rustfmt -c rust-src -c clippy \
 && rustup default nightly
# RUN rustup component add rust-src && rustup component add rust-src --toolchain nightly
# RUN rustup component add rustfmt && rustup component add rustfmt --toolchain nightly
# RUN rustup component add clippy && rustup component add clippy --toolchain nightly
RUN rustup update

FROM gitpod/workspace-full

# Install custom tools, runtimes, etc.
RUN rustup install nightly \
    && rustup update \
    && rustup default nightly \
    && rustup target add wasm32-unknown-unknown --toolchain nightly

# Substrate and rust compiler dependencies.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        cmake \
        build-essential \
        clang \
        libclang-dev \
        lld
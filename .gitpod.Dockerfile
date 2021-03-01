FROM gitpod/workspace-full

# Install custom tools, runtimes, etc.
RUN rustup install $RUST_VERSION \
    && rustup update \
    && rustup default $RUST_VERSION \
    && rustup target add wasm32-unknown-unknown --toolchain $RUST_VERSION

# Substrate and rust compiler dependencies.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        cmake \
        build-essential \
        clang \
        libclang-dev \
        lld
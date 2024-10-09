# ubuntu 22.04, consider using a Rust base image in the future
FROM ubuntu@sha256:3d1556a8a18cf5307b121e0a98e93f1ddf1f3f8e092f1fddfd941254785b95d7


RUN apt update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -qy --no-install-recommends \
        libsdl2-dev \
        curl \
        ca-certificates \
        gcc

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup default stable

COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
FROM rust:1.82.0-bookworm@sha256:33a0ea4769482be860174e1139c457bdcb2a236a988580a28c3a48824cbc17d6

ARG GID
ARG UID

RUN groupadd --gid ${GID} whales
RUN useradd --home /home/docker --gid ${GID} --create-home --shell /bin/bash --uid ${UID} docker

USER docker

RUN cargo install cargo-watch --version 8.5.3
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres --version 0.8.2

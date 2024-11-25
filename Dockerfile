FROM rust:1.82.0-bookworm@sha256:33a0ea4769482be860174e1139c457bdcb2a236a988580a28c3a48824cbc17d6

ARG UID

RUN useradd --home /home/docker --create-home --shell /bin/bash --uid ${UID} docker

USER docker

COPY --chown=docker:docker . /home/docker/crust

RUN mkdir -p /home/docker/crust/target

VOLUME [ "/home/docker/crust/target" ]

RUN cargo install cargo-watch --version 8.5.3
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres --version 0.8.2

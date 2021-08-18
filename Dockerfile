FROM rust:1.52-slim-bullseye

ENV DEBIAN_FRONTEND="noninteractive"

RUN apt update \
    && apt install -y pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /app/src

COPY . /app/

RUN cargo install --path /app

CMD ["/usr/local/cargo/bin/pr_bump_bin"]

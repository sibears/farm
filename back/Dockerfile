FROM rust:1.78-bullseye as builder
RUN apt-get update &&  \
    apt-get install -y openssl libsqlite3-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /srv
COPY . .
RUN --mount=type=cache,target=/var/cache/buildkit \
    CARGO_HOME=/var/cache/buildkit/cargo \
    CARGO_TARGET_DIR=/var/cache/buildkit/target \
    cargo build --release --locked && \
    cp /var/cache/buildkit/target/release/sibears_farm /sibears_farm

FROM rust:1.78-slim-bullseye
RUN apt-get update &&  \
    apt-get install -y openssl libsqlite3-dev libpq-dev &&  \
    rm -rf /var/lib/apt/lists/* 
WORKDIR /srv
COPY --from=builder /sibears_farm ./
COPY ./entrypoint.sh ./entrypoint.sh
COPY ./wait-for-it.sh ./wait-for-it.sh
COPY ./migrations ./migrations
COPY ./Rocket.toml ./Rocket.toml
ENTRYPOINT ["./entrypoint.sh"]

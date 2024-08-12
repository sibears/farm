FROM rust:1.78-bullseye as builder
RUN apt-get update &&  \
    apt-get install -y openssl libsqlite3-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /srv
COPY . .
RUN cargo build --release

FROM rust:1.78-slim-bullseye
LABEL authors="x5113nc3x"
RUN apt-get update &&  \
    apt-get install -y openssl libsqlite3-dev libpq-dev &&  \
    rm -rf /var/lib/apt/lists/* &&  \
    cargo install diesel_cli --no-default-features --features postgres
WORKDIR /srv
COPY --from=builder /srv/target/release/sibears_farm ./
COPY ./entrypoint.sh ./entrypoint.sh
COPY ./wait-for-it.sh ./wait-for-it.sh
COPY ./migrations ./migrations
COPY ./Rocket.toml ./Rocket.toml
ENTRYPOINT ["./entrypoint.sh"]

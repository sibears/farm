FROM blackdex/rust-musl:x86_64-musl AS builder

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp /app/target/x86_64-unknown-linux-musl/release/sibears_farm /sibears_farm


FROM alpine:3.20 AS runner
RUN apk add bash
LABEL authors="sibears"
WORKDIR /srv
COPY --from=builder /sibears_farm ./
COPY --chmod=755 entrypoint.sh ./entrypoint.sh
COPY --chmod=755 ./wait-for-it.sh ./wait-for-it.sh
COPY ./migrations ./migrations
COPY ./Rocket.toml ./Rocket.toml
RUN sed -i 's/\r$//' ./entrypoint.sh ./wait-for-it.sh && \
    chmod +x ./entrypoint.sh ./wait-for-it.sh
ENTRYPOINT ["/bin/sh", "./entrypoint.sh"]
FROM rust:1.90.0-bookworm as build-env

WORKDIR /usr/src/discord-bot-rs

COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
	&& apt-get install --no-install-recommends -y ca-certificates libssl3 \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=build-env /usr/src/discord-bot-rs/target/release/discord-bot-rs /usr/local/bin/discord-bot-rs

CMD ["discord-bot-rs"]
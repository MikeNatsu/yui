FROM rust:1.63

WORKDIR /app
COPY . .

RUN cargo build --release
CMD ["DISCORD_TOKEN=$DISCORD_TOKEN ./target/release/yui"]


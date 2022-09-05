FROM rust:1.63

WORKDIR /app
COPY . .
ENV DISCORD_TOKEN=$DISCORD_TOKEN

RUN cargo build --release
CMD ["./target/release/yui"]


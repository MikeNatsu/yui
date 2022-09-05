FROM rust:1.63

WORKDIR /app
COPY . .

RUN cargo build --release
CMD ["./target/release/yui"]


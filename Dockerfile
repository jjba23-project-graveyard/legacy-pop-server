FROM rust:1.84

WORKDIR /app

COPY Cargo.toml .

RUN cargo build --release

COPY target/release/* .

EXPOSE 50088

CMD ["./main"]

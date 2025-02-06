#
# Build image
#
FROM docker.io/library/rust:1.84 as build

WORKDIR /app

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src

RUN cargo build --target x86_64-unknown-linux-gnu --release

#
# Release image
#
FROM docker.io/library/rust:1.84

COPY --from=build /app/target/x86_64-unknown-linux-gnu/release/pop-server /app/pop-server

EXPOSE 50088

CMD ["./pop-server"]

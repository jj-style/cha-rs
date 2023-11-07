FROM rust:1.73-slim-bullseye as build
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
WORKDIR /app
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
USER 1000:1000
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/cha-rs ./
ENTRYPOINT ["/cha-rs"]
FROM rust:latest
COPY . .
WORKDIR /
RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release
RUN cargo run --example setup_env
EXPOSE 8000
CMD ["./target/release/sutom-api"]
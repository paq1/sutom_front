FROM rust:latest
COPY . .
WORKDIR /
RUN cargo install dioxus-cli
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release
EXPOSE 8080
CMD ["bash","-c","cargo run --example setup_env && dioxus serve"]
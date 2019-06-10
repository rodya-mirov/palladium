FROM rust:1.35

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-web

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo check
RUN cargo test
RUN cargo build --release
RUN cargo web deploy

RUN rm -rf src

COPY src ./src
COPY static ./static

RUN touch src/main.rs

RUN cargo web deploy

RUN ls target && ls target/deploy

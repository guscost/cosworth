# "debug" stage builds and runs the project
FROM clux/muslrust:stable as debug

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --lib build
WORKDIR /build

# preliminary layers cache dependencies
RUN cargo install --force cargo-watch
RUN cargo install diesel_cli --no-default-features --features "postgres"
COPY ./diesel.toml ./Cargo.toml ./Cargo.lock* /build/
RUN cargo build
RUN rm /build/target/x86_64-unknown-linux-musl/debug/deps/libcosworth-*
RUN rm /build/src/lib.rs

# compile and watch project in debug mode
EXPOSE 8080
COPY ./src /build/src
CMD ["cargo", "watch", "-x", "run"]

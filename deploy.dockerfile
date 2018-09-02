# "build" stage builds the project
FROM clux/muslrust:stable as build

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --lib cosworth

# preliminary layers cache dependencies
WORKDIR /cosworth
COPY ./Cargo.toml /cosworth/Cargo.toml
COPY ./Cargo.lock /cosworth/Cargo.lock
RUN cargo build --release
RUN rm /cosworth/src/lib.rs
RUN rm /cosworth/target/x86_64-unknown-linux-musl/release/deps/libcosworth-*

# compile project in release mode
COPY ./src /cosworth/src
RUN cargo build --release --bin sandbox

# "deploy" stage copies the compiled binary into an empty container
FROM scratch as deploy
COPY --from=build /cosworth/target/x86_64-unknown-linux-musl/release/sandbox .
CMD ["./sandbox"]

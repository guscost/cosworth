# "build" stage builds the project
FROM clux/muslrust:stable as build

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --lib build

# preliminary layers cache dependencies
WORKDIR /build
COPY ./Cargo.toml ./Cargo.lock* /build
RUN cargo build --release
RUN rm /build/src/lib.rs
RUN rm /build/target/x86_64-unknown-linux-musl/release/deps/libcosworth-*

# compile project in release mode
COPY ./src /build/src
RUN cargo build --release
RUN ls /build/target/x86_64-unknown-linux-musl/release/

# "deploy" stage copies the compiled binary into an empty container
FROM scratch as deploy
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/cosworth_example .
CMD ["./cosworth_example"]

# "build" stage builds the project
FROM clux/muslrust:stable as build

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --lib build
WORKDIR /build

# preliminary layers cache dependencies
COPY ./diesel.toml ./Cargo.toml ./Cargo.lock* /build/
RUN cargo build --release
RUN rm /build/target/x86_64-unknown-linux-musl/release/deps/libcosworth-*
RUN rm /build/src/lib.rs

# compile project in release mode
COPY ./src /build/src
RUN cargo build --release

# "deploy" stage copies the compiled binary into an empty container
FROM scratch as deploy
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/cosworth_example .
CMD ["./cosworth_example"]

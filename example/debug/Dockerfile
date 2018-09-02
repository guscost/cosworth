# "debug" stage builds and runs the project
FROM clux/muslrust:stable as debug

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --lib build

# preliminary layers cache dependencies
WORKDIR /build
COPY ./Cargo.toml ./Cargo.lock* /build
RUN cargo build
RUN rm /build/src/lib.rs
RUN rm /build/target/x86_64-unknown-linux-musl/debug/deps/libcosworth-*

# compile and run project in debug mode
EXPOSE 8080
COPY ./src /build/src
CMD ["cargo", "run"]

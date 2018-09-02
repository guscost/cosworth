# "debug" stage builds and runs the project
FROM clux/muslrust:stable as debug

# create a new empty shell project
WORKDIR /
RUN USER=root cargo new --lib cosworth

# preliminary layers cache dependencies
WORKDIR /cosworth
COPY ./Cargo.toml /cosworth/Cargo.toml
COPY ./Cargo.lock /cosworth/Cargo.lock
RUN cargo build
RUN rm /cosworth/src/lib.rs
RUN rm /cosworth/target/x86_64-unknown-linux-musl/debug/deps/libcosworth-*

# compile and run project in debug mode
EXPOSE 8080
COPY ./src /cosworth/src
CMD ["cargo", "run", "--bin", "sandbox"]

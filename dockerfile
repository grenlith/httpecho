FROM rust:1.36.0 AS build

#build the application in the rust image
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new echo
WORKDIR /usr/src/echo
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY src ./src
RUN cargo build --release

#create a scratch image and copy our executable to it
FROM scratch
COPY --from=build /usr/src/echo/target/release/echo .

#run!
USER 1000
CMD ["./echo"]
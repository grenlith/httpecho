FROM clux/muslrust AS build

#build the application in the rust image
WORKDIR /usr/src
RUN USER=root cargo new httpecho
WORKDIR /usr/src/httpecho
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

#create a scratch image and copy our executable to it
FROM scratch
COPY --from=build /root/.cargo/bin/httpecho .

#run!
USER 1000
CMD ["/httpecho"]
ARG RUST_VERSION=1.80

# build stage
FROM rust:${RUST_VERSION}-slim as build

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev libssl-dev openssl clang pkg-config; \
    pkg-config openssl; \ 
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build; \
    rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migration ./migration
COPY ./entity ./entity

# rebuild app with project source
RUN rm ./target/debug/deps/towny-api*; \
    cargo build --release

# deploy stage
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/towny-api .

# expose port
EXPOSE 8000

# run the binary
CMD ["/app/towny-api"]
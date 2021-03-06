FROM rustlang/rust:nightly-slim as builder
WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools libpq-dev && \
    rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new kitty-api
# Copying config/build files
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
# Copying modules
COPY data_item data_item
COPY data_item_derive data_item_derive
# Build command
RUN cargo install --locked --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/kitty-api .
# Copying static files/data
COPY data data
COPY static static
COPY templates templates
# Copying config file for prod
COPY Rocket.toml .
USER 1000
CMD ["./kitty-api"]

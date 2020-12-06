FROM rustlang/rust:nightly-slim as builder
WORKDIR /usr/src/app

RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new kitty-api
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/kitty-api .
# Copying static files/data
COPY data static templates .
COPY Rocket.toml .
USER 1000
CMD ["./kitty-api"]

FROM rust:alpine as build

RUN apk add musl-dev \
        libpq-dev \
        tzdata \
        protoc

WORKDIR /build

ADD ./Cargo.toml ./
ADD ./crates/ ./crates
ADD ./.cargo/ ./.cargo

RUN cargo build --package flymodel-cli --bin flymodel --release

FROM alpine

RUN addgroup -S 101 && \
    adduser -S 101 -G 101 && \
    mkdir -p /home/101/bin

COPY --from=build /build/target/release/flymodel /home/101/bin/

RUN chmod +x /home/101/bin/flymodel

USER 101

RUN flymodel --version

CMD ["flymodel", "serve"]

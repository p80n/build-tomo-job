FROM rustlang/rust:nightly-alpine AS build

RUN cargo new builder
WORKDIR /builder
COPY Cargo.toml Cargo.lock /builder/
RUN apk --no-cache add musl-dev openssl-dev && \
    cargo build --release

COPY src /builder/src
RUN cargo build --release && \
    cargo install --target x86_64-unknown-linux-musl --path ./

ENV IMG_SHA256 cc9bf08794353ef57b400d32cd1065765253166b0a09fba360d927cfbd158088
RUN wget -qO img "https://github.com/genuinetools/img/releases/download/v0.5.11/img-linux-amd64"  && \
    echo "${IMG_SHA256}  img" | sha256sum -c - && \
    chmod a+x "img"

##################################################
FROM rust:alpine

WORKDIR /app
COPY --from=build /usr/local/cargo/bin/build-tomo-job .
COPY --from=build /builder/img .


ENTRYPOINT ["/app/build-tomo-job"]

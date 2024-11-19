FROM rust:1.82-bullseye AS build

RUN apt-get update && apt-get install -y \
  clang \
  pkg-config \
  libssl-dev

WORKDIR /src/app

COPY . .

RUN rustup component add rustfmt

RUN cargo fmt -- --check

RUN cargo build --release

RUN cargo test

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
  curl \
  openssl \
  tini \
  ;

RUN useradd svc

COPY --from=build /src/app/target/release/api-rs /

RUN chown -R svc /api-rs

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["/api-rs"]

# syntax=docker/dockerfile:1.3-labs
FROM rust:1.67 as builder

WORKDIR /opt/cmsrs

RUN apt-get update && apt-get install -y libsqlite3-dev

COPY ["Cargo.toml", "Cargo.lock",  "./"]
# Make empty fake project
RUN mkdir src && mkdir handlebar && echo "fn main() {}" > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

# Clobber fake project with real project
COPY src src
COPY handlebar handlebar
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  set -e && \
  touch src/main.rs && \
  cargo install --path .


FROM debian:bullseye-slim

WORKDIR /opt/cmsrs

ENV TZ="America/Los_Angeles"

RUN apt-get update && apt-get install -y libsqlite3-dev

COPY --from=builder /usr/local/cargo/bin/cmsrs /usr/local/bin/cmsrs

ADD public public
ADD handlebar handlebar
ADD Rocket.toml .

CMD ["cmsrs"]

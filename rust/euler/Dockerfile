# FROM rust

# COPY ./target/release/euler /bin/euler

# CMD ["/bin/euler"]

FROM rust AS build
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new rust-euler
WORKDIR /usr/src/rust-euler
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine
COPY --from=build /usr/src/rust-euler /bin/rust-euler
RUN chmod +x /bin/rust-euler
COPY --from=0 /etc/passwd /etc/passwd
USER dockeruser
CMD ["/bin/rust-euler"]
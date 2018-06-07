FROM rust:1.26.2

WORKDIR /usr/src/rust-web

COPY . .

RUN cargo install

CMD ["rust-web"]

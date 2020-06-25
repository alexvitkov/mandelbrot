FROM rust:1-alpine3.12

WORKDIR /usr/src/mandelbrot
COPY Cargo.toml .
COPY src ./src

RUN cargo install --path .

ENTRYPOINT [ "sh" ]

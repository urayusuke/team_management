FROM rust:1.55.0

WORKDIR /api

COPY .. /api

RUN cargo install --path /api

CMD ["cargo", "run"]

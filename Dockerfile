FROM rust:latest

WORKDIR /team_management/api

COPY . .

RUN cargo install --path .

CMD ["cargo", "run"]

# ベースイメージ
FROM rust:1-slim-buster

# 作業ディレクトリを指定
WORKDIR /api

# ホスト側のファイルをコンテナにコピー
COPY . .

# Rustの依存関係をビルド
RUN apt-get update && \
    apt-get install -y default-libmysqlclient-dev && \
    cargo install diesel_cli --no-default-features --features mysql

RUN cargo clean && \
    cargo update && \
    cargo build --release

RUN diesel migration run && \
    diesel migration generate

# Cargo.lockを削除
RUN rm Cargo.lock

# 起動コマンドを指定
CMD ["./target/release/api"]

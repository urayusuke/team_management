# ベースイメージ
FROM rust:1-slim-buster

# 作業ディレクトリを指定
WORKDIR /api

# ホスト側のファイルをコンテナにコピー
COPY . .

# Rustの依存関係をビルド
RUN cargo build --release

# 起動コマンドを指定
CMD ["./target/release/api"]

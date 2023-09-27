FROM rust:latest
WORKDIR /app/

COPY . .

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

RUN useradd dev -u 1000

EXPOSE 4000

CMD ["cargo", "watch", "--why", "--", "echo"]
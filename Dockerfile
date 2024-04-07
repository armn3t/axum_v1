FROM rust:latest

WORKDIR /app/

COPY . .

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres
# RUN cargo install cargo-watch

RUN useradd dev -u 1000

EXPOSE 5005

ENV DOCKER_TARGET_DIR /target

# CMD ["cargo", "watch", "-x", "run"]
CMD ["cargo", "run"]
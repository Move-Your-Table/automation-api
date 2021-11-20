FROM rust:alpine3.14

WORKDIR /app
COPY . .

RUN apk update && \
    apk add musl-dev
RUN cargo install cargo-watch

EXPOSE 3030
CMD ["cargo", "watch", "-x", "run"]

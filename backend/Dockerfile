FROM rust:latest

# Create rust user
RUN groupadd rust && useradd -g rust rust

WORKDIR /app

COPY . .
# Cannot access db, so compile-time checks are disabled
ENV SQLX_OFFLINE true
# https://stackoverflow.com/a/60590697/10508498
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --path .

USER rust

CMD ["backend"]

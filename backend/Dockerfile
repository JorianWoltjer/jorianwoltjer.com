FROM rust:latest

# Create rust user
RUN groupadd rust && useradd -g rust rust

# Setup cron
RUN apt-get update
RUN apt-get -y install cron

COPY crontab /etc/cron.d/revalidate-frontend
RUN chmod 0644 /etc/cron.d/revalidate-frontend
RUN crontab -u rust /etc/cron.d/revalidate-frontend
RUN chmod u+s /usr/sbin/cron

WORKDIR /app

COPY . .
# Cannot access db, so compile-time checks are disabled
ENV SQLX_OFFLINE=true
# Optimization: https://stackoverflow.com/a/60590697/10508498
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --path .

USER rust

CMD ["sh", "-c", "echo \"$INTERNAL_TOKEN\" > /tmp/internal_token && cron && backend"]

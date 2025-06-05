# Create a low-privilege user for the app to use
psql -U ${POSTGRES_USER} <<-END
    CREATE USER app WITH PASSWORD '${DB_PASSWORD}';
    GRANT USAGE, CREATE ON SCHEMA public TO app;
    GRANT CREATE ON DATABASE ${POSTGRES_DB} TO app;
END

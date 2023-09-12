# Create a low-privilege user for the backend to use
psql -U ${POSTGRES_USER} <<-END
    CREATE USER backend WITH PASSWORD '${BACKEND_PASSWORD}';
    GRANT USAGE, CREATE ON SCHEMA public TO backend;
    GRANT CREATE ON DATABASE ${POSTGRES_DB} TO backend;
END

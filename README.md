# jorianwoltjer.com

## Development

Rebuild only one specific container while running:

```Shell
docker-compose up --build --force-recreate --no-deps -d backend
```

Manually revalidate frontend

```Shell
docker exec -it jw-backend curl -X POST http://frontend/api/revalidate -d path=/
```

Run database server for testing

```Shell
docker run --name postgres-dev -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres
```

Run and create tables on local database

```Shell
sqlx database drop && sqlx database create && sqlx migrate run
```

## Resources

* http://www.craigwardman.com/Blogging/BlogEntry/ssg-isr-and-environment-variables-in-next-js-and-docker
* https://github.com/launchbadge/realworld-axum-sqlx
* https://github.com/launchbadge/sqlx/issues/1014

## TODO

* check performence of text search with all posts
* clear cloudflare cache on revalidation endpoint

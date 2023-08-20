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

Run mysql server for testing

```Shell
docker run --name mysql-dev -p 3306:3306 -e MYSQL_ALLOW_EMPTY_PASSWORD=1 -d mysql
```

## Resources

* http://www.craigwardman.com/Blogging/BlogEntry/ssg-isr-and-environment-variables-in-next-js-and-docker
* https://github.com/launchbadge/realworld-axum-sqlx/tree/main

## TODO

1. frontend depends on backend
2. at frontend startup, revalidate all endpoints once (afterward backend will use /api/revalidate)

* OK ACTUAL SOLUTION THIS TIME (PROMISE):
  1. go back to separate docker-compose
  2. Use "moch" server while building docker for NextJS (if `fetch()` fails, use `[]`)
  3. after backend is done building and startup is done, send a POST to NextJS running `/api/revalidate` with localhost check
  4. Now everything is up to date

* change unwrap in backend to ? with error handling to return 500

* combination of revalidate for generated pages that are EDITED/CREATED
  * + FALLBACK for pages that are not generated yet in this build with loading page
  * Rust will send updated/created slug to NextJS which will handle updating index, folder, post

* REDIRECTS table for slug changes

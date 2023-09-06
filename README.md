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
* https://github.com/launchbadge/realworld-axum-sqlx/tree/main
* https://github.com/launchbadge/sqlx/issues/1014

## TODO

* Search using websockets
* Add all fields to post/folder
* Hidden posts using serversideprops
* RSS + sitemap
* Image upload
* Add other static pages

* Security:
  * Set DB password
  * disable CORS
  * run all as www-data instead of root
  * CSP

* Add redirects to invalid pages using nginx
  * `/blog/p` -> `/blog`, same for `f`
  * `/blog/p/ctf/folder` (p instead of f) -> `/blog/f/ctf/folder` if not found on `/p`
    * `/blog/f/ctf/wrong` -> 404

* Have fun with [`__filename`](https://stackoverflow.com/a/3133313/10508498) to dynamically link footer Github to source code

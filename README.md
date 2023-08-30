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

Run and create tables on local database

```Shell
sqlx database drop && sqlx database create && sqlx migrate run
```

## Resources

* http://www.craigwardman.com/Blogging/BlogEntry/ssg-isr-and-environment-variables-in-next-js-and-docker
* https://github.com/launchbadge/realworld-axum-sqlx/tree/main

## TODO

* For admin functionality, have a client-side `authorized` boolean? (defined by cookies set or not) that will fetch admin API endpoints with cookies, and if a page requires admin, redirect to login
  * decide on manual `axum-session` or `axum-login`

* Add redirects to invalid pages
  * `/blog/p` -> `/blog`, same for `f`
  * `/blog/p/ctf/folder` (p instead of f) -> `/blog/f/ctf/folder` if not found on `/p`
    * `/blog/f/ctf/wrong` -> 404

* change unwrap in backend to ? with error handling to return 500

Redirects table:
- columns: slug, destination_id
- on POST UPDATE: change post slug + insert prev into redirects
	- revalidate: prev folder slug and current folder slug (404-handling)
- on FOLDER UPDATE: change folder slug + insert prev into redirects
  - SQL UPDATE: recursively starting-with+'/'-replace() previous slug string for every folder/post globally
    - For each updated record, add to redirects table
  - revalidate: my parent's slug (404-handling)

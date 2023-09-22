# [jorianwoltjer.com](https://jorianwoltjer.com/)

**Personal blog website with Markdown using a [NextJS](https://nextjs.org/) frontend and [Axum](https://docs.rs/axum/latest/axum/) (Rust) backend**

## Setup

A [`.env`](.env.example) file should be created with two strong passwords for the database.

The following environment variables are required to use [BuildKit](https://docs.docker.com/build/buildkit/#getting-started) with docker-compose:

```Shell
export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1
```

Then running the following command will build and start the containers (note that it takes a few minutes to fully start up):

```Shell
docker-compose up -d --build

# View logs temporarely
docker-compose logs --tail=10 --follow
```

Finally, use the [password.sh](scripts/password.sh) script to set a strong password for logging in as an Admin.

See [this gist](https://gist.github.com/JorianWoltjer/3409ef1c7b59c7c5e7b80a294f5564d0) to see how I automatically pull changes from GitHub push webhooks.

## Development

Rebuild only one specific container while running:

```Shell
docker-compose up --build --force-recreate --no-deps -d backend
```

Run database server for testing

```Shell
docker run --name postgres-dev -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres
```

Run and create tables on local database

```Shell
sqlx database drop && sqlx database create && sqlx migrate run
```

Transfer old MySQL data to new PostgreSQL format:

```sql
SELECT id, parent, url as slug, title, description, COALESCE(icon, img) as img, timestamp FROM folders;
SELECT id, title, text, img, href, category FROM projects;
SELECT id, url as slug, title, description, img, markdown, timestamp, parent as folder, points, views, featured, (hidden IS NOT NULL) as hidden FROM posts;
SELECT post as post_id, tag as tag_id FROM post_tags;

-- replace strings in *.sql files:
$ sed "s/\\\\'/''/g" | sed 's/\\"/"/g' | sed "s/\\\\r/'||chr(13)||'/g" | sed "s/\\\\n/'||chr(10)||'/g" | sed 's/\\\\/\\/g' | sed 's/\/img\/blog\///g'
-- *manually remove ` backticks from statements*
-- *manually set featured and hidden to ::bool*
```

#### Scripts

* [revalidate.sh](scripts/revalidate.sh): Revalidate a custom URL on the frontend
* [sign.sh](scripts/sign.sh): Create a HMAC signature for a specific hidden blog post ID

## Resources

* http://www.craigwardman.com/Blogging/BlogEntry/ssg-isr-and-environment-variables-in-next-js-and-docker
* https://github.com/launchbadge/realworld-axum-sqlx
* https://github.com/launchbadge/sqlx/issues/1014

## TODO

* clear cloudflare cache on revalidation endpoint
* test hotswap with docker-compose on VPS
  * https://github.com/docker/compose/issues/1786#issuecomment-579794865

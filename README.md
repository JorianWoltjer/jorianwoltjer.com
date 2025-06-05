# [jorianwoltjer.com](https://jorianwoltjer.com/)

**Personal blog website with Markdown using a plain HTML/CSS/JavaScript frontend and [Axum](https://docs.rs/axum/latest/axum/) (Rust) backend**

## Setup

Clone this repository **including submodules** to make syntax highlighting work.

```bash
git clone --recurse-submodules --shallow-submodules https://github.com/JorianWoltjer/jorianwoltjer.com
```

A [`.env`](.env.example) file should be created with two strong randomly generated passwords for the database.

The following command will build and start the containers:

```bash
docker-compose up -d --build

# View logs temporarely
docker-compose logs --tail=10 --follow
```

While it is running, use the [`manager`](app/src/bin/manager.rs) binary to perform some administrative actions.

```shell
$ docker compose exec -it app manager
Perform simple management tasks on the website

Usage: manager <COMMAND>

Commands:
  theme     Syntax highlighing themes
  render    Update generated HTML for all posts
  password  Set administrator password (will be prompted)
  help      Print this message or the help of the given subcommand(s)
```

The most imporant to start with is `password` to set a new strong password for the `/login` page (default is "secret").  
Any changes to the [render code](app/src/render.rs) should be followed by running the `render` management command to refresh all HTML in the database.

## Development

Run database server for testing

```bash
docker run --name postgres-dev -p 5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres
```

Run and create tables on local database

```bash
sqlx database drop && sqlx database create && sqlx migrate run
```

Create database dump:

```bash
docker exec -it jw-db pg_dump -U postgres > app.dump
```

Restore database dump:

```bash
docker exec -i jw-db psql -U postgres < app.dump
```

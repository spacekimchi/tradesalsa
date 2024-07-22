# Trader Salsa

### Purpose

An online dashboard for a detailed view into NinjaTrader trades

### Rust

The recommended way of [installing Rust](https://www.rust-lang.org/tools/install) is through rustup

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Backend

The backend is built using [Axum](https://github.com/tokio-rs/axum).

I chose Rust and Axum over other languages and frameworks because of how powerful it is even on a cheap machine.

Rust is also great for web programming because it requires that errors and all cases be handled. The only bugs really would be logic ones.

### PostgreSQL

The project is using [PostgreSQL](https://www.postgresql.org/). You can install PostgreSQL for whatever machine you are using.

There is a docker script for starting a PostgreSQL database inside `scripts/init_db.sh`

Use this command to connect to the PostgreSQL docker container.

`psql -h 127.0.0.1 -p 5432 -U postgres`

### sqlx

This project uses [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) to manage the database. The following command will install for only postgres

`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

We can initialize the database using. Make sure there is a `DATABASE_URL` environment variable set in the `.env` file.

`sqlx database setup`

Add a migration using sqlx

`sqlx migrate add <migration_name>`

Run the migrations with

`sqlx migrate run`

Revert the last migration with

`sqlx migrate revert`

## When deploying to server

Remember to get a copy of the `configuration/local.yaml`, `configuration/base.yaml`, and `configuration/production.yaml`.

`base_url` needs to be set in the `configuration/production.yaml`. This can be set to the domain the project will be hosted on.

Create a systemd service to run the application.

The systemd service loads environment variables using a path. Be sure to restrict reading access to this file in order to protect secrets

## Development

For autocompiling on code changes install cargo-watch with: `cargo install cargo-watch`

Then run `cargo watch -x run`

Run with `cargo watch --no-vcs-ignores -x run` to ignore the .gitignore and use the `.ignore_file` instead

## Emailing

The project is set up to email using SMTP servers.

I recommend using [MailHog](https://github.com/mailhog/MailHog) for testing email sending during development.
You are able to view sent mail at [http://localhost:8025](http://localhost:8025)

The default port for mailhog is 1025, and the default email view for the browser is 8025.

Email templates are placed under the `templates/emails` directory.

## Frontend

Instead of using a frontend framework, this project will use SSR to serve HTML, SCSS, and JavaScript.

The project uses [tera](https://github.com/Keats/tera) for templating. Tera uses the `templates/` directory as a base directory

The scss will be compiled when the project starts using the [grass crate](https://github.com/connorskees/grass)

The project is set up to compile any scss files in the `scss/` directory.

This way, we only need to build the project with cargo. No javascript building involved!

## Javascript

Add javascript files to /static/js/ directory and include them in the html wherever they are needed

## Roles

The application is set up to create an initial user with the admin role. The name can be changed by looking at the `migrations/20240721170003_seed_users.sql` file.

## Tests

Run tests with the command `cargo test`

If you want to run a certain test, you can specify the name of the test.
 - Ex: `cargo test authorized_user_creation` will run tests with names that match `authorized_user_creation`
   - Ex: `authorized_user_creation` and `unauthorized_user_creation` both match `authorized_user_creation`

If you want to capture `println!()` statements when running tests, add `-- --nocapture` to the command.
 - Ex: `cargo test -- --nocapture`


## Simple CRUD Application with RUST

### Run and Build

```sh
echo DATABASE_URL=postgres://postgres:postgres@localhost/diesel_demo > .env

cargo install diesel_cli
diesel run migration

env $(cat .env | xargs) cargo run
```

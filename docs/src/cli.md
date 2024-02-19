# Cli

```sh
Usage: flymodel [OPTIONS] <COMMAND>

Commands:
  serve
  migrate
  setup-storage
  upsert
  help           Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  [default: ./flymodel.toml]
  -h, --help             Print help
  -V, --version          Print version
```

## Serve

```sh
Usage: flymodel serve [OPTIONS] --database-url <DATABASE_URL>

Options:
  -p, --port <PORT>                  [default: 9009]
  -b, --bind <BIND>                  [default: localhost]
  -d, --database-url <DATABASE_URL>  [env: DB_URL=postgresql://postgres:postgres@localhost:5432/mlops]
  -c, --config <CONFIG>              [default: ./flymodel.toml]
  -h, --help                         Print help
```

## Migrate

```sh
Usage: flymodel migrate [OPTIONS] <COMMAND>

Commands:
  up
  down
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  [default: ./flymodel.toml]
  -h, --help             Print help
```

### Up

```sh
Usage: flymodel migrate up [OPTIONS] --database-url <DATABASE_URL>

Options:
  -d, --database-url <DATABASE_URL>  [env: DB_URL=postgresql://postgres:postgres@localhost:5432/mlops]
      --test-data <TEST_DATA>        [possible values: basic, multi_region]
      --steps <STEPS>
  -c, --config <CONFIG>              [default: ./flymodel.toml]
  -h, --help                         Print help
```

### Down

```sh
Usage: flymodel migrate down [OPTIONS] --database-url <DATABASE_URL>

Options:
  -d, --database-url <DATABASE_URL>  [env: DB_URL=postgresql://postgres:postgres@localhost:5432/mlops]
      --test-data <TEST_DATA>        [possible values: basic, multi_region]
      --steps <STEPS>
  -c, --config <CONFIG>              [default: ./flymodel.toml]
  -h, --help                         Print help
```

## Setup-Storage

```sh
Usage: flymodel setup-storage [OPTIONS]

Options:
  -c, --config <CONFIG>  [default: ./flymodel.toml]
  -h, --help             Print help
```

## Upsert

```sh
Usage: flymodel upsert [OPTIONS]

Options:
  -c, --config <CONFIG>  [default: ./flymodel.toml]
  -h, --help             Print help
```

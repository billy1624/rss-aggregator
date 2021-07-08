# RSS Aggregator

RSS, but in rust


## How to init/migrate database

## Using docker

```shell
docker run --rm \
    -v "$PWD:/volume" \
    -w /volume \
    -e "DATABASE_URL=postgres://rss-aggregator:rss-aggregator@localhost/rss-aggregator" \
    -it clux/diesel-cli diesel migration run
```

## Using diesel cli directly

Requires rust and cargo 

```shell
cargo install diesel_cli --no-default-features --features postgres
# Time to take a coffee brake ☕
DATABASE_URL="postgres://rss-aggregator:rss-aggregator@localhost/rss-aggregator" diesel migration run
```
# tracing-worker

[tracing_subscriber](https://crates.io/crates/tracing-subscriber) implementation for Cloudflare Workers' [workers.rs](https://crates.io/crates/worker) crate.

This is an independent open-source implementation, not affiliated with Cloudflare.

## Add as a dependency
```
cargo add tracing-worker
```

## Initialize
```rust
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    tracing_worker::init(&env);
    (...)
```

## Tracing records will now be printed to the console:
```
$ wrangler dev
Thu Apr 06 2023 09:24:09 GMT+0000  INFO country_counter: [/], located at: (52.2296, 21.0067), within: Mazovia (event src/lib.rs:10)
Thu Apr 06 2023 09:24:09 GMT+0000 DEBUG libsql_client::workers: Stream opened (event /home/sarna/.cargo/registry/src/github.com-1ecc6299db9ec823/libsql-client-0.22.3/src/workers.rs:78)
```

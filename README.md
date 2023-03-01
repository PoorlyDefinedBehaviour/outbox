# Running

Set up dependencies
```
make dev
```

Start the outbox processor
```
cargo run --bin processor
```

Start a process to produce outbox messages
```
cargo run --bin producer
```
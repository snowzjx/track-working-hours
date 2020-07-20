# track-working-hours

## Installation

```
yum install postgresql
```

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
rustup default nightly
```

```
git clone https://github.com/snowzjx/track-working-hours.git
```

```
cd track-working-hours
```

```
cargo build
```

```
cargo install diesel_cli --no-default-features --features postgres
```

```
echo DATABASE_URL=postgres://user:password@localhost/track_working_hours > .env
```

```
cd data; diesel setup
```

```
cargo run --bin init_data
```

```
cargo run --bin server
```


# track-working-hours

## Installation

```
apt install build-essential postgresql postgresql-contrib libpq-dev
```

```
/usr/lib/postgresql/10/bin/pg_ctl -D /etc/postgresql/10/main/  -l logfile start
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
echo DATABASE_URL=postgres://postgres@localhost/track_working_hours > .env
```

```
cd data
diesel setup
cd ..
```

```
cargo run --bin init_data
```

```
cargo run --bin server
```


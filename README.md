# timestmp

example:

```
cargo build --release
python3 -u scripts/loop.py | target/release/timestmp
```
or
```
python3 -u scripts/loop.py | cargo run
```

example output:
```
# length = 6 @ 2022-07-16 12:43:52.451979924 +02:00 #
hello!
# length = 6 @ 2022-07-16 12:43:57.457578298 +02:00 #
hello!
[...]
```
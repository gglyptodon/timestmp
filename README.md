# timestmp

[![Rust](https://github.com/gglyptodon/timestmp/actions/workflows/rust.yml/badge.svg)](https://github.com/gglyptodon/timestmp/actions/workflows/rust.yml)

Hint: 
For python, set ``export PYTHONUNBUFFERED=1`` 
or  use ``python -u``. 

In general, you can use ``stdbuf -o OPTION`` to adjust the stdout buffer, e.g. use ``stdbuf -oL MY_COMMAND | MY_OTHER_COMMAND `` for line buffering.


Example:
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

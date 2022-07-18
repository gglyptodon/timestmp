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
---
Example (redirect stderr to stdout and add timestamp):

``python3 -m http.server 2>&1| target/release/timestmp
``
```
# length = 60 @ 2022-07-18 10:47:37.147247051 +02:00 #
Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...
# length = 59 @ 2022-07-18 10:47:45.683374044 +02:00 #
127.0.0.1 - - [18/Jul/2022 10:47:45] "GET / HTTP/1.1" 200 -
# length = 66 @ 2022-07-18 10:47:53.072460013 +02:00 #

```
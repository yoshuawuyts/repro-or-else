# repro-or-else
Trying to figure out if the answer suggested in
https://twitter.com/goto_bus_stop/status/996783396906061824 works.

Possible solution for
https://github.com/yoshuawuyts/speedbumps/blob/master/2018-05-16-rust-or-else.md.

## Original proposed code
```rust
let node = self.cache.get(key) // Option<K>
  .ok_or_else(|| self.db.get(key))?; // Result<K, E> can be ?-ed // node is now a K
```

## License
Apache-2.0

# Fourché

[![crate::fourche](
https://img.shields.io/crates/v/fourche?label=crates&style=flat)](
https://crates.io/crates/fourche) [![doc::fourche](
https://docs.rs/fourche/badge.svg)](https://docs.rs/crate/fourche)

A simple task queue named after a lion with forked tail (queue fourché),
works with Redis.

## Repositories

This library is developed mainly on [Codeberg.org](
https://codeberg.org/grauwoelfchen/fourche), but the source code is hosted also
on [sourcehut](https://git.sr.ht/~grauwoelfchen/fourche).

Any patches, merge/pull requests or issues on those repositories are welcomed.

```zsh
# the main branch is "trunk"
% git clone git@codeberg.org:grauwoelfchen/fourche.git
% git --no-pager branch -v
* trunk xxxxxxx XXX
```

## Installation

```zsh
% cargo install fourche
```

## Example

```rust
#[derive(Debug, Deserialize, Serialize)]
struct Job { id: u64 }
```

```rust
// enqueue
let client = redis::Client::open("redis://127.0.0.1:6379/0").unwrap();
let mut conn = client.get_connection().unwrap();

let job = Job { id: 1 }

let mut queue = Queue::new("name", &mut conn);
if let Err(err) = queue.enqueue::<Job>(job) {
    println!("err: {}", err);
}
```

```rust
// dequeue
let client = redis::Client::open("redis://127.0.0.1:6379/0").unwrap();
let mut conn = client.get_connection().unwrap();

let mut queue = Queue::new("name", &mut conn);
loop {
  match queue.dequeue::<Job>() {
    Ok(job) => println!("job: {}", job),
    Err(err) => {
        println!("err: {}", err);
        break;
    },
  }
}
```

## License

```text
Fourche
Copyright 2019-2025 Yasha

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

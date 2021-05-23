# Fourché

[![pipeline](
https://gitlab.com/grauwoelfchen/fourche/badges/trunk/pipeline.svg)](
https://gitlab.com/grauwoelfchen/fourche/commits/trunk) [![coverage](
https://gitlab.com/grauwoelfchen/fourche/badges/trunk/coverage.svg)](
https://gitlab.com/grauwoelfchen/fourche/commits/trunk) [![crate::fourche](
https://img.shields.io/crates/v/fourche?label=crates&style=flat)](
https://crates.io/crates/fourche) [![doc::fourche](
https://docs.rs/fourche/badge.svg)](https://docs.rs/crate/fourche)

A simple task queue named after a lion with forked tail (queue fourché),
works with Redis.

## Repositories

This is mainly developed on [GitLab.com](
https://gitlab.com/grauwoelfchen/fourche), but the source code is hosted also
in several following repositories.

Any merge/pull requests or issues on any repository are welcomed.

* https://gitlab.com/grauwoelfchen/fourche
* https://github.com/grauwoelfchen/fourche
* https://git.sr.ht/~grauwoelfchen/fourche

```zsh
# the main branch is "trunk"
% git clone git@gitlab.com:grauwoelfchen/fourche.git
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
struct Job {id: u64 }
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

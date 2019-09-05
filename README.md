# Fourché

A simple task queue named after a lion with forked tail (queue fourché),
works with Redis.


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

let queue = Queue::new("name", &conn);
if let Err(err) = queue.enqueue::<Job>(job) {
    println!("err: {}", err);
}
```

```rust
// dequeue
let client = redis::Client::open("redis://127.0.0.1:6379/0").unwrap();
let mut conn = client.get_connection().unwrap();

let queue = Queue::new("name", &conn);
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

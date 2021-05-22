use redis::{Commands, Connection, ErrorKind, RedisError, Value};
use serde::{de::DeserializeOwned, Serialize};

pub struct Queue<'c> {
    conn: &'c mut Connection,
    actual_queue: String,
    forked_queue: String,
}

impl<'c> Queue<'c> {
    pub fn new(name: &str, conn: &'c mut Connection) -> Self {
        let actual_queue = name.to_string();
        let forked_queue = format!("{}:{}", name, "forked");
        Self {
            conn,
            actual_queue,
            forked_queue,
        }
    }

    pub fn enqueue<T: Serialize>(&mut self, job: T) -> Result<(), RedisError> {
        let task = serde_json::to_vec(&job).unwrap();
        let queue = self.actual_queue.as_str();
        self.conn.lpush(queue, task)
    }

    pub fn dequeue<T>(&mut self) -> Result<T, RedisError>
    where
        T: DeserializeOwned,
    {
        let actual_queue = &self.actual_queue[..];
        let forked_queue = &self.forked_queue[..];

        match self.conn.brpoplpush(actual_queue, forked_queue, 0) {
            Ok(ref value) => match *value {
                Value::Data(ref v) => {
                    serde_json::from_slice(&*v).map_err(|e| {
                        From::from((
                            ErrorKind::TypeError,
                            "invalid",
                            format!("err: {}", e),
                        ))
                    })
                }
                _ => Err(From::from((ErrorKind::TypeError, "unknown"))),
            },
            Err(e) => Err(e),
        }
    }
}

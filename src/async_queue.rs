use redis::{AsyncCommands, Direction, ErrorKind, RedisError, Value};
use redis::aio::MultiplexedConnection;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

// TODO: use fourche
pub struct AsyncQueue<'c> {
    conn: &'c mut MultiplexedConnection,
    actual_queue: String,
    forked_queue: String,
}

impl<'c> AsyncQueue<'c> {
    pub fn new(name: &str, conn: &'c mut MultiplexedConnection) -> Self {
        let actual_queue = name.to_string();
        let forked_queue = format!("{}:{}", name, "fokerd");
        Self {
            conn,
            actual_queue,
            forked_queue,
        }
    }

    pub async fn enqueue<T: Serialize>(
        &mut self,
        job: T,
    ) -> Result<(), RedisError> {
        let task = serde_json::to_vec(&job).unwrap();
        let queue = self.actual_queue.as_str();
        self.conn.lpush(queue, task).await
    }

    pub async fn dequeue<T>(&mut self) -> Result<T, RedisError>
    where
        T: DeserializeOwned,
    {
        let actual_queue = &self.actual_queue[..];
        let forked_queue = &self.forked_queue[..];

        match self
            .conn
            .blmove(
                actual_queue,
                forked_queue,
                Direction::Right,
                Direction::Left,
                0 as f64,
            )
            .await
        {
            Ok(ref value) => match *value {
                Value::BulkString(ref v) => {
                    serde_json::from_slice(v).map_err(|e| {
                        From::from((
                            ErrorKind::TypeError,
                            "invalid",
                            format!("err: {e}"),
                        ))
                    })
                }
                _ => Err(From::from((ErrorKind::TypeError, "unknown"))),
            },
            Err(err) => Err(err),
        }
    }
}

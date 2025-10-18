use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[cfg(feature = "async")]
use fourche::AsyncQueue;

#[cfg(feature = "async")]
use crate::run_test_async;

#[cfg(feature = "async")]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Job<T: Serialize> {
    id: T,
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_async_enqueue() {
    run_test_async(|mut conn| async move {
        let job = Job::<i64> { id: 1 };

        let name = "test";
        let mut queue = AsyncQueue::new(name, &mut conn);
        if let Err(err) = queue.enqueue::<Job<_>>(job).await {
            eprintln!("err: {}", err);
        }

        let count: i64 = conn.llen(name).await.unwrap();
        assert_eq!(1, count);
    })
    .await;
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_async_dequeue() {
    run_test_async(|mut conn| async move {
        let job = Job::<i64> { id: 1 };

        let name = "test";
        let mut queue = AsyncQueue::new(name, &mut conn);
        if let Err(err) = queue.enqueue::<Job<_>>(job).await {
            eprintln!("err: {}", err);
        }

        let result = queue.dequeue::<Job<i64>>().await;

        assert!(result.is_ok());
        assert_eq!(1, result.ok().unwrap().id);

        let count: i64 = conn.llen(name).await.unwrap();
        assert_eq!(0, count);
    })
    .await;
}

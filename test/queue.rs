use fourche::queue::Queue;
use serde::{Deserialize, Serialize};
use redis::Commands;

use crate::run_test;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Job<T: Serialize> {
    id: T,
}

#[test]
fn test_enqueue() {
    run_test(|conn| {
        let job = Job::<i64> { id: 1 };

        let name = "test";
        let mut queue = Queue::new(name, conn);
        if let Err(err) = queue.enqueue::<Job<_>>(job) {
            eprintln!("err: {}", err);
        }

        let count: i64 = conn.llen(name).unwrap();
        assert_eq!(1, count);
    });
}

#[test]
fn test_dequeue() {
    run_test(|conn| {
        let job = Job::<i64> { id: 1 };

        let name = "test";
        let mut queue = Queue::new(name, conn);
        if let Err(err) = queue.enqueue::<Job<_>>(job) {
            eprintln!("err: {}", err);
        }

        let result = queue.dequeue::<Job<i64>>();

        assert!(result.is_ok());
        assert_eq!(1, result.ok().unwrap().id);

        let count: i64 = conn.llen(name).unwrap();
        assert_eq!(0, count);
    });
}

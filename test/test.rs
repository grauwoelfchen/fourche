use std::panic::{self, AssertUnwindSafe, UnwindSafe};
use std::sync::Mutex;

use redis::Client;

#[cfg(feature = "async")]
use std::future::Future;

#[cfg(feature = "async")]
use std::marker;

mod queue;
mod async_queue;

static LOCK: Mutex<()> = Mutex::new(());

pub fn run_test<T>(test: T)
where
    T: FnOnce(redis::Connection) + UnwindSafe,
{
    let _lock = LOCK.lock();

    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        clean(&mut conn);
        test(conn);
    }));
    assert!(result.is_ok());
}

fn clean(conn: &mut redis::Connection) {
    redis::cmd("FLUSHDB").exec(conn).unwrap()
}

#[cfg(feature = "async")]
pub async fn run_test_async<T, F>(test: T)
where
    T: FnOnce(redis::aio::MultiplexedConnection) -> F + marker::Send,
    F: Future<Output = ()> + marker::Send,
{
    let _lock = LOCK.lock();

    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();

    let result = panic::catch_unwind(AssertUnwindSafe(|| async {
        clean_async(&mut conn).await;
        test(conn).await;
    }));
    assert!(result.is_ok());
}

#[cfg(feature = "async")]
async fn clean_async(conn: &mut redis::aio::MultiplexedConnection) {
    redis::cmd("FLUSHDB").exec_async(conn).await.unwrap()
}

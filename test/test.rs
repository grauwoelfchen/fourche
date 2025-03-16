use std::panic::{self, AssertUnwindSafe, UnwindSafe};
use std::sync::Mutex;

mod queue;

static LOCK: Mutex<()> = Mutex::new(());

pub fn run_test<T>(test: T)
where
    T: FnOnce(&mut redis::Connection) + UnwindSafe,
{
    let _lock = LOCK.lock();

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();

    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        clean(&mut conn);

        test(&mut conn);

        clean(&mut conn);
    }));
    assert!(result.is_ok());
}

fn clean(conn: &mut redis::Connection) {
    redis::cmd("FLUSHDB").exec(conn).unwrap()
}

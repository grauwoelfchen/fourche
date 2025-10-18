extern crate redis;
extern crate serde;

#[cfg(feature = "sync")]
mod queue;

#[cfg(feature = "sync")]
pub use queue::Queue;

#[cfg(feature = "async")]
mod async_queue;

#[cfg(feature = "async")]
pub use async_queue::AsyncQueue;

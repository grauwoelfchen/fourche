extern crate redis;
extern crate serde;

#[cfg(feature = "sync")]
pub mod queue;

#[cfg(feature = "async")]
pub mod async_queue;

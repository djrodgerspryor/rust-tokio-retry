//! This library provides extensible asynchronous retry behaviours
//! for use with the popular [`futures`](https://crates.io/crates/futures) crate
//! and the ecosystem of [`tokio`](https://tokio.rs/) libraries.
//!
//! # Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tokio-retry = "*"
//! ```
//!
//! # Examples
//!
//! ```rust
//! extern crate futures;
//! extern crate tokio_timer;
//! extern crate tokio_retry;
//!
//! use std::time::Duration;
//! use std::default::Default;
//! use futures::future::Future;
//! use tokio_timer::Timer;
//! use tokio_retry::RetryStrategy;
//! use tokio_retry::strategies::ExponentialBackoff;
//!
//! fn action() -> Result<u64, ()> {
//!     // do some real-world stuff here...
//!     Ok(42)
//! }
//!
//! pub fn main() {
//!     let retry_strategy = ExponentialBackoff::from_millis(10)
//!         .limit_delay(Duration::from_millis(1000))
//!         .limit_retries(3)
//!         .jitter();
//!     let retry_future = retry_strategy.run(Timer::default(), action);
//!     let retry_result = retry_future.wait();
//!
//!     assert_eq!(retry_result, Ok(42));
//! }
//! ```

extern crate either;
extern crate futures;
extern crate rand;
extern crate tokio_timer;

mod future;
mod strategy;
/// Assorted retry strategies including fixed interval and exponential back-off.
pub mod strategies;

pub use future::{RetryError, RetryFuture};
pub use strategy::*;

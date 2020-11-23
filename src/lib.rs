//! async-runtime - Traits for writing async runtime agnostic code
//!
//! This crate provides a `Runtime` trait that can be implemented for runtimes
//! providing `spawn` and `timeout` features.
//!
//! This crate enables library authors to write runtime agnostic code by using
//! a `Runtime` trait object provided by this crate and implemented for the
//! specific runtime.
#![warn(missing_docs)]

use std::fmt;
use std::future::Future;
use std::time::Duration;

use futures::future::BoxFuture;

pub mod rt;

/// Error returned by `timeout`
#[derive(Debug, PartialEq, Eq)]
pub enum TimeoutError {
    /// The duration has elapsed and the given future did not return
    /// in time.
    Timeout,
}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Timeout => "Timeout",
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for TimeoutError {}

/// Error returned by `spawn`
#[derive(Debug)]
pub struct SpawnError(Box<dyn std::error::Error>);

impl fmt::Display for SpawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for SpawnError {}

/// Result of the [`timeout`] function
pub type Timeout<'a, T> = BoxFuture<'a, Result<T, TimeoutError>>;

/// Runtime trait
pub trait Runtime: Sync + Send {
    /// Waits until duration has elapsed.
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()>;
    /// Push future onto the executer that will run it to completion.
    fn spawn(&self, future: BoxFuture<'static, ()>);
}

/// Require a Future to complete before the specified duration has elapsed.
///
/// If the future completes before the duration has elapsed, then the
/// completed value is returned. Otherwise, an error is returned and the
/// future is canceled.
pub fn timeout<T: Future + Send + 'static>(
    runtime: &dyn Runtime,
    duration: Duration,
    future: T,
) -> Timeout<T::Output> {
    let sleep_fut = runtime.sleep(duration);
    Box::pin(async move {
        // FIXME Reimplement `select` without depending on tokio or any
        // other runtime. We only need support to poll two futures so
        // no macro should be needed.
        tokio_03::select! {
            output = future => Ok(output),
            _ = sleep_fut => Err(TimeoutError::Timeout),
        }
    })
}

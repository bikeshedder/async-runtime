//! async-runtime - Traits for writing async runtime agnostic code
//!
//! This crate provides a `Runtime` trait that can be implemented for runtimes
//! providing `spawn` and `timeout` features.
//!
//! This crate enables library authors to write runtime agnostic code by using
//! a `Runtime` trait object provided by this crate and implemented for the
//! specific runtime.
#![warn(missing_docs)]

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use std::{fmt, task::Poll};

use pin_project_lite::pin_project;

pub mod rt;

/// An owned dynamically typed [`Future`] for use in cases where you can't
/// statically type your result or need to add some indirection.
/// (This type alias is a 1:1 copy of the one in the `futures` crate.)
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

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

/// Runtime trait
pub trait Runtime: Sync + Send {
    /// Waits until duration has elapsed.
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()>;
    /// Push future onto the executer that will run it to completion.
    fn spawn(&self, future: BoxFuture<'static, ()>);
}

pin_project! {
    /// Result of the [`timeout`] function
    pub struct Timeout<T, F: Future<Output = T>> {
        #[pin]
        future: F,
        sleep: BoxFuture<'static, ()>,
    }
}

impl<T, F: Future<Output = T>> Future for Timeout<T, F> {
    type Output = Result<T, TimeoutError>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Ready(ok) => Poll::Ready(Ok(ok)),
            Poll::Pending => this
                .sleep
                .as_mut()
                .poll(cx)
                .map(|_| Err(TimeoutError::Timeout)),
        }
    }
}

/// Require a Future to complete before the specified duration has elapsed.
///
/// If the future completes before the duration has elapsed, then the
/// completed value is returned. Otherwise, an error is returned and the
/// future is canceled.
pub fn timeout<'a, T, F>(runtime: &dyn Runtime, duration: Duration, future: F) -> Timeout<T, F>
where
    F: Future<Output = T>,
{
    Timeout {
        future,
        sleep: runtime.sleep(duration),
    }
}

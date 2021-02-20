//! This module contains the Tokio 1 implementation of the Runtime trait.
use std::time::Duration;

use futures::future::BoxFuture;

/// Tokio 1 runtime
#[derive(Default, Clone, Copy)]
pub struct Runtime {}

impl crate::Runtime for Runtime {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio_1::time::sleep(duration))
    }
    fn spawn(&self, future: BoxFuture<'static, ()>) {
        tokio_1::spawn(future);
    }
}

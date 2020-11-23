//! This module contains the Tokio 0.3 implementation of the Runtime trait.
use std::time::Duration;

use futures::future::BoxFuture;

/// Tokio 0.3 runtime
#[derive(Default, Clone, Copy)]
pub struct Runtime {}

impl crate::Runtime for Runtime {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio_03::time::sleep(duration))
    }
}

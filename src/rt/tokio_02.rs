//! This module contains the Tokio 0.2 implementation of the Runtime trait.
use std::time::Duration;

use futures::future::BoxFuture;

/// Tokio 0.2 runtime
#[derive(Default, Clone, Copy)]
pub struct Runtime {}

impl crate::Runtime for Runtime {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio_02::time::delay_for(duration))
    }
}

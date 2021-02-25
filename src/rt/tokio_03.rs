//! This module contains the Tokio 0.3 implementation of the Runtime trait.
use std::time::Duration;

use crate::BoxFuture;

/// Tokio 0.3 runtime
#[derive(Default, Clone, Copy)]
pub struct Runtime {}

impl crate::Runtime for Runtime {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio_03::time::sleep(duration))
    }
    fn spawn(&self, future: BoxFuture<'static, ()>) {
        tokio_03::spawn(future);
    }
}

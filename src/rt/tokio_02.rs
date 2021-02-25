//! This module contains the Tokio 0.2 implementation of the Runtime trait.
use std::time::Duration;

use crate::BoxFuture;

/// Tokio 0.2 runtime
#[derive(Default, Clone, Copy)]
pub struct Runtime {}

impl crate::Runtime for Runtime {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(tokio_02::time::delay_for(duration))
    }
    fn spawn(&self, future: BoxFuture<'static, ()>) {
        tokio_02::spawn(future);
    }
}

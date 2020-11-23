//! This module contains the async-std 1 implementation of the Runtime trait.
use std::time::Duration;

use futures::future::BoxFuture;

/// async-std 1 runtime
#[derive(Default, Clone)]
pub struct Runtime {}

impl crate::Runtime for Runtime {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(async_std_1::task::sleep(duration))
    }
}

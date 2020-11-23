//! This module contains the Smol 1 implementation of the Runtime trait.
use std::time::Duration;

use futures::future::BoxFuture;

/// Smol 1 runtime
#[derive(Default, Clone)]
pub struct Runtime {}

impl crate::Runtime for Runtime {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(async move {
            smol_1::Timer::after(duration).await;
        })
    }
}

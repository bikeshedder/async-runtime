async-runtime - Crate for runtime agnostic async code

This crate provides a `Runtime` trait that can be implemented for runtimes
providing `spawn` and `timeout` features.

This crate enables library authors to write runtime agnostic code by using
a `Runtime` trait object provided by this crate and implemented for the
specific runtime.

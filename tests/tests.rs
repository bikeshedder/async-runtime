use std::time::Duration;

use async_runtime::{timeout, Runtime, TimeoutError};

#[allow(dead_code)]
async fn run_tests(rt: &dyn Runtime) -> Result<(), Box<dyn std::error::Error>> {
    // Test normal case
    assert_eq!(
        timeout(rt, Duration::from_millis(10), async {}).await,
        Ok(())
    );
    // Test normal case (timeout=0)
    // This test ensures that the passed future has a higher priority
    // than the timeout.
    assert_eq!(
        timeout(rt, Duration::from_millis(0), async {}).await,
        Ok(())
    );
    // Test timeout case
    let sleep = rt.sleep(Duration::from_millis(1000));
    assert_eq!(
        timeout(rt, Duration::from_millis(0), async { sleep.await }).await,
        Err(TimeoutError::Timeout)
    );
    Ok(())
}

#[cfg(feature = "with-tokio_02")]
#[test]
fn test_tokio_02() {
    let mut tokio_rt = tokio_02::runtime::Runtime::new().unwrap();
    use async_runtime::rt::tokio_02::Runtime;
    let rt = Runtime::default();
    tokio_rt
        .block_on(async move { run_tests(&rt).await })
        .unwrap();
}

#[cfg(feature = "with-tokio_03")]
#[test]
fn test_tokio_03() {
    let tokio_rt = tokio_03::runtime::Runtime::new().unwrap();
    use async_runtime::rt::tokio_03::Runtime;
    let rt = Runtime::default();
    tokio_rt
        .block_on(async move { run_tests(&rt).await })
        .unwrap();
}

#[cfg(feature = "with-tokio_1")]
#[test]
fn test_tokio_1() {
    let tokio_rt = tokio_1::runtime::Runtime::new().unwrap();
    use async_runtime::rt::tokio_1::Runtime;
    let rt = Runtime::default();
    tokio_rt
        .block_on(async move { run_tests(&rt).await })
        .unwrap();
}

#[cfg(feature = "with-async-std_1")]
#[test]
fn test_async_std_1() {
    use async_runtime::rt::async_std_1::Runtime;
    let rt = Runtime::default();
    async_std_1::task::block_on(async move { run_tests(&rt).await }).unwrap();
}

#[cfg(feature = "with-smol_1")]
#[test]
fn test_smol_1() {
    use async_runtime::rt::smol_1::Runtime;
    let rt = Runtime::default();
    smol_1::block_on(async move { run_tests(&rt).await }).unwrap();
}

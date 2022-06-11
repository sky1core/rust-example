use std::thread;
use std::time::Duration;

use futures::executor::block_on;
use futures::future::join_all;
use tokio::time::Instant;

/// async -> non-async -> async
/// single thread async test
pub async fn test1() {
    log::info!("{:?} ------------------------------------------------------------", thread::current().id());

    let now = Instant::now();
    fn_non_async();
    log::info!("end (elapsed: {}s)", now.elapsed().as_secs_f64());
}


fn fn_non_async() {
    log::info!("{:?} start fn_non_async", thread::current().id());

    // block_on use current thread
    let results = block_on(async move {
        join_all([
            fn_async(true),
            fn_async(false),
            fn_async(true),
            fn_async(false),
        ]).await
    });

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(x) => { log::info!("{} success {}", i, x); }
            Err(e) => { log::error!("{} error {}", i, e); }
        }
    }

    log::info!("end fn_non_async1")
}

async fn fn_async(arg1: bool) -> Result<bool, &'static str> {
    log::info!("{:?} start fn_async arg1[{}]", thread::current().id(), arg1);

    tokio::time::sleep(Duration::from_secs(2)).await;

    if arg1 {
        return Ok(true);
    }
    return Err("false");
}




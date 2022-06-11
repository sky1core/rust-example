use std::thread;
use std::time::Duration;

use futures::executor::block_on;
use futures::future::join_all;
use tokio::time::Instant;

/// async -> non-async -> async
/// multi thread async test
pub async fn test2() {
    log::info!("{:?} ------------------------------------------------------------", thread::current().id());

    let now = Instant::now();
    fn_non_async();
    log::info!("end (elapsed: {}s)", now.elapsed().as_secs_f64());
}


fn fn_non_async() {
    log::info!("{:?} start fn_non_async", thread::current().id());


    let results = block_on(async move {
        join_all([
            tokio::task::spawn(async move {
                fn_async(true).await
            }),
            tokio::task::spawn(async move {
                fn_async(false).await
            }),
            tokio::task::spawn(async move {
                fn_async(true).await
            }),
            tokio::task::spawn(async move {
                fn_async(false).await
            }),
        ]).await
    });

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(Ok(x)) => { log::info!("{} success {}", i, x); }
            Ok(Err(e)) => { log::error!("{} error {}", i, e); }
            Err(e) => { log::error!("{} join error {}", i, e); }
        }
    }

    log::info!("end fn_non_async1")
}

async fn fn_async(arg1: bool) -> Result<bool, &'static str> {
    log::info!("{:?} start fn_async arg1[{}]", thread::current().id(), arg1);

    thread::sleep(Duration::from_secs(2));

    if arg1 {
        return Ok(true);
    }
    return Err("false");
}




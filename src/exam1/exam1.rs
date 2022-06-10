use std::thread;
use std::time::Duration;

use futures::executor::block_on;
use futures::future::join_all;
use tokio::time::Instant;

pub async fn exam1() {
    log::info!("exam1 {:?}", thread::current().id());

    let now = Instant::now();
    fn_non_async(true);
    log::info!("end exam1 test1 (elapsed: {}s)", now.elapsed().as_secs_f64());

    let now = Instant::now();
    fn_non_async(false);
    log::info!("end exam1 test2 (elapsed: {}s)", now.elapsed().as_secs_f64());
}

fn fn_non_async(tokio_sleep: bool) {
    log::info!("start fn_non_async tokio_sleep:[{}] {:?}", tokio_sleep, thread::current().id());


    let results = block_on(async move {
        join_all([
            tokio::task::spawn(async move {
                fn_async(true, tokio_sleep).await
            }),
            tokio::task::spawn(async move {
                fn_async(false, tokio_sleep).await
            }),
            tokio::task::spawn(async move {
                fn_async(true, tokio_sleep).await
            }),
            tokio::task::spawn(async move {
                fn_async(false, tokio_sleep).await
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

async fn fn_async(arg1: bool, tokio_sleep: bool) -> Result<bool, &'static str> {
    log::info!("start fn_async arg1:[{}] {:?}", arg1, thread::current().id());

    let sleep_secs = 2;

    if tokio_sleep {
        tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
    } else {
        thread::sleep(Duration::from_secs(sleep_secs));
    }


    if arg1 {
        return Ok(true);
    }
    return Err("false");
}




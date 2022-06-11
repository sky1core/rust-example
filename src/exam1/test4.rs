use std::{panic, thread};
use std::time::Duration;

use futures::executor::block_on;

/// block_on panic을 피해보려고 block_on spawn을 남발하다 의도하지 않은 작동
pub async fn test4() {
    log::info!("{:?} ------------------------------------------------------------", thread::current().id());

    let result = panic::catch_unwind(|| {
        fn_non_async1();
    });

    if result.is_err() {
        log::error!("PANIC! {:?}", result.unwrap_err());
    }
}

fn fn_non_async1() {
    log::info!("{:?} fn_non_async1", thread::current().id());

    block_on(async move {
        tokio::task::spawn(async move {
            fn_async1().await;
        }).await;
    });
}

async fn fn_async1() {
    log::info!("{:?} fn_async1", thread::current().id());
    fn_non_async2();
}

fn fn_non_async2() {
    log::info!("{:?} fn_non_async2", thread::current().id());

    block_on(async move {
        let task = tokio::task::spawn(async move {
            fn_async2().await;
        });

        let timeout = tokio::time::sleep(Duration::from_secs(10));

        tokio::select! {
            _ = task => {
                log::info!("success");
            },
            _ = timeout => {
                log::error!("timeout!");
            }
        }
    });
}

async fn fn_async2() {
    log::info!("{:?} fn_async2", thread::current().id());
}




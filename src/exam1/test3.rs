use std::{panic, thread};

use futures::executor::block_on;

/// recursive block_on on thread
pub async fn test3() {
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
        fn_async1().await;
    });
}

async fn fn_async1() {
    log::info!("{:?} fn_async1", thread::current().id());
    fn_non_async2();
}

fn fn_non_async2() {
    log::info!("{:?} fn_non_async2", thread::current().id());

    // panicked at cannot execute `LocalPool` executor from within another executor
    block_on(async move {
        fn_async2().await;
    });
}

async fn fn_async2() {
    log::info!("{:?} fn_async2", thread::current().id());
}




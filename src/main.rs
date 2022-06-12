mod exam1;
mod exam2;

async fn main_task() {
    exam1::test1().await;
    exam1::test2().await;
    exam1::test3().await;
    exam1::test4().await;

    exam2::test1();
    exam2::test2().await;
}

fn main() {
    tracing_subscriber::fmt::init();

    log::info!("Hello, world!");

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            main_task().await;
        });
}


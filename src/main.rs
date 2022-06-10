mod exam1;

async fn main_task() {
    exam1::exam1().await;
}

fn main() {
    tracing_subscriber::fmt::init();

    log::info!("Hello, world!");

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            main_task().await;
        });
}


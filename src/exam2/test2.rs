use std::thread;

pub async fn test2() {
    log::info!("{:?} ------------------------------------------------------------", thread::current().id());
    let f1 = fn_normal;
    let r1 = f1("f1");
    log::info!("{}", r1);

    let f2 = get_fn(100);
    let r2 = f2("f2");
    log::info!("{}", r2);


    let f2 = get_fn(100);
    executor(Box::new(f1), "f1").await;
    executor(f2, "f2").await;
}

type FType = Box<dyn FnOnce(&str) -> String>;

async fn executor(f: FType, data: &str) {
    let r = f(data);
    log::info!("executor {}", r);
}

fn fn_normal(data: &str) -> String {
    return format!("{}-normal", data);
}

fn get_fn(n: i64) -> FType {
    return Box::new(move |data| {
        return format!("{}-{}", data, n);
    });
}




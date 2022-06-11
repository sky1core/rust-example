use std::thread;

pub fn test2() {
    log::info!("{:?} ------------------------------------------------------------", thread::current().id());
    let f1 = fn_normal;
    let r1 = f1("test");
    log::info!("{}", r1);


    let f2 = get_fn(100);
    let r2 = f2("test");
    log::info!("{}", r2);
}

type FType = Box<dyn FnOnce(&str) -> String>;

fn fn_normal(data: &str) -> String {
    return format!("{}-normal", data);
}

fn get_fn(n: i64) -> FType {
    return Box::new(move |data| {
        return format!("{}-{}", data, n);
    });
}




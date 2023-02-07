//use no_panic::no_panic;

use async_std::task;

//#[no_panic]
fn a() {
    task::block_on(b());
}

async fn b() {
    assert_eq!(c().await, "-");
}

async fn c() -> &'static str {
    return "abc";
}

fn main() {
    a();
}

use no_panic::no_panic;

use async_std::task;

//static mut data : &str = "";

//#[no_panic]
fn a() {
    task::block_on(b());
    //unsafe {
    //println!("{:?}", data);
    //}
}

#[no_panic]
async fn b() {
    //assert_eq!(c().await, "-");
    //println!("{:?}", c().await);
    //unsafe {
    //data = c().await;
    //}
    //return b().await;
    assert_eq!(c().await, "-");
}

//#[no_panic]
async fn c() -> &'static str {
    return "abc";
}

fn main() {
    a();
}

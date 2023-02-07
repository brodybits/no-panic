use no_panic::no_panic;

use std::thread;

async fn a() {
    thread::spawn(b);
}

async fn b() {
    thread::spawn(c);
}

async fn c() {
    panic!("xxx");
}

fn main() {
    a();
}

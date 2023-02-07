use no_panic::no_panic;

async fn a() {
    println!("a");
}

fn main() {
    async_std::task::block_on(a());
}

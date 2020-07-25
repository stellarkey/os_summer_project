mod handler;
mod context;
pub mod timer;   // 为了在main函数中测试，这里设置为pub
pub use context::Context;
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}
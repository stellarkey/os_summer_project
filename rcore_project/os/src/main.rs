//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]

/// 当 panic 发生时会调用该函数
use core::panic::PanicInfo; // 核心库 core，与标准库 std 不同，这个库不需要操作系统的支持
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}                 // 我们暂时将它的实现为一个死循环
}

/// 覆盖 crt0 中的 _start 函数
/// 我们暂时将它的实现为一个死循环
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // extern "C" 描述 _start 函数,表示此函数是一个 C 函数而非 Rust 函数。
    // 由于 _start 是作为 C 语言运行时的入口点，看起来合情合理。
    loop {}
}

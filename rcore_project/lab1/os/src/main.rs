#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

/// 加上 #[no_mangle] 告诉编译器对于此函数禁用编译期间的名称重整（Name Mangling），即确保编译器生成一个名为 _start 的函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello rCore-Tutorial !!!");
    
    interrupt::init();

    unsafe {
        // llvm_asm!("ebreak"::::"volatile");
        for i in 0..1000{
            interrupt::timer::tick();
        }
        
    };
    panic!("end of rust_main")
}
#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;
extern crate alloc;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

/// 加上 #[no_mangle] 告诉编译器对于此函数禁用编译期间的名称重整（Name Mangling），即确保编译器生成一个名为 _start 的函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello rCore-Tutorial !!!");
    
    // 初始化各种模块
    interrupt::init();
    memory::init();

    // 测试
    test_timer();
    test_allocation();
    test_physical_address();
    test_physical_memory_allocator();
    // test_virtual_address();
    unsafe{ llvm_asm!("ebreak"::::"volatile"); }
    
    print_line();
    panic!("end of rust_main")
    
}


pub fn test_timer(){
    print_line();
    unsafe {
        // llvm_asm!("ebreak"::::"volatile");
        for i in 0..300{
            interrupt::timer::tick();
        }
        
    };
}

pub fn test_allocation(){
    print_line();
    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("heap test passed");
}

pub fn test_physical_address(){
    print_line();
    // 注意这里的 KERNEL_END_ADDRESS 为 ref 类型，需要加 *
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
}

pub fn test_physical_memory_allocator(){
    print_line();
    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }
}

pub fn test_virtual_address(){
    print_line();
    // 注意这里的 KERNEL_END_ADDRESS 为 ref 类型，需要加 *
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
}

pub fn print_line(){
    println!("-------------------------------------");
}
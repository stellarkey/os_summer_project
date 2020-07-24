use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus};

pub fn init() {
    unsafe {
        sie::set_stimer(); 
        sstatus::set_sie();  // 开启 sstatus 寄存器中的 SIE 位
        // SIE 位决定中断是否能够打断 supervisor 线程。在这里我们需要允许时钟中断打断 内核态线程，因此置 SIE 位为 1。
    }
    set_next_timeout();
}

static INTERVAL: usize = 100000;

fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

/// 触发时钟中断计数
pub static mut TICKS: usize = 0;

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}
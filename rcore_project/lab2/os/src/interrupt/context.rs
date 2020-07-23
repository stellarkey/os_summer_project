use core::fmt;
use core::mem::zeroed;
use riscv::register::{sstatus::Sstatus};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context{        // 上下文
    pub x: [usize; 32],    // 32个通用寄存器
    pub sstatus: Sstatus,  // 具有许多状态位，控制全局中断使能等
    pub sepc: usize
}

impl Default for Context {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Context")
            .field("registers", &self.x)
            .field("sstatus", &self.sstatus)
            .field("sepc", &self.sepc)
            .finish()
    }
}
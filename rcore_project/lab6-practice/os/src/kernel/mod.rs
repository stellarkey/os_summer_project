//! 为进程提供系统调用等内核功能

mod condvar;
pub mod fs;
pub mod process;
pub mod syscall;

pub use crate::interrupt::*;
pub use crate::process::*;
use alloc::sync::Arc;
pub(self) use fs::*;
pub(self) use process::*;
use spin::Mutex;
pub(self) use syscall::*;

pub use condvar::Condvar;
pub use syscall::syscall_handler;

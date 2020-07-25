use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;


// 堆空间，放在 bss 段
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();
// [`LockedHeap`] 实现了 [`alloc::alloc::GlobalAlloc`] trait

pub fn init() {
    unsafe {
        HEAP.lock().init( HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}

#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("alloc error")
}
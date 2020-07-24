use super::context::Context;
use super::timer;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
     stvec,
};

global_asm!(include_str!("./interrupt.asm"));

pub fn init(){
    unsafe {
        extern "C" {
            fn __interrupt();  // 调用interrupt.asm的接口
        }
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // lab1-practice
        Trap::Exception(Exception::LoadFault) => load_fault(context, scause, stval),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}

fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    // context.sepc += 2;
    context.sepc = 0x0;
}

fn supervisor_timer(_: &Context) {
    timer::tick();
}

fn load_fault(context: &mut Context, scause: Scause, stval: usize){
    if stval == 0x0{
        panic!("SUCCESS!")
    }
    else{
        panic!(
            "LoadFault: {:?}\n{:x?}\nstval: {:x}",
            scause.cause(),
            context,
            stval
        )
    }
}

fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}

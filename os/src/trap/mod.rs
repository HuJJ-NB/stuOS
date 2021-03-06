mod context;

use crate::batch::run_next_app;
use core::arch::global_asm;
use crate::syscall::syscall;
use riscv::register::{
	mtvec::TrapMode,
	scause::{self, Exception, Trap},
	stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
	info!("Get ready fo all Traps from U-MODE to S_MMODE.");
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
	let scause = scause::read();
	let stval = stval::read();

	match scause.cause() {
		Trap::Exception(Exception::UserEnvCall) => {
			debug!("Sys-call from User.");
			trace!("syscall-num: {}", cx.x[17]);
			trace!("top of user_stack: {:x}", cx.x[2]);
			trace!("eage of user_stack: {:x} to {:x}", (cx.x[2] - 1) & (!(0x1000 - 1)), (cx.x[2] + 0x1000 - 1) & (!(0x1000 - 1)));
			cx.sepc += 4;
			cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]], cx.x[2]) as usize;
		}
		Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
			error!("PageFault in application, kernel killed it.");
			run_next_app();
		}
		Trap::Exception(Exception::IllegalInstruction) => {
			error!("IllegalInstruction in application, kernel killed it.");
			run_next_app();
		}
		_ => {
			panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
		}
	}
	cx
}

pub use context::TrapContext;

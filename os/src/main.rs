#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod batch;
mod sync;
mod trap;
mod syscall;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn bss_clear() {
	extern "C" {
		fn sbss();
		fn ebss();
	}
	(sbss as usize..ebss as usize).for_each( |a| {
		unsafe{
			(a as *mut u8).write_volatile(0)
		}
	});
}

fn print_message() {
	extern "C" {
		fn stext();
		fn etext();
		fn srodata();
		fn erodata();
		fn sdata();
		fn edata();
		fn sbss();
		fn ebss();
	}
	printk!("Hello, World!");
	info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
	info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
	info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
	info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
	warn!("It's just a simple os");
	trace!("thanks");	
}

#[no_mangle]
pub fn rust_init_entry() -> ! {
	bss_clear();
	print_message();
	batch::init();
	trap::init();
	batch::run_next_app();
}


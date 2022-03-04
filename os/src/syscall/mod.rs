const SYSCALL_COUNTS: usize = 128;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

mod fs;
mod process;

use crate::sync::UPSafeCell;
use fs::*;
use process::*;
use lazy_static::*;

struct SyscallManager {
	ever_syscalls_count: [usize; SYSCALL_COUNTS],
	all_syscalls_count: usize,
}

impl SyscallManager {
	pub fn counting (&mut self, syscall_id: usize) -> (usize, usize) {
		self.ever_syscalls_count[syscall_id] += 1;
		self.all_syscalls_count += 1;
		(self.ever_syscalls_count[syscall_id], self.all_syscalls_count)
	}
}
lazy_static! {
	static ref SYSCALL_MANAGER: UPSafeCell<SyscallManager> = unsafe { UPSafeCell::new(
		SyscallManager {
			ever_syscalls_count: [0; SYSCALL_COUNTS],
			all_syscalls_count: 0
		}
	)};
}

pub fn syscall(syscall_id: usize, args: [usize; 3], user_sp: usize) -> isize {
	let mut syscall_manager = SYSCALL_MANAGER.exclusive_access();
	let (ever_count, all_count) = syscall_manager.counting(syscall_id);
	drop(syscall_manager);
	trace!("call SYSCALL_{} {} times, in totaly {} syscalls.", syscall_id, ever_count, all_count);
	match syscall_id {
		SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2], user_sp),
		SYSCALL_EXIT => sys_exit(args[0] as i32),
		_ => panic!("Unsupported syscall_id: {}", syscall_id),
	}
}

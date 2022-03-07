const SYSCALL_COUNTS: usize = 128;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GETTASKINFO: usize = 94;
const SYSCALL_GETSYSCALLINFO: usize = 0;

mod fs;
mod process;
mod taskinfo;

use crate::sync::UPSafeCell;
use fs::*;
use process::*;
use taskinfo::*;
use lazy_static::*;

struct SyscallManager {
	count: [isize; SYSCALL_COUNTS],
}

impl SyscallManager {
	pub fn counting(&mut self, syscall_id: usize) -> (isize, isize) {
		if syscall_id > 0 {
			self.count[syscall_id] += 1;
			self.count[0] += 1;
		}
		(self.count[syscall_id], self.count[0])
	}
	pub fn get(&self, syscall_id: usize) -> isize {
		self.count[syscall_id]
	}
}
lazy_static! {
	static ref SYSCALL_MANAGER: UPSafeCell<SyscallManager> = unsafe { UPSafeCell::new(
		SyscallManager {
			count: [0; SYSCALL_COUNTS],
		}
	)};
}

pub fn syscall(syscall_id: usize, args: [usize; 3], user_sp: usize) -> isize {
	if syscall_id > 0 {
		let (ever_count, all_count) = SYSCALL_MANAGER.exclusive_access().counting(syscall_id);
		trace!("call SYSCALL_{} {} times, in totaly {} syscalls.", syscall_id, ever_count, all_count);
	}
	match syscall_id {
		SYSCALL_GETSYSCALLINFO => SYSCALL_MANAGER.exclusive_access().get(args[0]),
		SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2], user_sp),
		SYSCALL_EXIT => sys_exit(args[0] as i32),
		SYSCALL_GETTASKINFO => sys_get_taskinfo(),
		_ => panic!("Unsupported syscall_id: {}", syscall_id),
	}
}

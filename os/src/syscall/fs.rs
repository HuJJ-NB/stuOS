const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize, user_sp: usize) -> isize {
	if ((buf as usize) >= (user_sp - 1) & (!(0x1000 - 1)) && (buf as usize + len - 1) < (user_sp + 0x1000 - 1) & (!(0x1000 - 1))) || ((buf as usize) >= 0x80400000 && (buf as usize + len - 1) < 0x80402000) {
		match fd {        
			FD_STDOUT => {
				let slice = unsafe { core::slice::from_raw_parts(buf, len) };
				let str = core::str::from_utf8(slice).unwrap();
				crate::console::print(format_args!("{}", str));
				len as isize
			}
			_ => {
				error!("Unsupported fd in sys_write!");
				-1 as isize
				//panic!("Unsupported fd in sys_write!");
			}
		}
	}
	else {
		error!("access violation at address 0x{:x}!", buf as usize);
		trace!("user call for print {} Bytes, from {:x}.", len, buf as usize);
		trace!("edge of user stack: {:x} -> {:x}.", (user_sp - 1) & (!(0x1000 - 1)), (user_sp + 0x1000 - 1) & (!(0x1000 - 1)));
		error!("Information finished.");
		-1 as isize
	}
}

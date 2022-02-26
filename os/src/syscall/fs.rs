const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
	
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

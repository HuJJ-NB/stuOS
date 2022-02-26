use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	if let Some(location) = info.location() {
		error!(
			"Panicked at {}:{} {}",
			location.file(),
			location.line(),
			info.message().unwrap()
		);
	}
	else {
		error!("Panicked: {}", info.message().unwrap());
	}
	shutdown();
}

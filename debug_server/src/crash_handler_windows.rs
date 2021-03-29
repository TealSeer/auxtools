use crate::{server_types::BreakpointReason, DEBUG_SERVER};
use auxtools::*;
use winapi::{
	shared::ntdef::LONG,
	um::{errhandlingapi::SetUnhandledExceptionFilter, winnt::EXCEPTION_POINTERS},
	vc::excpt::EXCEPTION_EXECUTE_HANDLER,
};

extern "system" fn exception_filter(_: *mut EXCEPTION_POINTERS) -> LONG {
	unsafe {
		if let Some(dbg) = &mut *DEBUG_SERVER.get() {
			let ctx = *raw_types::funcs::CURRENT_EXECUTION_CONTEXT;

			dbg.handle_breakpoint(
				ctx,
				BreakpointReason::Runtime("native exception".to_owned()),
			);
		}
	}

	return EXCEPTION_EXECUTE_HANDLER;
}

#[init(full)]
fn crash_handler_init(_: &DMContext) -> Result<(), String> {
	unsafe {
		SetUnhandledExceptionFilter(Some(exception_filter));
	}

	Ok(())
}

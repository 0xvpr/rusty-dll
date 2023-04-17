#![cfg(windows)]
#![feature(libc)]

use winapi::um::libloaderapi;
use winapi::um::processthreadsapi;

use winapi::shared::minwindef;
use winapi::shared::minwindef::{
    BOOL,
    DWORD,
    LPVOID,
    HMODULE,
    HINSTANCE
};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]

extern "system" fn DllMain(h_instance: HINSTANCE, dw_reason: DWORD, _lp_reserved: LPVOID) -> BOOL {
    const DLL_PROCESS_DETACH: DWORD = 0;
    const DLL_PROCESS_ATTACH: DWORD = 1;

    match dw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                libloaderapi::DisableThreadLibraryCalls(h_instance);
                processthreadsapi::CreateThread(
                    std::ptr::null_mut(),
                    0,
                    Some(main_thread),
                    h_instance as LPVOID,
                    0,
                    std::ptr::null_mut()
                );
            }
        },
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    minwindef::TRUE
}

unsafe extern "system" fn main_thread(h_instance: *mut winapi::ctypes::c_void) -> u32 {
    libc::printf("%d\n\0".as_ptr() as *const _, 42069);
    libloaderapi::FreeLibraryAndExitThread(h_instance as HMODULE, 0);

    0
}

#![no_std]

use core::panic::PanicInfo;

use winapi::km::wdm::{DbgPrint, IoCreateDevice, DEVICE_OBJECT, DRIVER_OBJECT};
use winapi::shared::ntdef::{NTSTATUS, UNICODE_STRING};
use winapi::shared::ntstatus::STATUS_SUCCESS;

/*#[link(name="ntdll")]
extern "C" {
    pub fn DbgPrint(s: *const u8, ...) -> u32;
}*/

#[unsafe(no_mangle)]
pub extern "system" fn DriverEntry(driver: &mut DRIVER_OBJECT, _: *const UNICODE_STRING) -> NTSTATUS {
    let mut device: *mut DEVICE_OBJECT = core::ptr::null_mut();
    let mut name: UNICODE_STRING = UNICODE_STRING {
        Length: 0,
        MaximumLength: 0,
        Buffer: core::ptr::null_mut()
    };

    unsafe {
        IoCreateDevice(
            driver,
            0,
            &mut name,
            winapi::km::wdm::DEVICE_TYPE::FILE_DEVICE_UNKNOWN,
            0,
            0,
            &mut device
        );
        DbgPrint("ChrisWindowsDriverSec is alive!\n\0".as_ptr());
    }

    driver.DriverUnload = Some(DriverUnload);

    STATUS_SUCCESS /* NT_STATUS SUCCESS */
}

pub extern "system" fn DriverUnload(driver: &mut DRIVER_OBJECT) {
    unsafe {
        DbgPrint("Done!\0".as_ptr());
    }
}

// Need for fix linker error.
#[unsafe(no_mangle)]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8) -> i32 {
    unimplemented!()
}

// Need for fix linker error. Floating point calculations aren't allowed when running the Window

#[unsafe(export_name = "_fltused")]
static _FLTUSED: i32 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
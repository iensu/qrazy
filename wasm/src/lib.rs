use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;

#[allow(dead_code)]
mod logger {
    mod js_logger {
        #[link(wasm_import_module = "logger")]
        extern "C" {
            pub fn info(data: u32);
        }
    }

    pub fn info(data: u32) {
        unsafe { js_logger::info(data) }
    }
}

/// Allocate 1KB of memory.
#[no_mangle]
pub extern "C" fn alloc() -> *mut c_void {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

/// Deallocate 1KB of memory.
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}

/// Generate a QR code based on the string starting at `ptr`.
#[no_mangle]
pub unsafe extern "C" fn qrcode(ptr: *mut u8) {
    let input = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
    let qr = qr_gen::generate(input).unwrap();

    let header_bytes = std::slice::from_raw_parts_mut(ptr, 1024);
    let data = qr.to_vec();

    header_bytes[..data.len()].copy_from_slice(&data);
}

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

/// Allocate memory of `size` bytes.
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

/// Deallocate all memory.
///
/// # Safety
///
/// This will deallocate `size` amount of memory pointed to by `ptr`.
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void, size: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, size);
}

/// Generate a QR code based on the string of `size` bytes starting at `ptr`.
///
/// # Safety
///
/// Reads data at `ptr` as a null terminated C string.
#[no_mangle]
pub unsafe extern "C" fn qrcode(ptr: *mut u8, size: usize) {
    let Ok(input) = CStr::from_ptr(ptr as *const i8).to_str() else {
        std::process::abort();
    };
    let Ok(qr) = qr_gen::generate(input) else {
        std::process::abort();
    };

    let header_bytes = std::slice::from_raw_parts_mut(ptr, size);
    let data = qr.to_vec();

    header_bytes[..data.len()].copy_from_slice(&data);
}

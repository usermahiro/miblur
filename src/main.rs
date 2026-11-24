use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

const PROP_VALUE_MAX: usize = 92;

#[repr(C)]
pub struct PropInfo {
    _p: [u8; 0],
}

#[link(name = "log")]
extern "C" {
    fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

extern "C" {
    fn __system_property_set(name: *const c_char, value: *const c_char) -> c_int;
    fn __system_property_get(name: *const c_char, value: *mut c_char) -> c_int;
    fn __system_property_wait(
        pi: *const PropInfo,
        old_serial: u32,
        new_serial: *mut u32,
        timeout: *const libc::timespec,
    ) -> bool;
}

// (nama, nilai, lock). lock=true -> dipaksa terus; false -> diset sekali saja
const PROPS: &[(&str, &str, bool)] = &[
    ("persist.sys.support_view_smoothcorner", "true", true),
    ("persist.sys.support_window_smoothcorner", "true", true),
    ("persist.sys.background_blur_supported", "true", true),
    ("persist.sys.background_blur_status_default", "false", false),
    ("persist.sys.bionic_material_supported", "true", true),
];

fn log(msg: &str) {
    let tag = CString::new("smoothcornerd").unwrap();
    let text = CString::new(msg).unwrap_or_default();
    unsafe { __android_log_write(4, tag.as_ptr(), text.as_ptr()) };
}

fn get(name: &str) -> String {
    let n = CString::new(name).unwrap();
    let mut buf = [0 as c_char; PROP_VALUE_MAX];
    let len = unsafe { __system_property_get(n.as_ptr(), buf.as_mut_ptr()) };
    if len <= 0 {
        return String::new();
    }
    unsafe { CStr::from_ptr(buf.as_ptr()) }.to_string_lossy().into_owned()
}

fn set(name: &str, value: &str) {
    let n = CString::new(name).unwrap();
    let v = CString::new(value).unwrap();
    let rc = unsafe { __system_property_set(n.as_ptr(), v.as_ptr()) };
    if rc != 0 {
        log(&format!("gagal set {name}={value} (rc={rc}), cek avc"));
    } else {
        log(&format!("set {name}={value}"));
    }
}

fn enforce(locked_only: bool) {
    for (name, want, lock) in PROPS {
        if locked_only && !*lock {
            continue;
        }
        if get(name) != *want {
            set(name, want);
        }
    }
}

fn main() {
    log("start");
    enforce(false);

    // Watchdog: tunggu perubahan property global, cek ulang prop yang dikunci
    let mut serial: u32 = 0;
    loop {
        let changed =
            unsafe { __system_property_wait(ptr::null(), serial, &mut serial, ptr::null()) };
        if changed {
            enforce(true);
        }
    }
}

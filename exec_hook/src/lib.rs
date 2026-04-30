#![allow(unused, clippy::missing_safety_doc)]
use std::{
    ffi::CStr,
    mem,
    os::raw::{c_char, c_int},
};

type ExecveFn = unsafe extern "C" fn(
    pathname: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int;

#[unsafe(export_name = "execve")]
pub extern "C" fn execve(
    pathname: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    // get real execve
    let symbol = std::ffi::CString::new("execve").unwrap();
    let real_execve: ExecveFn = {
        let handle = unsafe { libc::dlsym(libc::RTLD_NEXT, symbol.as_ptr()) };
        if handle.is_null() {
            panic!("failed to find real execve");
        }
        unsafe { mem::transmute(handle) }
    };

    if !pathname.is_null() {
        let path = unsafe { CStr::from_ptr(pathname).to_string_lossy() };

        println!("[HOOK] exec: {}", path);
        if path.contains("xfconf") || path.contains("gsettings") || path.contains("qdbus") {
            if !argv.is_null() {
                let mut i = 0;
                loop {
                    let arg_ptr = unsafe { *argv.add(i) };
                    if arg_ptr.is_null() {
                        break;
                    }

                    let arg = unsafe { CStr::from_ptr(arg_ptr).to_string_lossy() };
                    println!("  arg[{}]: {}", i, arg);
                    i += 1;
                }
            }

            // side effect (optional)
            // let _ = std::process::Command::new(", notify-send")
            //     .arg("Wallpaper command intercepted")
            //     .output();
        }
    }

    unsafe { real_execve(pathname, argv, envp) }
}

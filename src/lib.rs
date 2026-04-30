#![allow(unused, clippy::missing_safety_doc)]
use std::{
    ffi::{CStr, c_char, c_void},
    ops::Not,
    process::Command,
};

type XfconfChannel = usize;

#[unsafe(export_name = "xfconf_init")]
pub extern "C" fn xfconf_init() -> i32 {
    println!("[stub] xfconf_init");
    1
}

#[unsafe(export_name = "xfconf_shutdown")]
pub extern "C" fn xfconf_shutdown() {
    println!("[stub] xfconf_shutdown");
}

#[unsafe(export_name = "xfconf_channel_get")]
pub unsafe extern "C" fn xfconf_channel_get(name: *const c_char) -> XfconfChannel {
    println!("[stub] xfconf_channel_get: {:?}", unsafe {
        as_cstr_or_empty(name)
    });
    0x1
}

#[unsafe(export_name = "xfconf_channel_get_int")]
pub unsafe extern "C" fn xfconf_channel_get_int(
    ch: XfconfChannel,
    prop: *const c_char,
    def: i32,
) -> i32 {
    println!("[stub] get_int: {:?}", unsafe { as_cstr_or_empty(prop) });
    def
}

#[unsafe(export_name = "xfconf_channel_get_string")]
pub unsafe extern "C" fn xfconf_channel_get_string(
    ch: XfconfChannel,
    prop: *const c_char,
    def: *const c_char,
) -> *const c_char {
    println!("[stub] get_string: {:?}", unsafe { as_cstr_or_empty(prop) });
    if def.is_null() { def } else { c"".as_ptr() }
}

#[unsafe(export_name = "xfconf_channel_get_property")]
pub unsafe extern "C" fn xfconf_channel_get_property(
    ch: XfconfChannel,
    prop: *const c_char,
    value: *mut c_void,
) -> i32 {
    println!("[stub] get_property: {:?}", unsafe {
        as_cstr_or_empty(prop)
    });
    0
}

#[unsafe(export_name = "xfconf_channel_set_property")]
pub unsafe extern "C" fn xfconf_channel_set_property(
    ch: XfconfChannel,
    prop: *const c_char,
    value: *const c_void,
) -> i32 {
    println!("[stub] set_property: {:?}", unsafe {
        as_cstr_or_empty(prop)
    });

    0
}

#[unsafe(export_name = "xfconf_channel_set_string")]
pub unsafe extern "C" fn xfconf_channel_set_string(
    ch: XfconfChannel,
    prop: *const c_char,
    value: *const c_char,
) -> i32 {
    println!(
        "[HOOK] set_string: {:?} = {:?}",
        unsafe { as_cstr_or_empty(prop) },
        unsafe { as_cstr_or_empty(value) }
    );

    if unsafe { as_cstr_or_empty(prop) } == c"/backdrop/screen0/monitor0/workspace0/last-image" {
        let Some(cmd) = std::fs::read_to_string("wallpaper-cmd").ok() else {
            println!("[HOOK] didn't find the wallpaper-cmd file");
            return 1;
        };

        println!("[HOOK] setting {:?}", unsafe {
            as_cstr_or_empty(value).to_str().ok()
        });

        if let Ok(output) =
            Command::new(format!("{cmd} {:?}", unsafe { as_cstr_or_empty(value) })).output()
        {
            println!("{}", String::from_utf8(output.stdout).unwrap_or_default());
            println!("{}", String::from_utf8(output.stderr).unwrap_or_default());
        }
    } else {
        println!("[HOOK] not a wallpaper setting thingy")
    }

    0
}

#[unsafe(export_name = "xfconf_channel_set_int")]
pub unsafe extern "C" fn xfconf_channel_set_int(
    ch: XfconfChannel,
    prop: *const c_char,
    value: i32,
) -> i32 {
    println!(
        "[stub] set_int: {:?} = {}",
        unsafe { as_cstr_or_empty(prop) },
        value
    );

    0
}

#[unsafe(export_name = "xfconf_channel_reset_property")]
pub unsafe extern "C" fn xfconf_channel_reset_property(
    ch: XfconfChannel,
    prop: *const c_char,
    recursive: i32,
) -> i32 {
    println!("[stub] reset_property: {:?}", unsafe {
        as_cstr_or_empty(prop)
    });

    0
}

pub unsafe fn as_cstr(s: *const c_char) -> Option<&'static CStr> {
    s.is_null().not().then(|| unsafe { CStr::from_ptr(s) })
}

pub unsafe fn as_cstr_or_empty(s: *const c_char) -> &'static CStr {
    unsafe { as_cstr(s).unwrap_or(c"") }
}

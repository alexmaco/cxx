use alloc::string::String;
use core::slice;
use core::str;

#[export_name = "cxxbridge1$str$valid"]
unsafe extern "C" fn str_valid(ptr: *const u8, len: usize) -> bool {
    let slice = slice::from_raw_parts(ptr, len);
    str::from_utf8(slice).is_ok()
}

#[export_name = "cxxbridge1$str$eq"]
unsafe extern "C" fn str_eq(
    this_ptr: *const u8,
    this_len: usize,
    other_ptr: *const u8,
    other_len: usize,
) -> bool {
    let this = str::from_utf8_unchecked(slice::from_raw_parts(this_ptr, this_len));
    let other = str::from_utf8_unchecked(slice::from_raw_parts(other_ptr, other_len));
    PartialEq::eq(this, other)
}

#[export_name = "cxxbridge1$str$eq_string"]
unsafe extern "C" fn str_eq_string(a_ptr: *const u8, a_len: usize, b: &String) -> bool {
    let a = str::from_utf8_unchecked(slice::from_raw_parts(a_ptr, a_len));
    PartialEq::eq(a, b)
}

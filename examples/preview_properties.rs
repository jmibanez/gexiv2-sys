//! Example of how to access preview image properties
//!
//! To run it, edit in a filename below, and try:
//!   $ cargo run --example preview_properties.rs

extern crate gexiv2_sys as gexiv2;
extern crate libc;

use std::ffi;
use std::ptr;

static FILE_PATH: &str = "/YOUR/FILE/PATH/GOES/HERE.jpg";

fn get_file_metadata(path: &str) -> *mut gexiv2::GExiv2Metadata {
    let mut err: *mut gexiv2::GError = ptr::null_mut();
    let c_str_path = ffi::CString::new(path.as_bytes()).unwrap();
    unsafe {
        let metadata = gexiv2::gexiv2_metadata_new();
        let ok = gexiv2::gexiv2_metadata_open_path(metadata, c_str_path.as_ptr(), &mut err);
        if ok != 1 {
            panic!("Couldn't open image at the given path ({:})", FILE_PATH);
        }
        metadata
    }
}

fn main() {
    unsafe {
        let meta = get_file_metadata(FILE_PATH);
        let all_preview_props = gexiv2::gexiv2_metadata_get_preview_properties(meta);

        if all_preview_props.is_null() {
            panic!("The given media file has no embedded preview images");
        }

        let mut cur_offset = 0;
        while !(*all_preview_props.offset(cur_offset)).is_null() {
            let preview_prop = *all_preview_props.offset(cur_offset);
            let mime_type = ffi::CStr::from_ptr(gexiv2::gexiv2_preview_properties_get_mime_type(
                preview_prop,
            ))
            .to_str();
            println!("{:?}", mime_type);
            cur_offset += 1;
        }
    }
}

// Copyright © 2017-2022 Felix A. Crux <felixc@felixcrux.com> and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Basic tests for gexiv2.

extern crate libc;
extern crate tempfile;

use std::ffi;
use std::fs;
use std::io::Write;
use std::ptr;
use std::slice;

use super::*;

static MINI_JPEG: &[u8] = &[
    255, 216, 255, 219, 00, 43, 00, 03, 02, 02, 02, 02, 02, 03, 02, 02, 02, 03, 03, 03, 03, 04, 06,
    04, 04, 04, 04, 04, 08, 06, 06, 05, 06, 09, 08, 10, 10, 09, 08, 09, 09, 10, 12, 15, 12, 10, 11,
    14, 11, 09, 09, 13, 11, 13, 14, 15, 10, 10, 11, 10, 10, 12, 12, 13, 12, 10, 13, 15, 10, 10, 10,
    255, 201, 00, 11, 08, 00, 01, 00, 01, 01, 01, 11, 00, 255, 204, 00, 06, 00, 10, 10, 05, 255,
    218, 00, 08, 01, 01, 00, 00, 63, 00, 210, 207, 20, 255, 217,
];

unsafe fn make_new_metadata() -> *mut GExiv2Metadata {
    let mut err: *mut GError = ptr::null_mut();
    let metadata = gexiv2_metadata_new();

    let ok = gexiv2_metadata_open_buf(
        metadata,
        MINI_JPEG.as_ptr(),
        MINI_JPEG.len() as libc::c_long,
        &mut err,
    );
    if ok != 1 {
        match ffi::CStr::from_ptr((*err).message).to_str() {
            Ok(v) => panic!("{}", v.to_string()),
            Err(_) => panic!("Unknown error"),
        }
    }

    metadata
}

struct Finalizer<F: Fn()> {
    cleanup: F,
}

impl<F: Fn()> Drop for Finalizer<F> {
    fn drop(&mut self) {
        println!("Drop");
        (self.cleanup)();
    }
}

#[test]
fn initialize() {
    unsafe {
        assert_eq!(gexiv2_initialize(), 1);
    }
}

#[test]
fn get_version() {
    unsafe {
        assert!(gexiv2_get_version() > 0);
    }
}

// Image information.

#[test]
fn metadata_get_supports_exif() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        assert_eq!(gexiv2_metadata_get_supports_exif(meta), 1);
    }
}

#[test]
fn metadata_get_supports_iptc() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        assert_eq!(gexiv2_metadata_get_supports_iptc(meta), 1);
    }
}

#[test]
fn metadata_get_supports_xmp() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        assert_eq!(gexiv2_metadata_get_supports_xmp(meta), 1);
    }
}

#[test]
fn metadata_get_mime_type() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        let result = gexiv2_metadata_get_mime_type(meta);
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "image/jpeg");
    }
}

#[test]
fn metadata_get_pixel_width() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        assert_eq!(gexiv2_metadata_get_pixel_width(meta), 1);
    }
}

#[test]
fn metadata_get_pixel_height() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        assert_eq!(gexiv2_metadata_get_pixel_height(meta), 1);
    }
}

// Helper & convenience getters/setters.

#[test]
fn metadata_set_and_get_metadata_pixel_width() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        gexiv2_metadata_set_metadata_pixel_width(meta, 2, ptr::null_mut());
        assert_eq!(
            gexiv2_metadata_get_metadata_pixel_width(meta, ptr::null_mut()),
            2
        );
    }
}

// Tag information functions.

#[test]
fn metadata_is_exif_tag() {
    unsafe {
        let exif_tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let not_exif_tag = ffi::CString::new("Iptc.Application2.Keywords").unwrap();
        assert_eq!(gexiv2_metadata_is_exif_tag(exif_tag.as_ptr()), 1);
        assert_eq!(gexiv2_metadata_is_exif_tag(not_exif_tag.as_ptr()), 0);
    }
}

#[test]
fn metadata_is_iptc_tag() {
    unsafe {
        let iptc_tag = ffi::CString::new("Iptc.Application2.Keywords").unwrap();
        let not_iptc_tag = ffi::CString::new("Xmp.dc.Description").unwrap();
        assert_eq!(gexiv2_metadata_is_iptc_tag(iptc_tag.as_ptr()), 1);
        assert_eq!(gexiv2_metadata_is_iptc_tag(not_iptc_tag.as_ptr()), 0);
    }
}

#[test]
fn metadata_is_xmp_tag() {
    unsafe {
        let xmp_tag = ffi::CString::new("Xmp.dc.Description").unwrap();
        let not_xmp_tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        assert_eq!(gexiv2_metadata_is_xmp_tag(xmp_tag.as_ptr()), 1);
        assert_eq!(gexiv2_metadata_is_xmp_tag(not_xmp_tag.as_ptr()), 0);
    }
}

#[test]
fn metadata_get_tag_label() {
    unsafe {
        let tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let result = gexiv2_metadata_get_tag_label(tag.as_ptr(), ptr::null_mut());
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "Image Description");
    }
}

#[test]
fn metadata_get_tag_description() {
    unsafe {
        let tag = ffi::CString::new("Exif.Image.FillOrder").unwrap();
        let result = gexiv2_metadata_get_tag_description(tag.as_ptr(), ptr::null_mut());
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "The logical order of bits within a byte");
    }
}

#[test]
fn metadata_get_tag_type() {
    unsafe {
        let tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
        let result = gexiv2_metadata_get_tag_type(tag.as_ptr(), ptr::null_mut());
        let result = ffi::CStr::from_ptr(result).to_str().unwrap();
        assert_eq!(result, "Ascii");
    }
}

// Exif thumbnail getter/setters.

// Disabled on Mac OS X due to https://github.com/felixc/gexiv2-sys/issues/28
#[cfg(not(target_os = "macos"))]
#[test]
fn metadata_get_and_set_exif_thumbnail_from_buffer() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        let mut thumb: *mut u8 = ptr::null_mut();
        let mut thumb_size: libc::c_int = 0;
        assert_eq!(
            gexiv2_metadata_get_exif_thumbnail(meta, &mut thumb, &mut thumb_size),
            0
        );
        gexiv2_metadata_set_exif_thumbnail_from_buffer(
            meta,
            MINI_JPEG.as_ptr(),
            MINI_JPEG.len() as libc::c_int,
        );
        assert_eq!(
            gexiv2_metadata_get_exif_thumbnail(meta, &mut thumb, &mut thumb_size),
            1
        );
        assert_eq!(MINI_JPEG, slice::from_raw_parts(thumb, thumb_size as usize));
    }
}

#[test]
fn metadata_set_exif_thumbnail_from_file() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };

        let tmp_dir = tempfile::tempdir().unwrap();
        let tmp_file_path = tmp_dir.path().join("thumb.jpg");
        let mut thumb_file = fs::File::create(tmp_file_path.clone()).unwrap();
        thumb_file.write_all(MINI_JPEG).unwrap();
        thumb_file.sync_all().unwrap();

        let mut err: *mut GError = ptr::null_mut();
        let c_str_path = ffi::CString::new(tmp_file_path.to_str().unwrap().as_bytes()).unwrap();
        assert_eq!(
            gexiv2_metadata_set_exif_thumbnail_from_file(meta, c_str_path.as_ptr(), &mut err),
            1
        );

        let mut thumb: *mut u8 = ptr::null_mut();
        let mut thumb_size: libc::c_int = 0;
        assert_eq!(
            gexiv2_metadata_get_exif_thumbnail(meta, &mut thumb, &mut thumb_size, &mut err),
            1
        );
        assert_eq!(MINI_JPEG, slice::from_raw_parts(thumb, thumb_size as usize));
    }
}

// Disabled on Mac OS X due to https://github.com/felixc/gexiv2-sys/issues/28
#[cfg(not(target_os = "macos"))]
#[test]
fn metadata_erase_exif_thumbnail() {
    unsafe {
        let meta = make_new_metadata();
        let _finalizer = Finalizer {
            cleanup: || gexiv2_metadata_free(meta),
        };
        let mut thumb: *mut u8 = ptr::null_mut();
        let mut thumb_size: libc::c_int = 0;
        gexiv2_metadata_set_exif_thumbnail_from_buffer(
            meta,
            MINI_JPEG.as_ptr(),
            MINI_JPEG.len() as libc::c_int,
        );
        assert_eq!(
            gexiv2_metadata_get_exif_thumbnail(meta, &mut thumb, &mut thumb_size),
            1
        );
        gexiv2_metadata_erase_exif_thumbnail(meta);
        assert_eq!(
            gexiv2_metadata_get_exif_thumbnail(meta, &mut thumb, &mut thumb_size),
            0
        );
    }
}

// Logging.

#[test]
fn log_get_and_set_level() {
    unsafe {
        assert_eq!(gexiv2_log_get_level(), GExiv2LogLevel::WARN);
        gexiv2_log_set_level(GExiv2LogLevel::INFO);
        assert_eq!(gexiv2_log_get_level(), GExiv2LogLevel::INFO);
    }
}

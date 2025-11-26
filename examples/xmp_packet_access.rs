//! Example of how to access raw XMP packet data via gexiv2.
//!
//! To run it, try:
//!   $ cargo run --features xmp-packet-access --example xmp_packet_access

mod open_buf;

#[cfg(feature = "xmp-packet-access")]
mod example {
    extern crate gexiv2_sys as gexiv2;
    extern crate libc;

    use std::ffi;

    pub fn example() {
        unsafe {
            let metadata = crate::open_buf::make_new_metadata();

            let tag = ffi::CString::new("Xmp.dc.title").unwrap();
            let tag_value = ffi::CString::new("Example").unwrap();
            gexiv2::gexiv2_metadata_set_tag_string(metadata, tag.as_ptr(), tag_value.as_ptr());

            gexiv2::gexiv2_metadata_generate_xmp_packet(
                metadata,
                (gexiv2::GExiv2XmpFormatFlags::OMIT_PACKET_WRAPPER
                    | gexiv2::GExiv2XmpFormatFlags::OMIT_ALL_FORMATTING)
                    .bits(),
                1,
            );
            let packet = gexiv2::gexiv2_metadata_get_xmp_packet(metadata);
            println!("{}", ffi::CStr::from_ptr(packet).to_str().unwrap());

            gexiv2::gexiv2_metadata_free(metadata);
        }
    }
}

#[cfg(not(feature = "xmp-packet-access"))]
mod example {
    pub fn example() {
        println!("You have not enabled the 'xmp_packet_access' feature!");
        println!("Try the --features xmp_packet_access argument to Cargo.");
    }
}

fn main() {
    example::example();
}

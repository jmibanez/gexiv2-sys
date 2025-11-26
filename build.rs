// Copyright © 2015-2022 Felix A. Crux <felixc@felixcrux.com>
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

//! Confirm gexiv2 library exists on the system.

extern crate cc;
extern crate pkg_config;

fn main() {
    let mut cfg = cc::Build::new();
    let gexiv2_lib = match pkg_config::Config::new().atleast_version("0.15").find("gexiv2") {
        Ok(lib) => lib,
        Err(e) => {
            println!(
                "\nThe gexiv2 library (at least version 0.15) was not found by pkg-config/pkgconf on your system.\n\n\
                 Consult the README.md file for suggestions on how to acquire it."
            );
            panic!("{}", e);
        }
    };
    cfg.file("src/glue.cpp");

    for path in gexiv2_lib.include_paths {
        cfg.include(path);
    }

    cfg.compile("gexiv2_sys_glue");
}

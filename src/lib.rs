extern crate xcb;
extern crate cairo;
extern crate cairo_sys;
extern crate libc;

use libc::c_int;
use cairo_sys::{cairo_surface_t};
use xcb::ffi::xproto::{xcb_visualtype_t, xcb_drawable_t};
use xcb::ffi::base::xcb_connection_t;
use xcb::{Visualtype};

#[link(name = "cairo")]
extern {
    pub fn cairo_xcb_surface_create(connection: *mut xcb_connection_t, 
                                    drawable: xcb_drawable_t, 
                                    visual: *mut xcb_visualtype_t, 
                                    width: c_int, 
                                    height: c_int) -> *mut cairo_surface_t;
}

pub fn xcb_surface_create(connection: &xcb::Connection, 
                      window: xcb_drawable_t, 
                      visual: &Visualtype, 
                      width: i32, 
                      height: i32) -> *mut cairo_surface_t {
    unsafe {
        cairo_xcb_surface_create(connection.get_raw_conn(), window, visual.ptr, width, height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

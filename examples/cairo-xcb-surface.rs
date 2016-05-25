extern crate xcb;
extern crate cairo;
extern crate cairo_sys;
extern crate cairo_xcb;
extern crate libc;

use cairo::Surface;
use cairo::prelude::SurfaceExt;
use xcb::ffi::xproto::xcb_visualid_t;
use xcb::Visualtype;
use cairo_xcb::xcb_surface_create;

fn find_visual<'a>(conn: &'a xcb::Connection, visual: xcb_visualid_t) -> Option<Visualtype<'a>> {
    for screen in conn.get_setup().roots() {
        for depth in screen.allowed_depths() {
            for vis in depth.visuals() {
                if visual == vis.visual_id() {
                    return Some(vis)
                }
            }
        }
    }
    None
}

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let values = [
        (xcb::CW_BACK_PIXEL, screen.white_pixel()),
        (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
    ];

    xcb::create_window(&conn,
        xcb::COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        150, 150,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &values);

    xcb::map_window(&conn, window);

    let title = "Basic Window";
    // setting title
    xcb::change_property(&conn, xcb::PROP_MODE_REPLACE as u8, window,
            xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, title.as_bytes());

    // inject cairo here
    let visual = find_visual(&conn, screen.root_visual()).unwrap();
    let csurface = xcb_surface_create(&conn, window, &visual, 150, 150);
    let surface = Surface(csurface);
    let cr = cairo::Context::new(&surface);

    conn.flush();

    // retrieving title
    let cookie = xcb::get_property(&conn, false, window, xcb::ATOM_WM_NAME,
            xcb::ATOM_STRING, 0, 1024);
    if let Ok(reply) = cookie.get_reply() {
        assert_eq!(std::str::from_utf8(reply.value()).unwrap(), title);
    } else {
        panic!("could not retrieve window title!");
    }

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.response_type() & !0x80;
                match r {
                    xcb::EXPOSE => {
                        cr.set_source_rgb(0., 1., 0.);
                        cr.paint();

                        cr.set_source_rgb(1., 0., 0.);
                        cr.move_to(0., 0.);
                        cr.line_to(150., 0.);
                        cr.line_to(150., 150.);
                        cr.close_path();
                        cr.fill();

                        cr.set_source_rgb(0., 0., 1.);
                        cr.set_line_width(20.);
                        cr.move_to(0., 150.);
                        cr.line_to(150., 0.);
                        cr.stroke();

                        surface.flush();
                        // surface_flush(csurface);                        

                        /* We flush the request */
                        conn.flush();

                    },
                    xcb::KEY_PRESS => {
                        let key_press : &xcb::KeyPressEvent = xcb::cast_event(&event);
                        println!("Key '{}' pressed", key_press.detail());
                        break;
                    },
                    _ => {}
                }
            }
        }
    }
}

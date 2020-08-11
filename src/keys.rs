use std::sync::{Arc, RwLock};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;
use std::{thread, ptr, mem};
use x11::{xlib, xinput2};
use x11::xlib::{XInitThreads, BadRequest, Success, XEvent, XNextEvent, XGetEventData, XGenericEventCookie, GenericEvent, XkbKeycodeToKeysym, NoSymbol, XKeysymToString};
use std::ffi::CString;
use x11::xinput2::{XI_LASTEVENT, XI_RawKeyPress, XIEventMask, XIRawEvent};
use std::mem::MaybeUninit;
use crate::ipc::Message;

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) {
    unsafe {
        XInitThreads();
        thread::spawn(move || {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                println!("Failed to open display");
                return;
            }

            let mut xi_opcode = 0;
            let mut query_event = 0;
            let mut query_error = 0;
            let name = CString::new("XInputExtension").unwrap();
            if xlib::XQueryExtension(display, name.as_ptr(), &mut xi_opcode as *mut libc::c_int, &mut query_event as *mut libc::c_int, &mut query_error as *mut libc::c_int)
                == 0 {
                println!("Xinput ext required");
                return;
            }

            let mut major = 2;
            let mut minor = 0;
            let result = xinput2::XIQueryVersion(display, &mut major as *mut libc::c_int, &mut minor as *mut libc::c_int);
            if result as u8 == BadRequest {
                println!("Required at least XI 2.0, found {}.{}", major, minor);
                return;
            } else if result as u8 != Success {
                println!("Internal error");
                return;
            }

            let wnd = xlib::XDefaultRootWindow(display);
            let mut m = xinput2::XIEventMask::default();
            m.deviceid = xinput2::XIAllMasterDevices;
            m.mask_len = ((XI_LASTEVENT) >> 3) + 1; // XIMaskLen is a macro; this is its body
            m.mask = libc::calloc(m.mask_len as usize, mem::size_of::<libc::c_char>()) as *mut u8;
            let offset = m.mask.offset((XI_RawKeyPress >> 3) as isize);
            *offset |= 1 << (XI_RawKeyPress & 7);
            drop(offset);
            xinput2::XISelectEvents(display, wnd, &mut m as *mut XIEventMask, 1);
            xlib::XSync(display, 0);
            libc::free(m.mask as *mut libc::c_void);

            loop {
                let mut ev = MaybeUninit::<XEvent>::zeroed();
                XNextEvent(display, ev.as_mut_ptr());
                let mut ev = ev.assume_init();
                let cookie = &mut ev.generic_event_cookie;

                if XGetEventData(display, cookie as *mut XGenericEventCookie) == 1&&
                    cookie.type_ == GenericEvent &&
                    cookie.extension == xi_opcode {
                    match cookie.evtype {
                        13 => {
                            let rawev = cookie.data as *mut XIRawEvent;

                            let sym = XkbKeycodeToKeysym(display, (*rawev).detail as u8, 0, 0);
                            if sym == NoSymbol as u64 {
                                continue;
                            }

                            let s = CString::from_raw(XKeysymToString(sym));
                            let key = s.to_str().unwrap();
                            if key == "Return" {
                                state.write().unwrap().ds.disable();
                                addr.do_send(Message::UpdateEnableStatus { enabled: false, from_backend: true });
                                println!("Disable the robot");
                            } else if key == "space" {
                                state.write().unwrap().ds.estop();
                                addr.do_send(Message::EstopRobot { from_backend: true });
                                println!("Estop the robot");
                            }
                            mem::forget(s); // segfaults and stuff ensue if rust tries to free that pointer
                            // but i didn't see anything in lsan about this being a leak so idk
                        }
                        _ => {}
                    }
                }
            }
        });
    }
}


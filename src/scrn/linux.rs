use x11::xlib::{XOpenDisplay, XDefaultRootWindow, XFree, XCloseDisplay};
use std::ptr;
use x11::xrandr::{XRRGetScreenResources, XRRGetCrtcInfo, XRRFreeCrtcInfo, XRRFreeScreenResources};


pub unsafe fn screen_resolution() -> (f64, f64) {
    let disp = XOpenDisplay(ptr::null_mut());
    let def_wnd = XDefaultRootWindow(disp);
    let screens = XRRGetScreenResources(disp, def_wnd);

    // Get screen 0, use those dimensions
    if (*screens).ncrtc != 0 {
        let info = XRRGetCrtcInfo(disp, screens, *(*screens).crtcs);
        let width = (*info).width;
        let height = (*info).height;
        XRRFreeCrtcInfo(info);
        XRRFreeScreenResources(screens);
        XCloseDisplay(disp);
        (width as f64, height as f64)
    } else {
        println!("Could not detect any screens.");
        (0.0, 0.0)
    }
}
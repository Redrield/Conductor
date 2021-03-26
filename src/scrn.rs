//pub use self::arch::*;
//
//#[cfg(target_os = "linux")]
//#[path = "scrn/linux.rs"]
//pub mod arch;

//#[cfg(target_os = "macos")]
//#[path = "scrn/mac.rs"]
//mod arch;
//
//#[cfg(all(not(target_os = "linux"), not(target_os = "macos")))]
//#[path = "scrn/default.rs"]
//mod arch;
//

pub fn screen_resolution() -> (f64, f64) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.with_primary_monitor(|_, mon| {
        mon.map(|mon| {
            let (_, _, w, h) = mon.get_workarea();
            (w as f64, h as f64)
        }).unwrap_or((0f64, 0f64))
    })
}
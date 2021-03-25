pub use self::arch::*;

#[cfg(target_os = "linux")]
#[path = "scrn/linux.rs"]
mod arch;

#[cfg(target_os = "macos")]
#[path = "scrn/mac.rs"]
mod arch;

#[cfg(all(not(target_os = "linux"), not(target_os = "macos")))]
#[path = "scrn/default.rs"]
mod arch;

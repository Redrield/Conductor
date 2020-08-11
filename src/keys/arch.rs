pub use self::platform::*;

#[cfg(target_os = "linux")]
#[path = "arch/linux.rs"]
mod platform;


#[cfg(target_os = "windows")]
#[path = "arch/win.rs"]
mod platform;

#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "windows")
))]
#[path = "arch/default.rs"]
mod platform;

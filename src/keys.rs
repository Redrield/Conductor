pub use self::arch::*;

#[cfg(target_os = "linux")]
#[path = "keys/linux.rs"]
mod arch;

#[cfg(target_os = "macos")]
#[path = "keys/mac.rs"]
mod arch;


#[cfg(all(
not(target_os = "linux"),
not(target_os = "macos")
))]
#[path = "keys/default.rs"]
mod arch;

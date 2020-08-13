use core_foundation::base::{CFAllocatorRef, kCFAllocatorDefault, Boolean, CFIndexConvertible, CFRelease};
use core_foundation::array::CFArrayRef;
use core_foundation::dictionary::{CFDictionaryRef, CFDictionaryCreateMutable, kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionarySetValue, CFMutableDictionaryRef, CFDictionaryApplierFunction};
use libc::c_void;
use core_foundation::set::{CFSetRef, CFSetGetCount};
use core_foundation::number::CFNumberCreate;
use core_foundation::string::{CFStringCreateWithBytes, kCFStringEncodingUTF8};

#[path = "mgr/ffi.rs"]
mod ffi;

use ffi::*;

pub struct InputManager {
    iomgr: IOHIDManagerRef,
    return_key: IOHIDElementRef,
    space_key: IOHIDElementRef,
}

impl InputManager {
    pub fn new() {
        unsafe {
            let mgr = IOHIDManagerCreate(kCFAllocatorDefault, 0);

            let openStatus = IOHIDManagerOpen(mgr, 0);
        }
    }
}

impl Drop for InputManager {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.iomgr as *const _);
            CFRelease(self.return_key as *const _);
            CFRelease(self.space_key as *const _);
        }
    }
}

pub unsafe fn copy_devices(mgr: IOHIDManagerRef, page: u32, usage: u32) -> Option<CFSetRef> {
    let mask = copy_devices_mask(page, usage);

    IOHIDManagerSetDeviceMatching(mgr, mask);
    CFRelease(mask as *const _);

    let devices = IOHIDManagerCopyDevices(mgr);

    if devices.is_null() {
        return None;
    }

    let devCount = CFSetGetCount(devices);
    if devCount < 1 {
        CFRelease(devices as *const _);
        return None;
    }

    Some(devices)

}

pub unsafe fn copy_devices_mask(page: u32, usage: u32) -> CFDictionaryRef {
    let dict = CFDictionaryCreateMutable(kCFAllocatorDefault, 2,
    &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);

    let string  = "DeviceUsagePage";
    let string_ref = CFStringCreateWithBytes(kCFAllocatorDefault,
                                             string.as_ptr(),
                                             string.len().to_CFIndex(),
                                             kCFStringEncodingUTF8,
                                             false as Boolean);
    let value = CFNumberCreate(kCFAllocatorDefault, 9, &page as *const u32 as *const _);
    CFDictionarySetValue(dict, string_ref as *const _, value as *const _);
    CFRelease(string_ref as *const _);
    CFRelease(value as *const _);


    let string  = "DeviceUsage";
    let string_ref = CFStringCreateWithBytes(kCFAllocatorDefault,
                                             string.as_ptr(),
                                             string.len().to_CFIndex(),
                                             kCFStringEncodingUTF8,
                                             false as Boolean);
    let value = CFNumberCreate(kCFAllocatorDefault, 9, &usage as *const u32 as *const _);
    CFDictionarySetValue(dict, string_ref as *const _, value as *const _);
    CFRelease(string_ref as *const _);
    CFRelease(value as *const _);

    dict as CFDictionaryRef
}


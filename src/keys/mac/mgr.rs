use core_foundation::base::{CFAllocatorRef, kCFAllocatorDefault, Boolean, CFIndexConvertible, CFRelease};
use core_foundation::array::{CFArrayRef, CFArrayGetCount, CFArrayGetValueAtIndex};
use core_foundation::dictionary::{CFDictionaryRef, CFDictionaryCreateMutable, kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks, CFDictionarySetValue, CFMutableDictionaryRef, CFDictionaryApplierFunction};
use libc::c_void;
use core_foundation::set::{CFSetRef, CFSetGetCount, CFSetGetValue, CFSetGetValues};
use core_foundation::number::CFNumberCreate;
use core_foundation::string::{CFStringCreateWithBytes, kCFStringEncodingUTF8};

#[path = "mgr/ffi.rs"]
mod ffi;

use ffi::*;
use std::mem::MaybeUninit;
use std::ptr;

pub struct InputManager {
    iomgr: IOHIDManagerRef,
    return_key: IOHIDElementRef,
    space_key: IOHIDElementRef,
}

impl InputManager {
    pub fn new() -> Option<InputManager> {
        unsafe {
            let mgr = IOHIDManagerCreate(kCFAllocatorDefault, 0);

            let open_status = IOHIDManagerOpen(mgr, 0);
            if open_status != 0 {
                println!("Failed to create macOS HID manager");
                CFRelease(mgr as *const _);
                return None;
            }

            if let Some((ret, spc)) = initialize_keys(mgr) {
                Some(InputManager {
                    iomgr: mgr,
                    return_key: ret,
                    space_key: spc
                })
            } else {
                CFRelease(mgr as *const _);
                return None;
            }
        }
    }

    pub fn poll_enter(&self) -> bool {
        unsafe {
            let mut value = MaybeUninit::<IOHIDValueRef>::zeroed();
            let dev = IOHIDElementGetDevice(self.return_key);
            IOHIDDeviceGetValue(dev, self.return_key, value.as_mut_ptr());
            let value = value.assume_init();
            IOHIDValueGetIntegerValue(value) == 1
        }
    }

    pub fn poll_spacebar(&self) -> bool {
        unsafe {
            let mut value = MaybeUninit::<IOHIDValueRef>::zeroed();
            let dev = IOHIDElementGetDevice(self.space_key);
            IOHIDDeviceGetValue(dev, self.space_key, value.as_mut_ptr());
            let value = value.assume_init();
            IOHIDValueGetIntegerValue(value) == 1
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

unsafe fn initialize_keys(mgr: IOHIDManagerRef) -> Option<(IOHIDElementRef, IOHIDElementRef)> {
    let kHIDPage_GenericDesktop = 0x01;
    let kHIDUsage_GD_Keyboard = 0x06;

    match copy_devices(mgr, kHIDPage_GenericDesktop, kHIDUsage_GD_Keyboard) {
        Some(keebs) => {
            let count = CFSetGetCount(keebs);
            let mut v = vec![0 as IOHIDDeviceRef; count as usize];
            CFSetGetValues(keebs, v.as_mut_ptr() as *mut *const _);

            let mut ret = None;
            for keyboard in v {
                ret = load_keyboard(mgr, keyboard);
                if ret.is_some() {
                    break;
                }
            }
            CFRelease(keebs as *const _);
            ret
        }
        None => None
    }
}

unsafe fn load_keyboard(mgr: IOHIDManagerRef, keyboard: IOHIDDeviceRef) -> Option<(IOHIDElementRef, IOHIDElementRef)> {
    let keys = IOHIDDeviceCopyMatchingElements(keyboard, ptr::null(), 0);
    if keys.is_null() {
        println!("NULL keys pointer");
        return None;
    }

    let count = CFArrayGetCount(keys);
    if count == 0 {
        println!("Keyless keyboard");
        CFRelease(keys as *const _);
    }

    let mut return_key = None;
    let mut space_key = None;

    for i in 0..count {
        let key = CFArrayGetValueAtIndex(keys, i) as IOHIDElementRef;

        if IOHIDElementGetUsagePage(key) != kHIDPage_KeyboardOrKeypad {
            continue;
        }

        let usage = IOHIDElementGetUsage(key);

        if usage == kHIDUsage_KeyboardReturnOrEnter {
            return_key = Some(key);
        }

        if usage == kHIDUsage_KeyboardSpacebar {
            space_key = Some(key);
        }
    }

    match (return_key, space_key) {
        (Some(ret), Some(spc)) => Some((ret, spc)),
        _ => None
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


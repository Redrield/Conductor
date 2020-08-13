use libc::c_void;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::base::CFAllocatorRef;
use core_foundation::set::CFSetRef;
use core_foundation::array::CFArrayRef;

#[repr(C)]
struct __IOHIDManager(c_void);
pub type IOHIDManagerRef = *const __IOHIDManager;

#[repr(C)]
struct __IOHIDDevice(c_void);
pub type IOHIDDeviceRef = *const __IOHIDDevice;

#[repr(C)]
struct __IOHIDElement(c_void);
pub type IOHIDElementRef = *const __IOHIDElement;

#[repr(C)]
struct __IOHIDValue(c_void);
pub type IOHIDValueRef = *const __IOHIDDeviceValue;

pub const kHIDPage_KeyboardOrKeypad: u8 = 0x07;

#[link(name = "IOKit", kind = "framework")]
extern {
    pub static kIOReturnSuccess: i32;

    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: u32) -> IOHIDManagerRef;
    pub fn IOHIDManagerOpen(mgr: IOHIDManagerRef, options: u32) -> i32;
    pub fn IOHIDDeviceCopyMatchingElements(device: IOHIDDeviceRef, matching: CFDictionaryRef, options: u32) -> CFArrayRef;
    pub fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetUsage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerCopyDevices(manager: IOHIDManagerRef) -> CFSetRef;
    pub fn IOHIDElementGetDevice(element: IOHIDElementRef) -> IOHIDDeviceRef;
    pub fn IOHIDDeviceGetValue(device: IOHIDDeviceRef, element: IOHIDElementRef, value: *mut IOHIDValueRef) -> i32;
    pub fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> i64;
}




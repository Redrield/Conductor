#![allow(non_upper_case_globals)]

use libc::c_void;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::base::CFAllocatorRef;
use core_foundation::set::CFSetRef;
use core_foundation::array::CFArrayRef;

#[repr(C)]
pub struct __IOHIDManager(c_void);
pub type IOHIDManagerRef = *const __IOHIDManager;

#[repr(C)]
pub struct __IOHIDDevice(c_void);
pub type IOHIDDeviceRef = *const __IOHIDDevice;

#[repr(C)]
pub struct __IOHIDElement(c_void);
pub type IOHIDElementRef = *const __IOHIDElement;

#[repr(C)]
pub struct __IOHIDValue(c_void);
pub type IOHIDValueRef = *const __IOHIDValue;

pub const kHIDPage_KeyboardOrKeypad: u32 = 0x07;
pub const kHIDUsage_KeyboardReturnOrEnter: u32 = 0x28;
pub const kHIDUsage_KeyboardSpacebar: u32 = 0x2C;
pub const kHIDUsage_KeyboardOpenBracket: u32 = 0x2F;
pub const kHIDUsage_KeyboardCloseBracket: u32 = 0x30;
pub const kHIDUsage_KeyboardBackslash: u32 = 0x31;
pub const kHIDPage_GenericDesktop: u32 = 0x01;
pub const kHIDUsage_GD_Keyboard: u32 = 0x06;

#[link(name = "IOKit", kind = "framework")]
extern {
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




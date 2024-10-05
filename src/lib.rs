extern-crate libc;

use canlib_sys::*;
use libc::{c_int, c_uint, c_long, c_ulong, c_void};

mod canlib_sys;
use canlib_sys::*;


pub type CanHandle = i32;

#[derive(Copy, Clone)]
pub enum CanDriver {
    Off = 0,
    Silent =  1,
    Normal = 4,
    SelfReception =  8,
}

#[derive(Copy, Clone)]
pub enum CanOpen {
    Exclusive = 0x0008,
    RequireExtended = 0x0010,
    AcceptVirtual = 0x0020,
    OverrideExclusive = 0x0040,
    RequireInitAccess = 0x0080,
    NoInitAccess = 0x0100,
    AcceptLargeDlc = 0x0200,
    CanFd = 0x0400,
    CanFdNonIso = 0x0800,
    InternalL = 0x1000,
}

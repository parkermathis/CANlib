extern crate libc;

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

#[derive(Copy, Clone)]
pub enum CanBitrate {
    _1M     = -1,
    _500K   = -2,
    _250K   = -3,
    _125K   = -4,
    _100K   = -5,
    _62K    = -6,
    _50K    = -7,
    _83K    = -8,
    _10K    = -9,
    Fd500K  = -1000,
    Fd1M    = -1001,
    Fd2M    = -1002,
    Fd4M    = -1003,
    Fd8M    = -1004,
}

#[derive(Copy, Clone)]
pub enum CanMsg {
    Mask    = 0x00FF,
    Rtr     = 0x0001,
    Std     = 0x0002,
    Ext     = 0x0004,
    Wakeup  = 0x0008,
    Nerr    = 0x0010,
    ErrorFrame = 0x0020,
    TxAck   = 0x0040,
    TxRq    = 0x0080,
    DelayMsg = 0x0100,
    SingleShot = 0x1000000,
    TxNack  = 0x2000000,
    MsgAbl  = 0x4000000,
}

pub enum CanFdMsg {
    Fdf = 0x010000,
    Brs = 0x020000,
    Esi = 0x040000,
}

pub enum CanStatus {
    CanOk               = 0,
    ErrParam            = -1,
    ErrNoMsg            = -2,
    ErrNotFound         = -3,
    ErrNoMem            = -4,
    ErrNoChannels       = -5,
    ErrInterrupted      = -6,
    ErrTimeout          = -7,
    ErrNotInitialized   = -8,
    ErrNoHandles        = -9,
    ErrInvHandle        = -10,
    ErrIniFile          = -11,
    ErrDriver           = -12,
    ErrTxBufOfl         = -13,
    ErrReserved1        = -14,
    ErrHardware         = -15,
    ErrDynaLoad         = -16,
    ErrDynaLib          = -17,
    ErrDynaInit         = -18,
    ErrNotSupported     = -19,
    ErrReserved5        = -20, 
    ErrReserved6        = -21, 
    ErrReserved2        = -22, 
    ErrDriverLoad       = -23,
    ErrDriverFailed     = -24,
    ErrNoConfigMgr      = -25,
    ErrNoCard           = -26,
    ErrReserved7        = -27, 
    ErrRegistry         = -28,
    ErrLicense          = -29,
    ErrInternal         = -30,
    ErrNoAccess         = -31,
    ErrNotImplemented   = -32,
    ErrDeviceFile       = -33,
    ErrHostFile         = -34,
    ErrDisk             = -35,
    ErrCrc              = -36,
    ErrConfig           = -37,
    ErrMemoFail         = -38,
    ErrScriptFail       = -39,
    ErrScriptWrongVersion = -40,
    ErrScriptTxeContainerVersion = -41,
    ErrScriptTxeContainerFormat = -42,
    ErrBufferTooSmall   = -43,
    ErrIoWrongPinType   = -44,
    ErrIoNotConfirmed   = -45,
    ErrIoConfigChanged  = -46,
    ErrIoPending        = -47,
    ErrIoNoValidConfig  = -48,
    ErrReserved         = -49,
}

fn i32_to_can_status(num: i32) -> CanStatus {
	match num {
		0 => CanStatus::CanOk,
		-1 => CanStatus::ErrParam,
		-2 => CanStatus::ErrNoMsg,       
		-3 => CanStatus::ErrNotFound,      
		-4 => CanStatus::ErrNoMem,          
		-5 => CanStatus::ErrNoChannels,        
		-6 => CanStatus::ErrInterrupted,   
		-7 => CanStatus::ErrTimeout,          
		-8 => CanStatus::ErrNotInitialized,   
		-9 => CanStatus::ErrNoHandles,        
		-10 => CanStatus::ErrInvHandle,        
		-11 => CanStatus::ErrIniFile,          
		-12 => CanStatus::ErrDriver,           
		-13 => CanStatus::ErrTxBufOfl,       
		-14 => CanStatus::ErrReserved1,        
		-15 => CanStatus::ErrHardware,       
		-16 => CanStatus::ErrDynaLoad,        
		-17 => CanStatus::ErrDynaLib,         
		-18 => CanStatus::ErrDynaInit,        
		-19 => CanStatus::ErrNotSupported,     
		-20 => CanStatus::ErrReserved5,         
		-21 => CanStatus::ErrReserved6,      
		-22 => CanStatus::ErrReserved2,      
		-23 => CanStatus::ErrDriverLoad,      
		-24 => CanStatus::ErrDriverFailed,    
		-25 => CanStatus::ErrNoConfigMgr,    
		-26 => CanStatus::ErrNoCard,          
		-27 => CanStatus::ErrReserved7,     
		-28 => CanStatus::ErrRegistry,      
		-29 => CanStatus::ErrLicense,       
		-30 => CanStatus::ErrInternal,       
		-31 => CanStatus::ErrNoAccess,          
		-32 => CanStatus::ErrNotImplemented,   
		-33 => CanStatus::ErrDeviceFile,       
		-34 => CanStatus::ErrHostFile,        
		-35 => CanStatus::ErrDisk,           
		-36 => CanStatus::ErrCrc,              
		-37 => CanStatus::ErrConfig,         
		-38 => CanStatus::ErrMemoFail,       
		-39 => CanStatus::ErrScriptFail,   
		-40 => CanStatus::ErrScriptWrongVersion,
		-41 => CanStatus::ErrScriptTxeContainerVersion,  
		-42 => CanStatus::ErrScriptTxeContainerFormat,  
		-43 => CanStatus::ErrBufferTooSmall, 
		-44 => CanStatus::ErrIoWrongPinType, 
		-45 => CanStatus::ErrIoNotConfirmed,  
		-46 => CanStatus::ErrIoConfigChanged, 
		-47 => CanStatus::ErrIoPending,       
		-48 => CanStatus::ErrIoNoValidConfig,  
		-49 => CanStatus::ErrReserved,       
		_   => unreachable!(), // Technically unreachable (?)
	}
}


pub fn can_initialize_library() {
    unsafe {
        canInitializeLibrary();
    }
}

pub fn can_unload_library() {
    unsafe {
        canUnloadLibrary();
    }
}

pub fn can_open_channel(channel_no: i32, channel_flags: i32) -> CanHandle {
    let handle;
    unsafe {
      handle = canOpenChannel(channel_no as c_int, channel_flags as c_int);
    }
    handle
}

pub fn can_set_bus_params(hnd: CanHandle, freq: CanBitrate, tseg1: u32, tseg2: u32, sjw: u32, no_samp: u32) -> CanStatus {
    let status;
    unsafe {
        status = canSetBusParams(hnd, freq as c_long, tseg1 as c_uint, tseg2 as c_uint, sjw as c_uint, no_samp as c_uint, 0) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_set_bus_output_control(hnd: CanHandle, driver_type: CanDriver) -> CanStatus {
    let status;
    unsafe {
        status = canSetBusOutputControl(hnd, driver_type as u32) as i32;
    }
    i32_to_can_status(status)
}
pub fn can_bus_on(hnd: CanHandle) -> CanStatus {
    let status;
    unsafe {
        status = canBusOn(hnd) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_bus_off(hnd: CanHandle) -> CanStatus {
    let status;
    unsafe {
        status = canBusOff(hnd) as i32;
    }    
    i32_to_can_status(status)
}

pub fn can_close(hnd: CanHandle) -> CanStatus {
    let status;
    unsafe {
        status = canClose(hnd) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_write(hnd: CanHandle, id: i64, msg_buffer: &mut [u8], dlc: u32, flag: u32) -> CanStatus {
    let msg_bfr_ptr: *mut c_void = msg_buffer as *mut _ as *mut c_void;            
    let status;
    unsafe {
        status = canWrite(hnd, id as c_long, msg_bfr_ptr, dlc as c_uint, flag as c_uint) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_write_wait(hnd: CanHandle, id: i64, msg_buffer: &mut [u8], dlc: u32, flag: u32, time: u32) -> CanStatus {
    let msg_bfr_ptr: *mut c_void = msg_buffer as *mut _ as *mut c_void;
    let status;
    unsafe {
        status = canWriteWait(hnd, id as c_long, msg_bfr_ptr, dlc as c_uint, flag as c_uint, time as c_ulong) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_write_sync(hnd: CanHandle, timeout: u32) -> CanStatus {
    let status;
    unsafe {
        status = canWriteSync(hnd, timeout as c_ulong) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_read(hnd: CanHandle, id: *mut i64, msg_buffer: &mut [u8], dlc: *mut u32, flag: *mut u32, time: *mut u32) -> CanStatus {
    let msg_bfr_ptr: *mut c_void = msg_buffer as *mut _ as *mut c_void; 
    let status;
    unsafe{
        status = canRead(hnd, id as *mut c_long, msg_bfr_ptr, dlc as *mut c_uint, flag as *mut c_uint, time as *mut c_ulong) as i32;   
    }
    i32_to_can_status(status)
}

pub fn can_read_wait(hnd: CanHandle, id: *mut i64, msg_buffer: &mut [u8], dlc: *mut u32, flag: *mut u32, time: *mut u32, timeout: *mut u32) -> CanStatus {
    let msg_bfr_ptr: *mut c_void = msg_buffer as *mut _ as *mut c_void; 
    let status;
    unsafe{
        status = canReadWait(hnd, id as *mut c_long, msg_bfr_ptr, dlc as *mut c_uint, flag as *mut c_uint, time as *mut c_ulong, timeout as *mut c_ulong) as i32;   
    }
    i32_to_can_status(status)
}

pub fn can_read_specific(hnd: CanHandle, id: i64, msg_buffer: &mut [u8], dlc: *mut u32, flag: *mut u32, time: *mut u32) -> CanStatus {
    let msg_bfr_ptr: *mut c_void = msg_buffer as *mut _ as *mut c_void;
    let status;
    unsafe {
        status = canReadSpecific(hnd, id as c_long, msg_bfr_ptr, dlc as *mut c_uint, flag as *mut c_uint, time as *mut c_ulong) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_read_sync(hnd: CanHandle, timeout: u32) -> CanStatus {
    let status;
    unsafe {
        status = canReadSync(hnd, timeout as c_ulong) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_read_sync_specific(hnd: CanHandle, id: i64, timeout: u32) -> CanStatus {
    let status;
    unsafe {
        status = canReadSyncSpecific(hnd, id as c_long, timeout as c_ulong) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_read_specific_skip(hnd: CanHandle, id: i64, msg_buffer: &mut [u8], dlc: *mut u32, flag: *mut u32, time: *mut u32) -> CanStatus {
    let msg_bfr_ptr: *mut c_void = msg_buffer as *mut _ as *mut c_void;
    let status;
    unsafe {
        status = canReadSpecificSkip(hnd, id as c_long, msg_bfr_ptr, dlc as *mut c_uint, flag as *mut c_uint, time as *mut c_ulong) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_flush_receive_queue(hnd: CanHandle) -> CanStatus {
    let status;
    unsafe {
        status = canFlushReceiveQueue(hnd) as i32;
    }
    i32_to_can_status(status)
}

pub fn can_flush_transmit_queue(hnd: CanHandle) -> CanStatus {
    let status;
    unsafe {
        status = canFlushTransmitQueue(hnd) as i32;
    }
    i32_to_can_status(status)
}

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libc;
use libc::{c_int, c_long, c_uint, c_void, c_ulong, c_char, c_uchar, c_ushort};

pub const canOPEN_EXCLUSIVE: i32  = 0x0008;
pub const canOPEN_REQUIRE_EXTENDED: i32 = 0x0010;
pub const canOPEN_ACCEPT_VIRTUAL: i32 = 0x0020;
pub const canOPEN_OVERRIDE_EXCLUSIVE: i32 = 0x0040;
pub const canOPEN_REQUIRE_INIT_ACCESS: i32 = 0x0080;
pub const canOPEN_NO_INIT_ACCESS: i32 = 0x0100;
pub const canOPEN_ACCEPT_LARGE_DLC: i32 = 0x0200;
pub const canOPEN_CAN_FD: i32 = 0x0400;
pub const canOPEN_CAN_FD_NONISO: i32 = 0x0800;

pub const canFILTER_ACCEPT: u32 = 1;
pub const canFILTER_REJECT: u32 = 2;
pub const canFILTER_SET_CODE_STD: u32 = 3;
pub const canFILTER_SET_MASK_STD: u32 = 4;
pub const canFILTER_SET_CODE_EXT: u32 = 5;
pub const canFILTER_SET_MASK_EXT: u32 = 6;
pub const canFILTER_NULL_MASK: u32 = 0;

pub const canDRIVER_NORMAL: u32 = 4; // DEFAULT
pub const canDRIVER_SILENT: u32 = 1;
pub const canDRIVER_OFF: u32 = 0;

pub const canBITRATE_1M: i32 = -1;
pub const canBITRATE_500K: i32 = -2;
pub const canBITRATE_250K: i32 = -3;
pub const canBITRATE_125K: i32 = -4;
pub const canBITRATE_100K: i32 = -5;
pub const canBITRATE_62K: i32 = -6;
pub const canBITRATE_50K: i32 = -7;
pub const canBITRATE_83K: i32 = -8;
pub const canBITRATE_10K: i32 = -9;

pub const canFD_BITRATE_500K_80P: i32 = -1000;
pub const canFD_BITRATE_1M_80P: i32 = -1001;
pub const canFD_BITRATE_2M_80P: i32 = -1002;
pub const canFD_BITRATE_4M_80P: i32 = -1003;
pub const canFD_BITRATE_8M_60P: i32 = -1004;

pub const canMSG_MASK: u32 = 255;
pub const canMSG_RTR: u32 = 1;
pub const canMSG_STD: u32 = 2;
pub const canMSG_EXT: u32 = 4;
pub const canMSG_WAKEUP: u32 = 8;

pub const canFDMSG_MASK: u32 = 16711680;
pub const canFDMSG_EDL: u32 = 65536;
pub const canFDMSG_FDF: u32 = 65536;
pub const canFDMSG_BRS: u32 = 131072;
pub const canFDMSG_ESI: u32 = 262144;

pub const canMSG_NERR: u32 = 16;
pub const canMSG_ERROR_FRAME: u32 = 32;
pub const canMSG_TXACK: u32 = 64;
pub const canMSG_TXRQ: u32 = 128;
pub const canMSG_DELAY_MSG: u32 = 256;
pub const canMSG_SINGLE_SHOT: u32 = 16777216;
pub const canMSG_TXNACK: u32 = 33554432;
pub const canMSG_ABL: u32 = 67108864;

pub const canMSGERR_MASK: u32 = 65280;
pub const canMSGERR_HW_OVERRUN: u32 = 512;
pub const canMSGERR_SW_OVERRUN: u32 = 1024;
pub const canMSGERR_STUFF: u32 = 2048;
pub const canMSGERR_FORM: u32 = 4096;
pub const canMSGERR_CRC: u32 = 8192;
pub const canMSGERR_BIT0: u32 = 16384;
pub const canMSGERR_BIT1: u32 = 32768;
pub const canMSGERR_OVERRUN: u32 = 1536;
pub const canMSGERR_BIT: u32 = 49152;
pub const canMSGERR_BUSERR: u32 = 63488;

pub const canCHANNELDATA_CHANNEL_CAP: u32 = 1;
pub const canCHANNELDATA_TRANS_CAP: u32 = 2;
pub const canCHANNELDATA_CHANNEL_FLAGS: u32 = 3;
pub const canCHANNELDATA_CARD_TYPE: u32 = 4;
pub const canCHANNELDATA_CARD_NUMBER: u32 = 5;
pub const canCHANNELDATA_CHAN_NO_ON_CARD: u32 = 6;
pub const canCHANNELDATA_CARD_SERIAL_NO: u32 = 7;
pub const canCHANNELDATA_TRANS_SERIAL_NO: u32 = 8;
pub const canCHANNELDATA_CARD_FIRMWARE_REV: u32 = 9;
pub const canCHANNELDATA_CARD_HARDWARE_REV: u32 = 10;
pub const canCHANNELDATA_CARD_UPC_NO: u32 = 11;
pub const canCHANNELDATA_TRANS_UPC_NO: u32 = 12;
pub const canCHANNELDATA_CHANNEL_NAME: u32 = 13;
pub const canCHANNELDATA_DLL_FILE_VERSION: u32 = 14;
pub const canCHANNELDATA_DLL_PRODUCT_VERSION: u32 = 15;
pub const canCHANNELDATA_DLL_FILETYPE: u32 = 16;
pub const canCHANNELDATA_TRANS_TYPE: u32 = 17;
pub const canCHANNELDATA_DEVICE_PHYSICAL_POSITION: u32 = 18;
pub const canCHANNELDATA_UI_NUMBER: u32 = 19;
pub const canCHANNELDATA_TIMESYNC_ENABLED: u32 = 20;
pub const canCHANNELDATA_DRIVER_FILE_VERSION: u32 = 21;
pub const canCHANNELDATA_DRIVER_PRODUCT_VERSION: u32 = 22;
pub const canCHANNELDATA_MFGNAME_UNICODE: u32 = 23;
pub const canCHANNELDATA_MFGNAME_ASCII: u32 = 24;
pub const canCHANNELDATA_DEVDESCR_UNICODE: u32 = 25;
pub const canCHANNELDATA_DEVDESCR_ASCII: u32 = 26;
pub const canCHANNELDATA_DRIVER_NAME: u32 = 27;
pub const canCHANNELDATA_CHANNEL_QUALITY: u32 = 28;
pub const canCHANNELDATA_ROUNDTRIP_TIME: u32 = 29;
pub const canCHANNELDATA_BUS_TYPE: u32 = 30;
pub const canCHANNELDATA_DEVNAME_ASCII: u32 = 31;
pub const canCHANNELDATA_TIME_SINCE_LAST_SEEN: u32 = 32;
pub const canCHANNELDATA_REMOTE_OPERATIONAL_MODE: u32 = 33;
pub const canCHANNELDATA_REMOTE_PROFILE_NAME: u32 = 34;
pub const canCHANNELDATA_REMOTE_HOST_NAME: u32 = 35;
pub const canCHANNELDATA_REMOTE_MAC: u32 = 36;
pub const canCHANNELDATA_MAX_BITRATE: u32 = 37;
pub const canCHANNELDATA_CHANNEL_CAP_MASK: u32 = 38;
pub const canCHANNELDATA_IS_REMOTE: u32 = 40;
pub const canCHANNELDATA_REMOTE_TYPE: u32 = 41;
pub const canCHANNELDATA_LOGGER_TYPE: u32 = 42;
pub const canCHANNELDATA_HW_STATUS: u32 = 43;
pub const canCHANNELDATA_FEATURE_EAN: u32 = 44;
pub const canCHANNELDATA_CUST_CHANNEL_NAME: u32 = 39;

pub const canCHANNEL_IS_EXCLUSIVE: u32 = 1;
pub const canCHANNEL_IS_OPEN: u32 = 2;
pub const canCHANNEL_IS_CANFD: u32 = 4;
pub const canCHANNEL_IS_LIN: u32 = 16;
pub const canCHANNEL_IS_LIN_MASTER: u32 = 32;
pub const canCHANNEL_IS_LIN_SLAVE: u32 = 64;

pub const canHWTYPE_NONE: u32 = 0;
pub const canHWTYPE_VIRTUAL: u32 = 1;
pub const canHWTYPE_LAPCAN: u32 = 2;
pub const canHWTYPE_CANPARI: u32 = 3;
pub const canHWTYPE_PCCAN: u32 = 8;
pub const canHWTYPE_PCICAN: u32 = 9;
pub const canHWTYPE_USBCAN: u32 = 11;
pub const canHWTYPE_PCICAN_II: u32 = 40;
pub const canHWTYPE_USBCAN_II: u32 = 42;
pub const canHWTYPE_SIMULATED: u32 = 44;
pub const canHWTYPE_ACQUISITOR: u32 = 46;
pub const canHWTYPE_LEAF: u32 = 48;
pub const canHWTYPE_PC104_PLUS: u32 = 50;
pub const canHWTYPE_PCICANX_II: u32 = 52;
pub const canHWTYPE_MEMORATOR_II: u32 = 54;
pub const canHWTYPE_MEMORATOR_PRO: u32 = 54;
pub const canHWTYPE_USBCAN_PRO: u32 = 56;
pub const canHWTYPE_IRIS: u32 = 58;
pub const canHWTYPE_BLACKBIRD: u32 = 58;
pub const canHWTYPE_MEMORATOR_LIGHT: u32 = 60;
pub const canHWTYPE_MINIHYDRA: u32 = 62;
pub const canHWTYPE_EAGLE: u32 = 62;
pub const canHWTYPE_BAGEL: u32 = 64;
pub const canHWTYPE_BLACKBIRD_V2: u32 = 64;
pub const canHWTYPE_MINIPCIE: u32 = 66;
pub const canHWTYPE_USBCAN_KLINE: u32 = 68;
pub const canHWTYPE_ETHERCAN: u32 = 70;
pub const canHWTYPE_USBCAN_LIGHT: u32 = 72;
pub const canHWTYPE_USBCAN_PRO2: u32 = 74;
pub const canHWTYPE_PCIE_V2: u32 = 76;
pub const canHWTYPE_MEMORATOR_PRO2: u32 = 78;
pub const canHWTYPE_LEAF2: u32 = 80;
pub const canHWTYPE_MEMORATOR_V2: u32 = 82;
pub const canHWTYPE_CANLINHYBRID: u32 = 84;
pub const canHWTYPE_DINRAIL: u32 = 86;

pub const canCHANNEL_CAP_EXTENDED_CAN: u32 = 1;
pub const canCHANNEL_CAP_BUS_STATISTICS: u32 = 2;
pub const canCHANNEL_CAP_ERROR_COUNTERS: u32 = 4;
pub const canCHANNEL_CAP_RESERVED_2: u32 = 8;
pub const canCHANNEL_CAP_GENERATE_ERROR: u32 = 16;
pub const canCHANNEL_CAP_GENERATE_OVERLOAD: u32 = 32;
pub const canCHANNEL_CAP_TXREQUEST: u32 = 64;
pub const canCHANNEL_CAP_TXACKNOWLEDGE: u32 = 128;
pub const canCHANNEL_CAP_VIRTUAL: u32 = 65536;
pub const canCHANNEL_CAP_SIMULATED: u32 = 131072;
pub const canCHANNEL_CAP_RESERVED_1: u32 = 262144;
pub const canCHANNEL_CAP_CAN_FD: u32 = 524288;
pub const canCHANNEL_CAP_CAN_FD_NONISO: u32 = 1048576;
pub const canCHANNEL_CAP_SILENT_MODE: u32 = 2097152;
pub const canCHANNEL_CAP_SINGLE_SHOT: u32 = 4194304;
pub const canCHANNEL_CAP_LOGGER: u32 = 8388608;
pub const canCHANNEL_CAP_REMOTE_ACCESS: u32 = 16777216;
pub const canCHANNEL_CAP_SCRIPT: u32 = 33554432;
pub const canCHANNEL_CAP_LIN_HYBRID: u32 = 67108864;
pub const canCHANNEL_CAP_IO_API: u32 = 134217728;
pub const canCHANNEL_CAP_DIAGNOSTICS: u32 = 268435456;
pub const canDRIVER_CAP_HIGHSPEED: u32 = 1;

pub const canVERSION_CANLIB32_VERSION: u32 = 0;
pub const canVERSION_CANLIB32_PRODVER: u32 = 1;
pub const canVERSION_CANLIB32_PRODVER32: u32 = 2;
pub const canVERSION_CANLIB32_BETA: u32 = 3;

pub const canOBJBUF_TYPE_AUTO_RESPONSE: u32 = 1;
pub const canOBJBUF_TYPE_PERIODIC_TX: u32 = 2;

pub const canOBJBUF_AUTO_RESPONSE_RTR_ONLY: u32 = 1;

pub const canTRANSCEIVER_LINEMODE_NA: u32 = 0;
pub const canTRANSCEIVER_LINEMODE_SWC_SLEEP: u32 = 4;
pub const canTRANSCEIVER_LINEMODE_SWC_NORMAL: u32 = 5;
pub const canTRANSCEIVER_LINEMODE_SWC_FAST: u32 = 6;
pub const canTRANSCEIVER_LINEMODE_SWC_WAKEUP: u32 = 7;
pub const canTRANSCEIVER_LINEMODE_SLEEP: u32 = 8;
pub const canTRANSCEIVER_LINEMODE_NORMAL: u32 = 9;
pub const canTRANSCEIVER_LINEMODE_STDBY: u32 = 10;
pub const canTRANSCEIVER_LINEMODE_TT_CAN_H: u32 = 11;
pub const canTRANSCEIVER_LINEMODE_TT_CAN_L: u32 = 12;
pub const canTRANSCEIVER_LINEMODE_OEM1: u32 = 13;
pub const canTRANSCEIVER_LINEMODE_OEM2: u32 = 14;
pub const canTRANSCEIVER_LINEMODE_OEM3: u32 = 15;
pub const canTRANSCEIVER_LINEMODE_OEM4: u32 = 16;
pub const canTRANSCEIVER_RESNET_NA: u32 = 0;
pub const canTRANSCEIVER_RESNET_MASTER: u32 = 1;
pub const canTRANSCEIVER_RESNET_MASTER_STBY: u32 = 2;
pub const canTRANSCEIVER_RESNET_SLAVE: u32 = 3;
pub const canTRANSCEIVER_TYPE_UNKNOWN: u32 = 0;
pub const canTRANSCEIVER_TYPE_251: u32 = 1;
pub const canTRANSCEIVER_TYPE_252: u32 = 2;
pub const canTRANSCEIVER_TYPE_DNOPTO: u32 = 3;
pub const canTRANSCEIVER_TYPE_W210: u32 = 4;
pub const canTRANSCEIVER_TYPE_SWC_PROTO: u32 = 5;
pub const canTRANSCEIVER_TYPE_SWC: u32 = 6;
pub const canTRANSCEIVER_TYPE_EVA: u32 = 7;
pub const canTRANSCEIVER_TYPE_FIBER: u32 = 8;
pub const canTRANSCEIVER_TYPE_K251: u32 = 9;
pub const canTRANSCEIVER_TYPE_K: u32 = 10;
pub const canTRANSCEIVER_TYPE_1054_OPTO: u32 = 11;
pub const canTRANSCEIVER_TYPE_SWC_OPTO: u32 = 12;
pub const canTRANSCEIVER_TYPE_TT: u32 = 13;
pub const canTRANSCEIVER_TYPE_1050: u32 = 14;
pub const canTRANSCEIVER_TYPE_1050_OPTO: u32 = 15;
pub const canTRANSCEIVER_TYPE_1041: u32 = 16;
pub const canTRANSCEIVER_TYPE_1041_OPTO: u32 = 17;
pub const canTRANSCEIVER_TYPE_RS485: u32 = 18;
pub const canTRANSCEIVER_TYPE_LIN: u32 = 19;
pub const canTRANSCEIVER_TYPE_KONE: u32 = 20;
pub const canTRANSCEIVER_TYPE_CANFD: u32 = 22;
pub const canTRANSCEIVER_TYPE_CANFD_LIN: u32 = 24;
pub const canTRANSCEIVER_TYPE_LINX_LIN: u32 = 64;
pub const canTRANSCEIVER_TYPE_LINX_J1708: u32 = 66;
pub const canTRANSCEIVER_TYPE_LINX_K: u32 = 68;
pub const canTRANSCEIVER_TYPE_LINX_SWC: u32 = 70;
pub const canTRANSCEIVER_TYPE_LINX_LS: u32 = 72;

pub const canEVENT_RX: u32 = 32000;
pub const canEVENT_TX: u32 = 32001;
pub const canEVENT_ERROR: u32 = 32002;
pub const canEVENT_STATUS: u32 = 32003;
pub const canEVENT_ENVVAR: u32 = 32004;
pub const canEVENT_BUSONOFF: u32 = 32005;

pub const canNOTIFY_NONE: u32 = 0;
pub const canNOTIFY_RX: u32 = 1;
pub const canNOTIFY_TX: u32 = 2;
pub const canNOTIFY_ERROR: u32 = 4;
pub const canNOTIFY_STATUS: u32 = 8;
pub const canNOTIFY_ENVVAR: u32 = 16;
pub const canNOTIFY_BUSONOFF: u32 = 32;

pub const canSTAT_ERROR_PASSIVE: u32 = 1;
pub const canSTAT_BUS_OFF: u32 = 2;
pub const canSTAT_ERROR_WARNING: u32 = 4;
pub const canSTAT_ERROR_ACTIVE: u32 = 8;
pub const canSTAT_TX_PENDING: u32 = 16;
pub const canSTAT_RX_PENDING: u32 = 32;
pub const canSTAT_RESERVED_1: u32 = 64;
pub const canSTAT_TXERR: u32 = 128;
pub const canSTAT_RXERR: u32 = 256;
pub const canSTAT_HW_OVERRUN: u32 = 512;
pub const canSTAT_SW_OVERRUN: u32 = 1024;
pub const canSTAT_OVERRUN: u32 = 1536;

pub const kvREMOTE_TYPE_NOT_REMOTE: u32 = 0;
pub const kvREMOTE_TYPE_WLAN: u32 = 1;
pub const kvREMOTE_TYPE_LAN: u32 = 2;
pub const kvLOGGER_TYPE_NOT_A_LOGGER: u32 = 0;
pub const kvLOGGER_TYPE_V1: u32 = 1;
pub const kvLOGGER_TYPE_V2: u32 = 2;

pub const kvLED_ACTION_ALL_LEDS_ON: u32 = 0;
pub const kvLED_ACTION_ALL_LEDS_OFF: u32 = 1;
pub const kvLED_ACTION_LED_0_ON: u32 = 2;
pub const kvLED_ACTION_LED_0_OFF: u32 = 3;
pub const kvLED_ACTION_LED_1_ON: u32 = 4;
pub const kvLED_ACTION_LED_1_OFF: u32 = 5;
pub const kvLED_ACTION_LED_2_ON: u32 = 6;
pub const kvLED_ACTION_LED_2_OFF: u32 = 7;
pub const kvLED_ACTION_LED_3_ON: u32 = 8;
pub const kvLED_ACTION_LED_3_OFF: u32 = 9;
pub const kvLED_ACTION_LED_4_ON: u32 = 10;
pub const kvLED_ACTION_LED_4_OFF: u32 = 11;
pub const kvLED_ACTION_LED_5_ON: u32 = 12;
pub const kvLED_ACTION_LED_5_OFF: u32 = 13;
pub const kvLED_ACTION_LED_6_ON: u32 = 14;
pub const kvLED_ACTION_LED_6_OFF: u32 = 15;
pub const kvLED_ACTION_LED_7_ON: u32 = 16;
pub const kvLED_ACTION_LED_7_OFF: u32 = 17;
pub const kvLED_ACTION_LED_8_ON: u32 = 18;
pub const kvLED_ACTION_LED_8_OFF: u32 = 19;
pub const kvLED_ACTION_LED_9_ON: u32 = 20;
pub const kvLED_ACTION_LED_9_OFF: u32 = 21;
pub const kvLED_ACTION_LED_10_ON: u32 = 22;
pub const kvLED_ACTION_LED_10_OFF: u32 = 23;
pub const kvLED_ACTION_LED_11_ON: u32 = 24;
pub const kvLED_ACTION_LED_11_OFF: u32 = 25;

pub const kvENVVAR_TYPE_INT: u32 = 1;
pub const kvENVVAR_TYPE_FLOAT: u32 = 2;
pub const kvENVVAR_TYPE_STRING: u32 = 3;
pub const kvEVENT_TYPE_KEY: u32 = 1;
pub const kvSCRIPT_STOP_NORMAL: u32 = 0;
pub const kvSCRIPT_STOP_FORCED: i32 = -9;
pub const kvSCRIPT_STATUS_LOADED: u32 = 1;
pub const kvSCRIPT_STATUS_RUNNING: u32 = 2;

pub const canTXEDATA_FILE_VERSION: u32 = 1;
pub const canTXEDATA_COMPILER_VERSION: u32 = 2;
pub const canTXEDATA_DATE: u32 = 3;
pub const canTXEDATA_DESCRIPTION: u32 = 4;
pub const canTXEDATA_SOURCE: u32 = 5;
pub const canTXEDATA_SIZE_OF_CODE: u32 = 6;
pub const canTXEDATA_IS_ENCRYPTED: u32 = 7;

pub const kvDEVICE_MODE_INTERFACE: u32 = 0;
pub const kvDEVICE_MODE_LOGGER: u32 = 1;

pub const kvIO_INFO_GET_MODULE_TYPE: u32 = 1;
pub const kvIO_INFO_GET_DIRECTION: u32 = 2;
pub const kvIO_INFO_GET_PIN_TYPE: u32 = 4;
pub const kvIO_INFO_GET_NUMBER_OF_BITS: u32 = 5;
pub const kvIO_INFO_GET_RANGE_MIN: u32 = 6;
pub const kvIO_INFO_GET_RANGE_MAX: u32 = 7;
pub const kvIO_INFO_GET_DI_LOW_HIGH_FILTER: u32 = 8;
pub const kvIO_INFO_GET_DI_HIGH_LOW_FILTER: u32 = 9;
pub const kvIO_INFO_GET_AI_LP_FILTER_ORDER: u32 = 10;
pub const kvIO_INFO_GET_AI_HYSTERESIS: u32 = 11;
pub const kvIO_INFO_GET_MODULE_NUMBER: u32 = 14;
pub const kvIO_INFO_SET_DI_LOW_HIGH_FILTER: u32 = 8;
pub const kvIO_INFO_SET_DI_HIGH_LOW_FILTER: u32 = 9;
pub const kvIO_INFO_SET_AI_LP_FILTER_ORDER: u32 = 10;
pub const kvIO_INFO_SET_AI_HYSTERESIS: u32 = 11;

pub const kvIO_MODULE_TYPE_DIGITAL: u32 = 1;
pub const kvIO_MODULE_TYPE_ANALOG: u32 = 2;
pub const kvIO_MODULE_TYPE_RELAY: u32 = 3;

pub const kvIO_PIN_TYPE_DIGITAL: u32 = 1;
pub const kvIO_PIN_TYPE_ANALOG: u32 = 2;
pub const kvIO_PIN_TYPE_RELAY: u32 = 3;
pub const kvIO_PIN_DIRECTION_IN: u32 = 4;
pub const kvIO_PIN_DIRECTION_OUT: u32 = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum canStatus {
	canOK                  = 0,
	canERR_PARAM           = -1,
	canERR_NOMSG           = -2,
	canERR_NOTFOUND        = -3,
	canERR_NOMEM           = -4,
	canERR_NOCHANELS       = -5,
	canERR_INTERRUPTED     = -6,
	canERR_TIMEOUT         = -7,
	canERR_NOTINITIALIZED  = -8,
	canERR_NOHANDLES       = -9,
	canERR_INVHANDLE       = -10,
	canERR_INIFILE         = -11,
	canERR_DRIVER          = -12,
	canERR_TXBUFOFL        = -13,
	canERR_RESERVED_1      = -14,
	canERR_HARDWARE        = -15,
	canERR_DYNALOAD        = -16,
	canERR_DYNALIB         = -17,
	canERR_DYNAINIT        = -18,
	canERR_NOT_SUPPORTED   = -19,
	canERR_RESERVED_5      = -20, 
	canERR_RESERVED_6      = -21, 
	canERR_RESERVED_2      = -22, 
	canERR_DRIVERLOAD      = -23,
	canERR_DRIVERFAILED    = -24,
	canERR_NOCONFIGMGR     = -25,
	canERR_NOCARD          = -26,
	canERR_RESERVED_7      = -27, 
	canERR_REGISTRY        = -28,
	canERR_LICENSE         = -29,
	canERR_INTERNAL        = -30,
	canERR_NO_ACCESS       = -31,
	canERR_NOT_IMPLEMENTED = -32,
	canERR_DEVICE_FILE     = -33,
	canERR_HOST_FILE       = -34,
	canERR_DISK            = -35,
	canERR_CRC             = -36,
	canERR_CONFIG          = -37,
	canERR_MEMO_FAIL       = -38,
	canERR_SCRIPT_FAIL     = -39,
	canERR_SCRIPT_WRONG_VERSION = -40,
	canERR_SCRIPT_TXE_CONTAINER_VERSION = -41,
	canERR_SCRIPT_TXE_CONTAINER_FORMAT = -42,
	canERR_BUFFER_TOO_SMALL = -43,
	canERR_IO_WRONG_PIN_TYPE = -44,
	canERR_IO_NOT_CONFIRMED = -45,
	canERR_IO_CONFIG_CHANGED = -46,
	canERR_IO_PENDING = -47,
	canERR_IO_NO_VALID_CONFIG = -48,
	canERR__RESERVED       = -49
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct canBusStatistics_s {
	stdData: c_ulong,
	stdRemote: c_ulong,
	extData: c_ulong,
	extRemote: c_ulong,
	errFrame: c_ulong,
	busLoad: c_ulong,
	overruns: c_ulong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kvTimeDomainData_s {
	nMagiSyncGroups: c_int,
	nMagiSyncedMembers: c_int,
	nNonMagiSyncCards: c_int,
	nNonMagicSyncedMembers: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct canUserIoPortData {
	portNo: c_uint,
	portValue: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct canNotifyData {
	tag: *mut c_void,
	eventType: c_int,
	info: info,
} 

#[repr(C)]
#[derive(Copy,Clone)]
pub enum info {
	busErr {
		time: c_ulong,
	},
	rx {
		id: c_long,
		time: c_ulong,
	},
	tx {
		id: c_long,
		time: c_ulong,
	},
	status {
		busStatus: c_uchar,
		txErrorCounter: c_uchar,
		rxErrorCounter: c_uchar,
		time: c_ulong,
	},
}

pub type canHandle = c_int;
pub type CanHandle = canHandle;
pub type canBusStatistics = canBusStatistics_s;
pub type kvTimeDomain = *mut c_void;
pub type kvTimeDomainData = kvTimeDomainData_s;
pub type kvEnvHandle = i64;
pub use self::canStatus as kvStatus;

#[link(name = "canlib32")]
extern "C" {
    pub fn canInitializeLibrary();

	pub fn canUnloadLibrary() -> canStatus;

	pub fn canOpenChannel(channel: c_int, flags: c_int) -> CanHandle;
	
	pub fn canClose(hnd: CanHandle) -> canStatus;

	pub fn canBusOn(hnd: CanHandle) -> canStatus;
	
	pub fn canBusOff(hnd: CanHandle) -> canStatus;
	
	pub fn canSetBusParams(
		hnd: CanHandle,
		freq: c_long,
		tseg1: c_uint,
		tseg2: c_uint,
		sjw: c_uint,
		noSamp: c_uint,
		syncmode: c_uint
	) -> canStatus;

	pub fn canSetBusOutputControl(
		hnd: CanHandle, 
		drivertype: c_uint
	) -> canStatus;

	pub fn canGetBusParams(
		hnd: CanHandle,
		freq: *mut c_long,
		tseg1: *mut c_uint,
		tseg2: *mut c_uint,
		sjw: *mut c_uint,
		noSamp: *mut c_uint,
		syncmode: *mut c_uint,
	) -> canStatus;

	pub fn canGetBusParamsFd(
		hnd: CanHandle,
		freq_brs: *mut c_long,
		tseg1_brs: *mut c_uint,
		tseg2_brs: *mut c_uint,
		sjw_brs: *mut c_uint,
	) -> canStatus;
  
	pub fn canGetBusOutputControl(
		hnd: CanHandle,
		drivertype: *mut c_uint,
	) -> canStatus;
	
	pub fn canWrite(
		hnd: CanHandle,
		id: c_long,
		msg: *mut c_void,
		dlc: c_uint,
		flag: c_uint
	) -> canStatus;

	pub fn canWriteSync(
		hnd: CanHandle, 
		timeout: c_ulong
	) -> canStatus;
 
	pub fn canWriteWait(
		hnd: CanHandle,
		id: c_long,
		msg: *mut c_void,
		dlc: c_uint,
		flag: c_uint,
		timeout: c_ulong,
	) -> canStatus;

	pub fn canRead(
		hnd: CanHandle,
		id: *mut c_long,
		msg: *mut c_void,
		dlc: *mut c_uint,
		flag: *mut c_uint,
		time: *mut c_ulong
	) -> canStatus;

	pub fn canReadWait(
		hnd: CanHandle,
		id: *mut c_long,
		msg: *mut c_void,
		dlc: *mut c_uint,
		flag: *mut c_uint,
		time: *mut c_ulong,
		timeout: *mut c_ulong
	) -> canStatus;

	pub fn canReadSpecific(
		hnd: CanHandle,
		id: c_long,
		msg: *mut c_void,
		dlc: *mut c_uint,
		flag: *mut c_uint,
		time: *mut c_ulong,
	) -> canStatus;

	pub fn canReadStatus(
		hnd: CanHandle, 
		flags: *mut c_ulong
	) -> canStatus;

	pub fn canReadSync(
		hnd: CanHandle, 
		timeout: c_ulong
	) -> canStatus;

	pub fn canReadSyncSpecific(
		hnd: CanHandle,
		id: c_long,
		timeout: c_ulong,
	) -> canStatus;

	pub fn canReadSpecificSkip(
		hnd: CanHandle,
		id: c_long,
		msg: *mut c_void,
		dlc: *mut c_uint,
		flag: *mut c_uint,
		time: *mut c_ulong,
	) -> canStatus;

	pub fn canGetNumberOfChannels(
		channelCount: *mut c_int
	) -> canStatus;
	
	pub fn canGetChannelData(
		channel: c_int,
		item: c_int,
		buffer: *mut c_void,
		bufsize: usize,
	) -> canStatus;

	pub fn canSetNotify(
		hnd: CanHandle,
		callback: Option<unsafe extern "C" fn(arg1: *mut canNotifyData)>,
		notifyFlags: c_uint,
		tag: *mut c_void,
	) -> canStatus;

	pub fn canAccept(
		hnd: CanHandle,
		envelope: c_long,
		flag: c_uint,
	) -> canStatus;   
	
	pub fn canSetAcceptanceFilter(
		hnd: CanHandle,
		code: c_uint,
		mask: c_uint,
		is_extended: c_int,
	) -> canStatus;

	pub fn canRequestBusStatistics(
		hnd: CanHandle
	) -> canStatus;
	
	pub fn canGetBusStatistics(
		hnd: CanHandle,
		stat: *mut canBusStatistics,
		bufsiz: usize,
	) -> canStatus;

	pub fn canGetErrorText(
		err: canStatus,
		buf: *mut c_char,
		bufsiz: c_uint,
	) -> canStatus;
	
	pub fn canReadErrorCounters(
		hnd: CanHandle,
		txErr: *mut c_uint,
		rxErr: *mut c_uint,
		ovErr: *mut c_uint,                        
	) -> canStatus;

	pub fn canGetRawHandle(
		hnd: CanHandle,
		pvFd: *mut c_void
	) -> canStatus;

	pub fn canGetHandleData(
		hnd: CanHandle,
		item: c_int,
		buffer: *mut c_void,
		bufsize: usize,
	) -> canStatus;

	pub fn canTranslateBaud(
		freq: *mut c_long,
		tseg1: *mut c_uint,
		tseg2: *mut c_uint,
		sjw: *mut c_uint,
		nosamp: *mut c_uint,
		syncMode: *mut c_uint,
	) -> canStatus;
	
	pub fn canGetVersion() -> c_ushort;
	
	pub fn canIoCtl(
		hnd: CanHandle,
		func: c_uint,
		buf: *mut c_void,
		buflen: c_uint,
	) -> canStatus;

	pub fn canSetBusParamsC200(
		hnd: CanHandle,
		btr0: c_uchar,
		btr1: c_uchar,
	) -> canStatus;

	pub fn canSetDriverMode(
		hnd: CanHandle,
		lineMode: c_int,
		resNet: c_int,
	) -> canStatus;

	pub fn canGetDriverMode(
		hnd: CanHandle,
		lineMode: *mut c_int,
		resNet: *mut c_int,
	) -> canStatus;
	
	pub fn canGetVersionEx(
		itemCode: c_uint
	) -> c_uint;

	pub fn canObjBufFreeAll(
		hnd: CanHandle
	) -> canStatus;

	pub fn canObjBufAllocate(
		hnd: CanHandle, 
		_type: c_int
	) -> canStatus;

	pub fn canObjBufFree(
		hnd: CanHandle, 
		idx: c_int
	) -> canStatus;

	pub fn canObjBufWrite(
		hnd: CanHandle,
		idx: c_int,
		id: c_int,
		msg: *mut c_void,
		dlc: c_uint,
		flags: c_uint,
	) -> canStatus;

	pub fn canObjBufSetFilter(
		hnd: CanHandle,
		idx: c_int,
		code: c_uint,
		mask: c_uint,
	) -> canStatus;

	pub fn canObjBufSetFlags(
		hnd: CanHandle,
		idx: c_int,
		flags: c_uint,
	) -> canStatus;

	pub fn canObjBufSetPeriod(
		hnd: CanHandle,
		idx: c_int,
		period: c_uint,
	) -> canStatus;

	pub fn canObjBufSetMsgCount(
		hnd: CanHandle,
		idx: c_int,
		count: c_uint,
	) -> canStatus;

	pub fn canObjBufEnable(
		hnd: CanHandle,
		idx: c_int,
	) -> canStatus;

	pub fn canObjBufDisable(
		hnd: CanHandle,
		idx: c_int,
	) -> canStatus;    

	pub fn canObjBufSendBurst(
		hnd: CanHandle,
		idx: c_int,
		burstlen: c_uint,
	) -> canStatus;

	pub fn canResetBus(
		hnd: CanHandle
	) -> canStatus;

	pub fn canFlushReceiveQueue(
		hnd: CanHandle
	) -> canStatus;

	pub fn canFlushTransmitQueue(
		hnd: CanHandle
	) -> canStatus;
}
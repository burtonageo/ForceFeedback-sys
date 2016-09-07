#![allow(overflowing_literals)]

use cf::{CFUUIDGetConstantUUIDWithBytes, CFUUIDRef, E_FAIL, E_INVALIDARG, E_NOTIMPL, E_NOINTERFACE,
         E_OUTOFMEMORY, HRESULT, S_OK, S_FALSE};
use core::ptr;
use MacTypes::{UInt32, SInt32};

pub type DWORD = UInt32;
pub type LPDWORD = *mut DWORD;

pub type LONG = SInt32;
pub type LPLONG = *mut LONG;

pub const FF_INFINITE: DWORD = 0xFFFFFFFF;
pub const FF_DEGREES: DWORD = 100;
pub const FF_FFNOMINALMAX: DWORD = 10_000;
pub const FF_SECONDS: DWORD = 1_000_000;

pub unsafe fn kFFEffectType_ConstantForce_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x60, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_RampForce_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x61, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_Square_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x62, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_Sine_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x63, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_Triangle_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x64, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_SawtoothUp_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x65, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_SawtoothDown_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x66, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_Spring_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x67, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_Damper_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x68, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_Inertia_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x69, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_Friction_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x6A, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub unsafe fn kFFEffectType_CustomForce_ID() -> CFUUIDRef {
    CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
        0xE5, 0x59, 0xC4, 0x6B, 0xC5, 0xCD, 0x11, 0xD6,
        0x8A, 0x1C, 0x00, 0x03, 0x93, 0x53, 0xBD, 0x00)
}

pub const FFEFF_OBJECTOFFSETS: DWORD = 0x00000002;

pub const FFEFF_CARTESIAN: FFCoordinateSystemFlag = 0x00000010;
pub const FFEFF_POLAR: FFCoordinateSystemFlag = 0x00000020;
pub const FFEFF_SPHERICAL: FFCoordinateSystemFlag = 0x00000040;
pub type FFCoordinateSystemFlag = UInt32;

pub const FFEP_DURATION : FFEffectParameterFlag = 0x00000001;
pub const FFEP_SAMPLEPERIOD : FFEffectParameterFlag = 0x00000002;
pub const FFEP_GAIN : FFEffectParameterFlag = 0x00000004;
pub const FFEP_TRIGGERBUTTON : FFEffectParameterFlag = 0x00000008;
pub const FFEP_TRIGGERREPEATINTERVAL : FFEffectParameterFlag = 0x00000010;
pub const FFEP_AXES : FFEffectParameterFlag = 0x00000020;
pub const FFEP_DIRECTION : FFEffectParameterFlag = 0x00000040;
pub const FFEP_ENVELOPE : FFEffectParameterFlag = 0x00000080;
pub const FFEP_TYPESPECIFICPARAMS : FFEffectParameterFlag = 0x00000100;
pub const FFEP_STARTDELAY : FFEffectParameterFlag = 0x00000200;
pub const FFEP_ALLPARAMS : FFEffectParameterFlag = 0x000003FF;
pub const FFEP_START : FFEffectParameterFlag = 0x20000000;
pub const FFEP_NORESTART : FFEffectParameterFlag = 0x40000000;
pub const FFEP_NODOWNLOAD : FFEffectParameterFlag = 0x80000000;
pub const FFEB_NOTRIGGER : FFEffectParameterFlag = 0xFFFFFFFF;
pub type FFEffectParameterFlag = UInt32;

pub const FFES_SOLO: FFEffectStartFlag = 0x00000001;
pub const FFES_NODOWNLOAD: FFEffectStartFlag = 0x80000000;
pub type FFEffectStartFlag = UInt32;

pub const FFEGES_NOTPLAYING: FFEffectStatusFlag = 0x00000000;
pub const FFEGES_PLAYING: FFEffectStatusFlag = 0x00000001;
pub const FFEGES_EMULATED: FFEffectStatusFlag = 0x00000002;
pub type FFEffectStatusFlag = UInt32;

pub const FFSFFC_RESET: FFCommandFlag = 0x00000001;
pub const FFSFFC_STOPALL: FFCommandFlag = 0x00000002;
pub const FFSFFC_PAUSE: FFCommandFlag = 0x00000004;
pub const FFSFFC_CONTINUE: FFCommandFlag = 0x00000008;
pub const FFSFFC_SETACTUATORSON: FFCommandFlag = 0x00000010;
pub const FFSFFC_SETACTUATORSOFF: FFCommandFlag = 0x00000020;
pub type FFCommandFlag = UInt32;

pub const FFGFFS_EMPTY: FFState = 0x00000001;
pub const FFGFFS_STOPPED: FFState = 0x00000002;
pub const FFGFFS_PAUSED: FFState = 0x00000004;
pub const FFGFFS_ACTUATORSON: FFState = 0x00000010;
pub const FFGFFS_ACTUATORSOFF: FFState = 0x00000020;
pub const FFGFFS_POWERON: FFState = 0x00000040;
pub const FFGFFS_POWEROFF: FFState = 0x00000080;
pub const FFGFFS_SAFETYSWITCHON: FFState = 0x00000100;
pub const FFGFFS_SAFETYSWITCHOFF: FFState = 0x00000200;
pub const FFGFFS_USERFFSWITCHON: FFState = 0x00000400;
pub const FFGFFS_USERFFSWITCHOFF: FFState = 0x00000800;
pub const FFGFFS_DEVICELOST: FFState = 0x80000000;
pub type FFState = UInt32;

pub const FFJOFS_X: UInt32 = 0;
pub const FFJOFS_Y: UInt32 = 4;
pub const FFJOFS_Z: UInt32 = 8;
pub const FFJOFS_RX: UInt32 = 2;
pub const FFJOFS_RY: UInt32 = 6;
pub const FFJOFS_RZ: UInt32 = 0;
#[macro_export]
macro_rules! FFJOFS_SLIDER {
    ($n:expr) => (24 + $n * mem::size_of::<LONG>())
}
#[macro_export]
macro_rules! FFJOFS_POV {
    ($n:expr) => (32 + $n * mem::size_of::<DWORD>())
}
#[macro_export]
macro_rules! FFJOFS_BUTTON {
    ($n:expr) => (48 + $n)
}
pub const FFJOFS_BUTTON0: UInt32 = FFJOFS_BUTTON!(0);
pub const FFJOFS_BUTTON1: UInt32 = FFJOFS_BUTTON!(1);
pub const FFJOFS_BUTTON2: UInt32 = FFJOFS_BUTTON!(2);
pub const FFJOFS_BUTTON3: UInt32 = FFJOFS_BUTTON!(3);
pub const FFJOFS_BUTTON4: UInt32 = FFJOFS_BUTTON!(4);
pub const FFJOFS_BUTTON5: UInt32 = FFJOFS_BUTTON!(5);
pub const FFJOFS_BUTTON6: UInt32 = FFJOFS_BUTTON!(6);
pub const FFJOFS_BUTTON7: UInt32 = FFJOFS_BUTTON!(7);
pub const FFJOFS_BUTTON8: UInt32 = FFJOFS_BUTTON!(8);
pub const FFJOFS_BUTTON9: UInt32 = FFJOFS_BUTTON!(9);
pub const FFJOFS_BUTTON10: UInt32 = FFJOFS_BUTTON!(10);
pub const FFJOFS_BUTTON11: UInt32 = FFJOFS_BUTTON!(11);
pub const FFJOFS_BUTTON12: UInt32 = FFJOFS_BUTTON!(12);
pub const FFJOFS_BUTTON13: UInt32 = FFJOFS_BUTTON!(13);
pub const FFJOFS_BUTTON14: UInt32 = FFJOFS_BUTTON!(14);
pub const FFJOFS_BUTTON15: UInt32 = FFJOFS_BUTTON!(15);
pub const FFJOFS_BUTTON16: UInt32 = FFJOFS_BUTTON!(16);
pub const FFJOFS_BUTTON17: UInt32 = FFJOFS_BUTTON!(17);
pub const FFJOFS_BUTTON18: UInt32 = FFJOFS_BUTTON!(18);
pub const FFJOFS_BUTTON19: UInt32 = FFJOFS_BUTTON!(19);
pub const FFJOFS_BUTTON20: UInt32 = FFJOFS_BUTTON!(20);
pub const FFJOFS_BUTTON21: UInt32 = FFJOFS_BUTTON!(21);
pub const FFJOFS_BUTTON22: UInt32 = FFJOFS_BUTTON!(22);
pub const FFJOFS_BUTTON23: UInt32 = FFJOFS_BUTTON!(23);
pub const FFJOFS_BUTTON24: UInt32 = FFJOFS_BUTTON!(24);
pub const FFJOFS_BUTTON25: UInt32 = FFJOFS_BUTTON!(25);
pub const FFJOFS_BUTTON26: UInt32 = FFJOFS_BUTTON!(26);
pub const FFJOFS_BUTTON27: UInt32 = FFJOFS_BUTTON!(27);
pub const FFJOFS_BUTTON28: UInt32 = FFJOFS_BUTTON!(28);
pub const FFJOFS_BUTTON29: UInt32 = FFJOFS_BUTTON!(29);
pub const FFJOFS_BUTTON30: UInt32 = FFJOFS_BUTTON!(30);
pub const FFJOFS_BUTTON31: UInt32 = FFJOFS_BUTTON!(31);

pub const FFPROP_FFGAIN: FFProperty = 1;
pub const FFPROP_AUTOCENTER: FFProperty = 3;
pub type FFProperty = UInt32;

pub const FFSCL_EXCLUSIVE: FFCooperativeLevelFlag = 0x00000001;
pub const FFSCL_NONEXCLUSIVE: FFCooperativeLevelFlag = 0x00000002;
pub const FFSCL_FOREGROUND: FFCooperativeLevelFlag = 0x00000004;
pub const FFSCL_BACKGROUND: FFCooperativeLevelFlag = 0x00000008;
pub type FFCooperativeLevelFlag = UInt32;

pub const FFCAP_ET_CONSTANTFORCE: FFCapabilitiesEffectType = 0x00000001;
pub const FFCAP_ET_RAMPFORCE: FFCapabilitiesEffectType = 0x00000002;
pub const FFCAP_ET_SQUARE: FFCapabilitiesEffectType = 0x00000004;
pub const FFCAP_ET_SINE: FFCapabilitiesEffectType = 0x00000008;
pub const FFCAP_ET_TRIANGLE: FFCapabilitiesEffectType = 0x00000010;
pub const FFCAP_ET_SAWTOOTHUP: FFCapabilitiesEffectType = 0x00000020;
pub const FFCAP_ET_SAWTOOTHDOWN: FFCapabilitiesEffectType = 0x00000040;
pub const FFCAP_ET_SPRING: FFCapabilitiesEffectType = 0x00000080;
pub const FFCAP_ET_DAMPER: FFCapabilitiesEffectType = 0x00000100;
pub const FFCAP_ET_INERTIA: FFCapabilitiesEffectType = 0x00000200;
pub const FFCAP_ET_FRICTION: FFCapabilitiesEffectType = 0x00000400;
pub const FFCAP_ET_CUSTOMFORCE: FFCapabilitiesEffectType = 0x00000800;
pub type FFCapabilitiesEffectType = UInt32;

pub const FFCAP_ST_KINESTHETIC: FFCapabilitiesEffectSubType = 1;
pub const FFCAP_ST_VIBRATION: FFCapabilitiesEffectSubType = 2;
pub type FFCapabilitiesEffectSubType = UInt32;

pub const FF_OK: HRESULT = S_OK;
pub const FF_FALSE: HRESULT = S_FALSE;
pub const FF_DOWNLOADSKIPPED: HRESULT = 0x00000003;
pub const FF_EFFECTRESTARTED: HRESULT = 0x00000004;
pub const FF_TRUNCATED: HRESULT = 0x00000008;
pub const FF_TRUNCATEDANDRESTARTED: HRESULT = 0x0000000C;
pub const FFERR_DEVICENOTREG: HRESULT = 0x80040154; // REGDB_E_CLASSNOTREG
pub const FFERR_INVALIDPARAM: HRESULT = E_INVALIDARG;
pub const FFERR_NOINTERFACE: HRESULT = E_NOINTERFACE;
pub const FFERR_GENERIC: HRESULT = E_FAIL;
pub const FFERR_OUTOFMEMORY: HRESULT = E_OUTOFMEMORY;
pub const FFERR_UNSUPPORTED: HRESULT = E_NOTIMPL;
pub const E_PENDING: HRESULT = 0x8000000A;
pub const FFERR_DEVICEFULL: HRESULT = 0x80040201;
pub const FFERR_MOREDATA: HRESULT = 0x80040202;
pub const FFERR_NOTDOWNLOADED: HRESULT = 0x80040203;
pub const FFERR_HASEFFECTS: HRESULT = 0x80040204;
pub const FFERR_INCOMPLETEEFFECT: HRESULT = 0x80040206;
pub const FFERR_EFFECTPLAYING: HRESULT = 0x80040208;
pub const FFERR_UNPLUGGED: HRESULT = 0x80040209;

pub const FFERR_INVALIDDOWNLOADID: HRESULT = 0x80040300;
pub const FFERR_DEVICEPAUSED: HRESULT = 0x80040301;
pub const FFERR_INTERNAL: HRESULT = 0x80040302;
pub const FFERR_EFFECTTYPEMISMATCH: HRESULT = 0x80040303;
pub const FFERR_UNSUPPORTEDAXIS: HRESULT = 0x80040304;
pub const FFERR_NOTINITIALIZED: HRESULT = 0x80040305;
pub const FFERR_EFFECTTYPENOTSUPPORTED: HRESULT = 0x80040306;
pub const FFERR_DEVICERELEASED: HRESULT = 0x80040307;


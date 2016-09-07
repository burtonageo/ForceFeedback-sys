#[allow(unused_imports)]
use cf::{self, CFUUIDRef, HRESULT, IUNKNOWN_C_GUTS};
use io::io_object_t;
use libc::c_void;
use mach::boolean::boolean_t;
use MacTypes::{finalStage, NumVersion, UInt32};
use ::{FFCAPABILITIES, FFCommandFlag, FFEFFECT, FFEffectParameterFlag, FFEffectStartFlag, FFEffectStatusFlag,
       FFEFFESCAPE, FFProperty};

pub const kFFPlugInAPIMajorRev: UInt32 = 1;
pub const kFFPlugInAPIMinorAndBugRev: UInt32 = 0;
pub const kFFPlugInAPIStage: UInt32 = finalStage;
pub const kFFPlugInAPINonRelRev: UInt32 = 0;

#[macro_export]
macro_rules! kIOForceFeedbackLibTypeID {
    () => ($crate::cf::CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
                       0xF4, 0x54, 0x5C, 0xE5, 0xBF, 0x5B, 0x11, 0xD6,
                       0xA4, 0xBB, 0x00, 0x03, 0x93, 0x3E, 0x3E, 0x3E));
}

#[macro_export]
macro_rules! kIOForceFeedbackDeviceInterfaceID {
    () => ($crate::cf::CFUUIDGetConstantUUIDWithBytes(ptr::null_mut(),
                       0x1C, 0x7C, 0x58, 0x50, 0xBB, 0x6A, 0x11, 0xD6,
                       0xB7, 0x5F, 0x00, 0x30, 0x65, 0xFB, 0xE6, 0xB0));
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ForceFeedbackDeviceState {
    pub dwSize: UInt32,
    pub dwState: UInt32,
    pub dwLoad: UInt32,
}
pub type ForceFeedbackDeviceStatePtr = *mut ForceFeedbackDeviceState;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ForceFeedbackVersion {
    pub apiVersion: NumVersion,
    pub plugInVersion: NumVersion,
}
pub type ForceFeedbackVersionPtr = *mut ForceFeedbackVersion;

pub type FFEffectDownloadID = UInt32;

#[repr(C)]
pub struct IOFORCEFEEDBACKDEVICE_FUNCS_100 {
    pub ForceFeedbackGetVersion: unsafe extern "C" fn(_self: *mut c_void, version: *mut ForceFeedbackVersion) -> HRESULT,
    pub InitializeTerminate: unsafe extern "C" fn(_self: *mut c_void, forceFeedbackAPIVersion: NumVersion,
                                                  hidDevice: io_object_t, begin: boolean_t) -> HRESULT,
    pub DestroyEffect: unsafe extern "C" fn(_self: *mut c_void, id: FFEffectDownloadID) -> HRESULT,
    pub DownloadEffect: unsafe extern "C" fn(_self: *mut c_void, effectType: CFUUIDRef, pDownloadID: *mut FFEffectDownloadID,
                                             pEffect: *mut FFEFFECT, flags: FFEffectParameterFlag) -> HRESULT,
    pub Escape: unsafe extern "C" fn(_self: *mut c_void, downloadID: FFEffectDownloadID, pEscape: *mut FFEFFESCAPE)
                                     -> HRESULT,
    pub GetEffectStatus: unsafe extern "C" fn(_self: *mut c_void, downloadID: FFEffectDownloadID,
                                              pStatusCode: *mut FFEffectStatusFlag) -> HRESULT,
    pub GetForceFeedbackCapabilities: unsafe extern "C" fn(_self: *mut c_void, pCapabilities: *mut FFCAPABILITIES)
                                                           -> HRESULT,
    pub GetForceFeedbackState: unsafe extern "C" fn(_self: *mut c_void, pDeviceState: *mut ForceFeedbackDeviceState)
                                                    -> HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "C" fn(_self: *mut c_void, state: FFCommandFlag) -> HRESULT,
    pub SetProperty: unsafe extern "C" fn(_self: *mut c_void, property: FFProperty, value: *mut c_void) -> HRESULT,
    pub StartEffect: unsafe extern "C" fn(_self: *mut c_void, downloadID: FFEffectDownloadID, mode: FFEffectStartFlag,
                                          iterations: UInt32) -> HRESULT,
    pub StopEffect: unsafe extern "C" fn(_self: *mut c_void, downloadID: FFEffectDownloadID) -> HRESULT,
}

#[repr(C)]
pub struct IOForceFeedbackDeviceInterface {
    pub iunknown: IUNKNOWN_C_GUTS,
    pub ff_device_interface: IOFORCEFEEDBACKDEVICE_FUNCS_100,
}

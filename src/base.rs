use cf::{CFUUIDRef, HRESULT};
use core::default::Default;
use core::mem;
use io::{IOByteCount, io_service_t};
use libc::c_void;
use MacTypes::{finalStage, NumVersion, UInt8, UInt32};
use ::{DWORD, FFCommandFlag, FFCooperativeLevelFlag, FFEffectStatusFlag, FFEffectParameterFlag, FFEffectStartFlag,
       FFProperty, FFState, LONG, LPDWORD, LPLONG};

pub const kFFAPIMajorRev: UInt32 = 1;
pub const kFFAPIMinorAndBugRev: UInt32 = 0;
pub const kFFAPIStage: UInt32 = finalStage;
pub const kFFAPINonRelRev: UInt32 = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FFCONSTANTFORCE {
    pub lMagnitude: LONG,
}

pub type PFFCONSTANTFORCE = *mut FFCONSTANTFORCE;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FFRAMPFORCE {
    pub lStart: LONG,
    pub lEnd: LONG,
}

pub type PFFRAMPFORCE = *mut FFRAMPFORCE;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FFPERIODIC {
    pub dwMagnitude: DWORD,
    pub lOffset: LONG,
    pub dwPhase: DWORD,
    pub dwPeriod: DWORD,
}

pub type PFFPERIODIC = *mut FFPERIODIC;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FFCONDITION {
    pub lOffset: LONG,
    pub lPositiveCoefficient: LONG,
    pub lNegativeCoefficient: LONG,
    pub dwPositiveSaturation: DWORD,
    pub dwNegativeSaturation: DWORD,
    pub lDeadBand: LONG,
}

pub type PFFCONDITION = *mut FFCONDITION;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FFCUSTOMFORCE {
    pub cChannels: DWORD,
    pub dwSamplePeriod: DWORD,
    pub cSamples: DWORD,
    pub rglForceData: LPLONG,
}

impl Default for FFCUSTOMFORCE {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type PFFCUSTOMFORCE = *mut FFCUSTOMFORCE;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FFENVELOPE {
    pub dwSize: DWORD,
    pub dwAttackLevel: DWORD,
    pub dwAttackTime: DWORD,
    pub dwFadeLevel: DWORD,
    pub dwFadeTime: DWORD,

}

pub type PFFENVELOPE = *mut FFENVELOPE;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FFEFFECT {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub dwDuration: DWORD,
    pub dwSamplePeriod: DWORD,
    pub dwGain: DWORD,
    pub dwTriggerButton: DWORD,
    pub dwTriggerRepeatInterval: DWORD,
    pub cAxes: DWORD,
    pub rgdwAxes: LPDWORD,
    pub rglDirection: LPLONG,
    pub lpEnvelope: PFFENVELOPE,
    pub cbTypeSpecificParams: DWORD,
    pub lpvTypeSpecificParams: *mut c_void,
    pub dwStartDelay: DWORD,
}

impl Default for FFEFFECT {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type PFFEFFECT = *mut FFEFFECT;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FFEFFESCAPE {
    pub dwSize: DWORD,
    pub dwCommand: DWORD,
    pub lpvInBuffer: *mut c_void,
    pub cbInBuffer: DWORD,
    pub lpvOutBuffer: *mut c_void,
    pub cbOutBuffer: DWORD,
}

impl Default for FFEFFESCAPE {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type PFFEFFESCAPE = *mut FFEFFESCAPE;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FFCAPABILITIES {
    pub ffSpecVer: NumVersion,
    pub supportedEffects: UInt32,
    pub emulatedEffects: UInt32,
    pub subType: UInt32,
    pub numFfAxes: UInt32,
    pub ffAxes: [UInt8; 32],
    pub storageCapacity: UInt32,
    pub playbackCapacity: UInt32,
    pub firmwareVer: NumVersion,
    pub hardwareVer: NumVersion,
    pub driverVer: NumVersion,
}

pub type PFFCAPABILITIES = *mut FFCAPABILITIES;

#[doc(hidden)]
#[repr(C)]
pub struct __FFDHIDDEN(c_void);

#[doc(hidden)]
#[repr(C)]
pub struct __FFEHIDDEN(c_void);

pub type FFDeviceObjectReference = *mut __FFDHIDDEN;
pub type FFEffectObjectReference = *mut __FFEHIDDEN;

extern "C" {
    pub fn FFCreateDevice(hidDevice: io_service_t, pDeviceReference: *mut FFDeviceObjectReference) -> HRESULT;
    pub fn FFReleaseDevice(deviceReference: FFDeviceObjectReference) -> HRESULT;

    pub fn FFIsForceFeedback(hidDevice: io_service_t) -> HRESULT;

    pub fn FFDeviceCreateEffect(deviceReference: FFDeviceObjectReference, uuidRef: CFUUIDRef,
                                pEffectDefinition: *mut FFEFFECT, pEffectReference: *mut FFEffectObjectReference)
                                -> HRESULT;
    pub fn FFDeviceReleaseEffect(deviceReference: FFDeviceObjectReference, effectReference: FFEffectObjectReference)
                                 -> HRESULT;

    pub fn FFDeviceEscape(deviceReference: FFDeviceObjectReference, pFFEffectEscape: *mut FFEFFESCAPE) -> HRESULT;

    pub fn FFDeviceGetForceFeedbackState(deviceReference: FFDeviceObjectReference, pFFState: *mut FFState) -> HRESULT;
    pub fn FFDeviceSendForceFeedbackCommand(deviceReference: FFDeviceObjectReference, flags: FFCommandFlag) -> HRESULT;

    pub fn FFDeviceSetForceFeedbackProperty(deviceReference: FFDeviceObjectReference, property: FFProperty,
                                            value: *mut c_void) -> HRESULT;
    pub fn FFDeviceGetForceFeedbackProperty(deviceReference: FFDeviceObjectReference, property: FFProperty,
                                            value: *mut c_void, valueSize: IOByteCount) -> HRESULT;

    pub fn FFDeviceSetCooperativeLevel(deviceReference: FFDeviceObjectReference, taskIdentifier: *mut c_void,
                                       flags: FFCooperativeLevelFlag) -> HRESULT;

    pub fn FFDeviceGetForceFeedbackCapabilities(deviceReference: FFDeviceObjectReference,
                                                pFFCapabilities: *mut FFCAPABILITIES) -> HRESULT;

    pub fn FFEffectDownload(effectReference: FFEffectObjectReference) -> HRESULT;
    pub fn FFEffectEscape(effectReference: FFEffectObjectReference, pFFEffectEscape: *mut FFEFFESCAPE) -> HRESULT;
    pub fn FFEffectGetEffectStatus(effectReference: FFEffectObjectReference, pFlags: *mut FFEffectStatusFlag)
                                   -> HRESULT;
    pub fn FFEffectGetParameters(effectReference: FFEffectObjectReference, pFFEffect: *mut FFEFFECT,
                                 flags: FFEffectParameterFlag) -> HRESULT;
    pub fn FFEffectSetParameters(effectReference: FFEffectObjectReference, pFFEffect: *mut FFEFFECT,
                                 flags: FFEffectParameterFlag) -> HRESULT;
    pub fn FFEffectStart(effectReference: FFEffectObjectReference, iterations: UInt32, flags: FFEffectStartFlag)
                         -> HRESULT;
    pub fn FFEffectStop(effectReference: FFEffectObjectReference) -> HRESULT;
    pub fn FFEffectUnload(effectReference: FFEffectObjectReference) -> HRESULT;
}

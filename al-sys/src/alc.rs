/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub const ALC_INVALID: ::std::os::raw::c_int = 0;
pub const ALC_VERSION_0_1: ::std::os::raw::c_int = 1;
pub const ALC_FALSE: ::std::os::raw::c_char = 0;
pub const ALC_TRUE: ::std::os::raw::c_char = 1;
pub const ALC_FREQUENCY: ::std::os::raw::c_int = 4103;
pub const ALC_REFRESH: ::std::os::raw::c_int = 4104;
pub const ALC_SYNC: ::std::os::raw::c_int = 4105;
pub const ALC_MONO_SOURCES: ::std::os::raw::c_int = 4112;
pub const ALC_STEREO_SOURCES: ::std::os::raw::c_int = 4113;
pub const ALC_NO_ERROR: ::std::os::raw::c_int = 0;
pub const ALC_INVALID_DEVICE: ::std::os::raw::c_int = 40961;
pub const ALC_INVALID_CONTEXT: ::std::os::raw::c_int = 40962;
pub const ALC_INVALID_ENUM: ::std::os::raw::c_int = 40963;
pub const ALC_INVALID_VALUE: ::std::os::raw::c_int = 40964;
pub const ALC_OUT_OF_MEMORY: ::std::os::raw::c_int = 40965;
pub const ALC_MAJOR_VERSION: ::std::os::raw::c_int = 4096;
pub const ALC_MINOR_VERSION: ::std::os::raw::c_int = 4097;
pub const ALC_ATTRIBUTES_SIZE: ::std::os::raw::c_int = 4098;
pub const ALC_ALL_ATTRIBUTES: ::std::os::raw::c_int = 4099;
pub const ALC_DEFAULT_DEVICE_SPECIFIER: ::std::os::raw::c_int = 4100;
pub const ALC_DEVICE_SPECIFIER: ::std::os::raw::c_int = 4101;
pub const ALC_EXTENSIONS: ::std::os::raw::c_int = 4102;
pub const ALC_EXT_CAPTURE: ::std::os::raw::c_int = 1;
pub const ALC_CAPTURE_DEVICE_SPECIFIER: ::std::os::raw::c_int = 784;
pub const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: ::std::os::raw::c_int = 785;
pub const ALC_CAPTURE_SAMPLES: ::std::os::raw::c_int = 786;
pub const ALC_ENUMERATE_ALL_EXT: ::std::os::raw::c_int = 1;
pub const ALC_DEFAULT_ALL_DEVICES_SPECIFIER: ::std::os::raw::c_int = 4114;
pub const ALC_ALL_DEVICES_SPECIFIER: ::std::os::raw::c_int = 4115;
pub struct ALCdevice_struct { _priv: () }
pub type ALCdevice = ALCdevice_struct;
pub struct ALCcontext_struct { _priv: () }
pub type ALCcontext = ALCcontext_struct;
pub type ALCboolean = ::std::os::raw::c_char;
pub type ALCchar = ::std::os::raw::c_char;
pub type ALCbyte = ::std::os::raw::c_char;
pub type ALCubyte = ::std::os::raw::c_uchar;
pub type ALCshort = ::std::os::raw::c_short;
pub type ALCushort = ::std::os::raw::c_ushort;
pub type ALCint = ::std::os::raw::c_int;
pub type ALCuint = ::std::os::raw::c_uint;
pub type ALCsizei = ::std::os::raw::c_int;
pub type ALCenum = ::std::os::raw::c_int;
pub type ALCfloat = f32;
pub type ALCdouble = f64;
pub type ALCvoid = ::std::os::raw::c_void;
pub type LPALCCREATECONTEXT =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               attrlist: *const ALCint)
                              -> *mut ALCcontext>;
pub type LPALCMAKECONTEXTCURRENT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)
                              -> ALCboolean>;
pub type LPALCPROCESSCONTEXT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCSUSPENDCONTEXT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCDESTROYCONTEXT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCGETCURRENTCONTEXT =
    ::std::option::Option<extern "C" fn() -> *mut ALCcontext>;
pub type LPALCGETCONTEXTSDEVICE =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)
                              -> *mut ALCdevice>;
pub type LPALCOPENDEVICE =
    ::std::option::Option<unsafe extern "C" fn(devicename: *const ALCchar)
                              -> *mut ALCdevice>;
pub type LPALCCLOSEDEVICE =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)
                              -> ALCboolean>;
pub type LPALCGETERROR =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)
                              -> ALCenum>;
pub type LPALCISEXTENSIONPRESENT =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               extname: *const ALCchar)
                              -> ALCboolean>;
pub type LPALCGETPROCADDRESS =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               funcname: *const ALCchar)
                              -> *mut ::std::os::raw::c_void>;
pub type LPALCGETENUMVALUE =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               enumname: *const ALCchar)
                              -> ALCenum>;
pub type LPALCGETSTRING =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               param: ALCenum)
                              -> *const ALCchar>;
pub type LPALCGETINTEGERV =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               param: ALCenum, size: ALCsizei,
                                               values: *mut ALCint)>;
pub type LPALCCAPTUREOPENDEVICE =
    ::std::option::Option<unsafe extern "C" fn(devicename: *const ALCchar,
                                               frequency: ALCuint,
                                               format: ALCenum,
                                               buffersize: ALCsizei)
                              -> *mut ALCdevice>;
pub type LPALCCAPTURECLOSEDEVICE =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)
                              -> ALCboolean>;
pub type LPALCCAPTURESTART =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCCAPTURESTOP =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCCAPTURESAMPLES =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               buffer: *mut ALCvoid,
                                               samples: ALCsizei)>;

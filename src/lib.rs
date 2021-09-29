pub mod ffi;

use crate::ffi::*;
use std::fmt::Formatter;

pub struct FMODError(FMOD_RESULT);

impl std::fmt::Debug for FMODError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FMOD error: {:?}", self.0)
    }
}

pub fn wrap(result: FMOD_RESULT) -> Result<(), FMODError>{
    if result == FMOD_RESULT::FMOD_OK {
        Ok(())
    } else {
        Err(FMODError(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serial_test::serial;

    #[test]
    #[serial]
    fn init() {
        unsafe {
            let mut system = std::ptr::null_mut::<FMOD_SYSTEM>();

            wrap(FMOD_System_Create(std::ptr::addr_of_mut!(system), FMOD_VERSION)).unwrap();

            wrap(FMOD_System_Init(system, 8, FMOD_INIT_NORMAL, std::ptr::null_mut())).unwrap();
            wrap(FMOD_System_Close(system)).unwrap();

            wrap(FMOD_System_Release(system)).unwrap();
        }
    }

    #[test]
    #[serial]
    fn init_studio() {
        unsafe {
            let mut system = std::ptr::null_mut::<FMOD_SYSTEM>();
            let mut studio_system = std::ptr::null_mut::<FMOD_STUDIO_SYSTEM>();

            wrap(FMOD_System_Create(std::ptr::addr_of_mut!(system), FMOD_VERSION)).unwrap();
            wrap(FMOD_System_Init(system, 8, FMOD_INIT_NORMAL, std::ptr::null_mut())).unwrap();

            wrap(FMOD_Studio_System_Create(std::ptr::addr_of_mut!(studio_system), FMOD_VERSION)).unwrap();

            wrap(FMOD_Studio_System_Release(studio_system)).unwrap();

            wrap(FMOD_System_Close(system)).unwrap();
            wrap(FMOD_System_Release(system)).unwrap();
        }
    }

    #[test]
    #[serial]
    fn play_sound() {
        unsafe {
            let filename = std::ffi::CString::new("assets/drumloop.wav").unwrap();

            let mut system = std::ptr::null_mut::<FMOD_SYSTEM>();
            let mut sound = std::ptr::null_mut::<FMOD_SOUND>();
            let mut channel = std::ptr::null_mut::<FMOD_CHANNEL>();

            wrap(FMOD_System_Create(std::ptr::addr_of_mut!(system), FMOD_VERSION)).unwrap();
            wrap(FMOD_System_Init(system, 32, FMOD_INIT_NORMAL, std::ptr::null_mut())).unwrap();
            wrap(FMOD_System_CreateSound(system, filename.as_ptr(), FMOD_DEFAULT, std::ptr::null_mut(), std::ptr::addr_of_mut!(sound))).unwrap();
            wrap(FMOD_Sound_SetMode(sound, FMOD_LOOP_OFF)).unwrap();

            wrap(FMOD_System_PlaySound(system, sound, std::ptr::null_mut(), 0, std::ptr::addr_of_mut!(channel))).unwrap();

            let mut playing = 1;
            while playing != 0 {
                FMOD_System_Update(system);
                match wrap(FMOD_Channel_IsPlaying(channel, std::ptr::addr_of_mut!(playing))) {
                    Ok(()) => Ok(()),
                    Err(result) => match result.0 {
                        FMOD_RESULT::FMOD_ERR_INVALID_HANDLE | FMOD_RESULT::FMOD_ERR_CHANNEL_STOLEN => Ok(()),
                        _ => Err(result),
                    }
                }.unwrap();
                std::thread::sleep(std::time::Duration::from_millis(50));
            }

            FMOD_Sound_Release(sound);
            FMOD_System_Close(system);
            FMOD_System_Release(system);
        }
    }
}

//
// #[repr(i32)]
// #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
// pub enum FMODError {
//     BADCOMMAND = 1,
//     CHANNEL_ALLOC = 2,
//     CHANNEL_STOLEN = 3,
//     DMA = 4,
//     DSP_CONNECTION = 5,
//     DSP_DONTPROCESS = 6,
//     DSP_FORMAT = 7,
//     DSP_INUSE = 8,
//     DSP_NOTFOUND = 9,
//     DSP_RESERVED = 10,
//     DSP_SILENCE = 11,
//     DSP_TYPE = 12,
//     FILE_BAD = 13,
//     FILE_COULDNOTSEEK = 14,
//     FILE_DISKEJECTED = 15,
//     FILE_EOF = 16,
//     FILE_ENDOFDATA = 17,
//     FILE_NOTFOUND = 18,
//     FORMAT = 19,
//     HEADER_MISMATCH = 20,
//     HTTP = 21,
//     HTTP_ACCESS = 22,
//     HTTP_PROXY_AUTH = 23,
//     HTTP_SERVER_ERROR = 24,
//     HTTP_TIMEOUT = 25,
//     INITIALIZATION = 26,
//     INITIALIZED = 27,
//     INTERNAL = 28,
//     INVALID_FLOAT = 29,
//     INVALID_HANDLE = 30,
//     INVALID_PARAM = 31,
//     INVALID_POSITION = 32,
//     INVALID_SPEAKER = 33,
//     INVALID_SYNCPOINT = 34,
//     INVALID_THREAD = 35,
//     INVALID_VECTOR = 36,
//     MAXAUDIBLE = 37,
//     MEMORY = 38,
//     MEMORY_CANTPOINT = 39,
//     NEEDS3D = 40,
//     NEEDSHARDWARE = 41,
//     NET_CONNECT = 42,
//     NET_SOCKET_ERROR = 43,
//     NET_URL = 44,
//     NET_WOULD_BLOCK = 45,
//     NOTREADY = 46,
//     OUTPUT_ALLOCATED = 47,
//     OUTPUT_CREATEBUFFER = 48,
//     OUTPUT_DRIVERCALL = 49,
//     OUTPUT_FORMAT = 50,
//     OUTPUT_INIT = 51,
//     OUTPUT_NODRIVERS = 52,
//     PLUGIN = 53,
//     PLUGIN_MISSING = 54,
//     PLUGIN_RESOURCE = 55,
//     PLUGIN_VERSION = 56,
//     RECORD = 57,
//     REVERB_CHANNELGROUP = 58,
//     REVERB_INSTANCE = 59,
//     SUBSOUNDS = 60,
//     SUBSOUND_ALLOCATED = 61,
//     SUBSOUND_CANTMOVE = 62,
//     TAGNOTFOUND = 63,
//     TOOMANYCHANNELS = 64,
//     TRUNCATED = 65,
//     UNIMPLEMENTED = 66,
//     UNINITIALIZED = 67,
//     UNSUPPORTED = 68,
//     VERSION = 69,
//     EVENT_ALREADY_LOADED = 70,
//     EVENT_LIVEUPDATE_BUSY = 71,
//     EVENT_LIVEUPDATE_MISMATCH = 72,
//     EVENT_LIVEUPDATE_TIMEOUT = 73,
//     EVENT_NOTFOUND = 74,
//     STUDIO_UNINITIALIZED = 75,
//     STUDIO_NOT_LOADED = 76,
//     INVALID_STRING = 77,
//     ALREADY_LOCKED = 78,
//     NOT_LOCKED = 79,
//     RECORD_DISCONNECTED = 80,
//     TOOMANYSAMPLES = 81,
// }
//
// impl From<FMOD_RESULT> for FMODError {
//     fn from(result: FMOD_RESULT) -> Self {
//         match result {
//             FMOD_RESULT::FMOD_OK | FMOD_RESULT::FMOD_RESULT_FORCEINT => panic!(),
//             _ => result as i32 as FMODError,
//         }
//     }
// }
//
//
// fn to_result(result: FMOD_RESULT) -> Result<(), FMODError>{
//     if result == FMOD_RESULT::FMOD_OK {
//         Ok(())
//     } else {
//         Err(result)
//     }
// }
//
// pub struct System {
//     system: *mut FMOD_SYSTEM,
// }
//
// impl System {
//     pub fn new() -> Result<Self, FMODError> {
//         let mut system = std::ptr::null_mut::<FMOD_SYSTEM>();
//         unsafe {
//             to_result(FMOD_System_Create(std::ptr::addr_of_mut!(system), FMOD_VERSION))?;
//         }
//         if system.is_null() {
//             panic!("FMOD failed to allocate System");
//         }
//         Ok(Self {
//             system
//         })
//     }
//
//     pub fn init(maxchannels: i32, flags: FMODInitFlags, extradriverdata: &ExtraDriverData) -> Result<(), FMODError> {
//         maxchanne
//     }
// }
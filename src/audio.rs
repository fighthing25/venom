use windows::core::Interface;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{
    eCapture, eCommunications, eMultimedia, eRender, MMDeviceEnumerator, IMMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};

pub struct AudioManager {
    pub com_initialized: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        let hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        Self {
            com_initialized: hr.is_ok(),
        }
    }

    fn get_enumerator() -> Option<IMMDeviceEnumerator> {
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok() }
    }

    pub fn toggle_mic_mute() -> bool {
        if let Some(enumerator) = Self::get_enumerator() {
            unsafe {
                if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eCapture, eCommunications) {
                    if let Ok(endpoint) = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) {
                        if let Ok(is_muted) = endpoint.GetMute() {
                            let new_state = !is_muted.as_bool();
                            let _ = endpoint.SetMute(new_state, std::ptr::null());
                            return new_state;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_mic_muted() -> bool {
        if let Some(enumerator) = Self::get_enumerator() {
            unsafe {
                if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eCapture, eCommunications) {
                    if let Ok(endpoint) = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) {
                        if let Ok(is_muted) = endpoint.GetMute() {
                            return is_muted.as_bool();
                        }
                    }
                }
            }
        }
        false
    }

    pub fn get_master_volume() -> f32 {
        if let Some(enumerator) = Self::get_enumerator() {
            unsafe {
                if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
                    if let Ok(endpoint) = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) {
                        if let Ok(level) = endpoint.GetMasterVolumeLevelScalar() {
                            return level;
                        }
                    }
                }
            }
        }
        0.5
    }

    pub fn set_master_volume(level: f32) {
        let clamped = level.clamp(0.0, 1.0);
        if let Some(enumerator) = Self::get_enumerator() {
            unsafe {
                if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
                    if let Ok(endpoint) = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) {
                        let _ = endpoint.SetMasterVolumeLevelScalar(clamped, std::ptr::null());
                    }
                }
            }
        }
    }

    pub fn step_master_volume(delta: f32) -> f32 {
        let current = Self::get_master_volume();
        let target = (current + delta).clamp(0.0, 1.0);
        Self::set_master_volume(target);
        target
    }
}
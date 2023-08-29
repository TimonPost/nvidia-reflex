use ash::vk::{self, Device, Handle, Semaphore, SemaphoreType};
use ffi::*;

mod ffi;

// Note: You need to define HANDLE, NVLL_VK_LATENCY_MARKER_PARAMS, and NVLL_VK_LATENCY_RESULT_PARAMS types properly.

fn init_low_latency_device(vk_device: Device) {
    let mut semaphore = Semaphore::null();
    unsafe { ffi::NvLL_VK_InitLowLatencyDevice(vk_device.as_raw() as HANDLE, semaphor) }
}

pub fn set_latency_marker(vk_device: vk::Device, frame_id: u64, marker_type: PclStatsEvent) {
    let params = ffi::NVLL_VK_LATENCY_MARKER_PARAMS {
        frameID: frame_id,
        markerType: marker_type as u8,
    };

    unsafe {
        ffi::NvLL_VK_SetLatencyMarker(vk_device.as_raw() as HANDLE, params);
    }
}

// pub fn get_latency(vk_device: vk::Device) {
//     unsafe {
//         let mut latency_result_params = NVLL_VK_LATENCY_RESULT_PARAMS {};

//         ffi::NvLL_VK_GetLatency(vk_device.as_raw() as HANDLE, &mut latency_result_params);
//         // Use latency_result_params to access latency results
//     }
// }

pub fn define_and_init_pclstats() {
    unsafe {
        ffi::pclstats_define();
        ffi::pclstats_init(0); // Pass appropriate flags here
    }
}

pub fn mark_pclstats(frame_id: u64, marker_type: PclStatsLatencyMarkerType) {
    unsafe {
        pclstats_marker(marker_type as u8, frame_id);
    }
}

pub fn is_ping_pclstats(msg_id: UINT) -> bool {
    unsafe { ffi::pclstats_is_ping_msg_id(msg_id) != 0 }
}

pub fn shutdown_pclstats() {
    unsafe {
        ffi::pclstats_shutdown();
    }
}

fn main() {
    let vk_device: ffi::HANDLE = std::ptr::null_mut(); // Example vkDevice handle
    let mut signal_semaphore_handle: ffi::HANDLE = std::ptr::null_mut();

    unsafe {
        ffi::NvLL_VK_InitLowLatencyDevice(vk_device, &mut signal_semaphore_handle);
    }
}

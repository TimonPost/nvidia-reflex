use std::os::raw::{c_ulonglong, c_void};

pub type HANDLE = *mut c_void;
pub type NvVKU64 = c_ulonglong;
pub type UINT = u32;
pub type UINT64 = u64;

#[repr(C)]
pub struct NVLL_VK_LATENCY_MARKER_PARAMS {
    frameID: u64,
    markerType: u8,
}

#[repr(C)]
pub struct NVLL_VK_LATENCY_RESULT_PARAMS {
  
}

pub enum PclStatsEvent {
    K_SIMULATION_START = 0,
    VK_SIMULATION_END = 1,
    VK_RENDERSUBMIT_START = 2,
    VK_RENDERSUBMIT_END = 3,
    VK_PRESENT_START = 4,
    VK_PRESENT_END = 5,
    VK_INPUT_SAMPLE = 6,
    VK_TRIGGER_FLASH = 7,
}

pub enum PclStatsLatencyMarkerType {
    PCLSTATS_SIMULATION_START = 0,
    PCLSTATS_SIMULATION_END = 1,
    PCLSTATS_RENDERSUBMIT_START = 2,
    PCLSTATS_RENDERSUBMIT_END = 3,
    PCLSTATS_PRESENT_START = 4,
    PCLSTATS_PRESENT_END = 5,
    PCLSTATS_INPUT_SAMPLE = 6,
    PCLSTATS_TRIGGER_FLASH = 7,
    PCLSTATS_PC_LATENCY_PING = 8,
}

// Declare FFI functions
extern "C" {
    // Init api
    pub fn NvLL_VK_InitLowLatencyDevice(vkDevice: HANDLE, signalSemaphoreHandle: *mut HANDLE);

    // The time api
    pub fn NvLL_VK_SetLatencyMarker(
        vkDevice: HANDLE,
        pSetLatencyMarkerParams: *const NVLL_VK_LATENCY_MARKER_PARAMS,
    );
    pub fn NvLL_VK_GetLatency(
        vkDevice: HANDLE,
        pGetLatencyResultParams: *mut NVLL_VK_LATENCY_RESULT_PARAMS,
    );

    pub fn pclstats_define();
    pub fn pclstats_init(flags: UINT);
    pub fn pclstats_marker(markerType: u8, frameID: UINT64);
    pub fn pclstats_is_ping_msg_id(msgId: UINT) -> std::os::raw::c_int;
    pub fn pclstats_shutdown();
}

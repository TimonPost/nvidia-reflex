// pclstats_wrapper.cpp
#include "pclstats.h"  // Include the PCL Stats header

/// 1. To setup PCL Stats, first place the PCLSTATS_DEFINE macro in a .cpp file at global scope. 
/// 2. Call the PCLSTATS_INIT macro one time in some application setup code to initialize the PCL 
/// Stats code. As for the parameters, for the basic mode using custom messages  and utilizing PCL 
/// Stats Present markers, use flags = 0. 
/// 3. Call the PCLSTATS_SHUTDOWN macro once in application cleanup code to end PCL Stats 
/// functionality. 
/// (In between the calls to PCLSTATS_INIT and PCLSTATS_SHUTDOWN, the application will be 
/// receiving custom ping messages at random intervals that are each roughly 100-300ms apart.) 
/// The timings of these ping messages are recorded and are used to measure the latency 
/// between the simulated input and the application picking up the input . 


/// 1. You will start by supplying the START / END markers to the 
/// driver via PCLSTATS_MARKER and NvAPI_D3D_SetLatencyMarker / NvLL_VK_SetLatencyMarker 
/// each frame. This enables the driver to provide a full simulate -to-GPU-Render latency result struct back 
/// Reflex Integration 
/// to the application, via per-stage timestamps, on a per-frame basis. These APIs are designed to simplify 
/// submission and collection of these timestamps across multiple concurrently executing threads and 
/// frames, and also to collect them in a unified timespace between the CPU and GPU hardware.


extern "C" {
    void pclstats_define() {
        PCLSTATS_DEFINE();
    }

    void pclstats_init(UINT flags) {
        PCLSTATS_INIT(flags);
    }

    void pclstats_marker(PCLSTATS_LATENCY_MARKER_TYPE markerType, UINT64 frameID) {
        PCLSTATS_MARKER(markerType, frameID);
    }

    int pclstats_is_ping_msg_id(UINT msgId) {
        return PCLSTATS_IS_PING_MSG_ID(msgId);
    }

    void pclstats_shutdown() {
        PCLSTATS_SHUTDOWN();
    }
}
//! C-ABI and JNI Foreign Function Interface for oxide-mesh-net
//!
//! Enables zero-overhead embedding in Android (`VpnService`), iOS (`NEPacketTunnelProvider`),
//! desktop wrappers, and C/C++ clients.

use libc::c_char;
use oxide_daemon::{DaemonConfig, DaemonEngine};
use oxide_transport::routing::L1DirectMappedCache;
use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

/// FFI Error Codes
pub const OXIDE_OK: i32 = 0;
pub const OXIDE_ERR_NULL_PTR: i32 = -1;
pub const OXIDE_ERR_INVALID_CONFIG: i32 = -2;
pub const OXIDE_ERR_PACKET_DROPPED: i32 = -3;
pub const OXIDE_ERR_NO_ROUTE: i32 = -4;

/// Opaque FFI Context
pub struct OxideEngineContext {
    pub engine: Arc<DaemonEngine>,
    pub runtime: Runtime,
    pub l1_cache: Mutex<L1DirectMappedCache>,
}

/// Create a new Oxide Engine instance from a JSON configuration string
///
/// # Safety
/// `config_json` must be a valid null-terminated C string or null for default configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_engine_create(
    config_json: *const c_char,
) -> *mut OxideEngineContext {
    let config = if config_json.is_null() {
        DaemonConfig::default()
    } else {
        let c_str = unsafe { CStr::from_ptr(config_json) };
        match c_str.to_str() {
            Ok(s) => match serde_json::from_str::<DaemonConfig>(s) {
                Ok(cfg) => cfg,
                Err(_) => return std::ptr::null_mut(),
            },
            Err(_) => return std::ptr::null_mut(),
        }
    };

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return std::ptr::null_mut(),
    };

    let engine = Arc::new(DaemonEngine::new(config));
    let ctx = Box::new(OxideEngineContext {
        engine,
        runtime,
        l1_cache: Mutex::new(L1DirectMappedCache::new()),
    });

    Box::into_raw(ctx)
}

/// Destroy an Oxide Engine instance
///
/// # Safety
/// `ctx` must be a valid pointer returned from `oxide_engine_create`, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_engine_destroy(ctx: *mut OxideEngineContext) {
    if !ctx.is_null() {
        unsafe {
            drop(Box::from_raw(ctx));
        }
    }
}

/// Process an inbound or outbound IP packet through the data plane pipeline
///
/// Returns:
/// - `OXIDE_OK` (0) on successful route decision, copying the 32-byte target node ID into `out_target_node_id` (if non-null).
/// - `OXIDE_ERR_NO_ROUTE` (-4) if destination has no match in Radix table.
/// - `OXIDE_ERR_PACKET_DROPPED` (-3) if packet was dropped by SIMD ACL or malformed.
/// - `OXIDE_ERR_NULL_PTR` (-1) if invalid pointers provided.
///
/// # Safety
/// Pointers must be valid and `packet_data` must point to at least `packet_len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_process_packet(
    ctx: *mut OxideEngineContext,
    packet_data: *mut u8,
    packet_len: usize,
    out_target_node_id: *mut u8,
) -> i32 {
    if ctx.is_null() || packet_data.is_null() || packet_len == 0 {
        return OXIDE_ERR_NULL_PTR;
    }

    let context = unsafe { &*ctx };
    let slice = unsafe { std::slice::from_raw_parts_mut(packet_data, packet_len) };

    let mut l1 = match context.l1_cache.lock() {
        Ok(guard) => guard,
        Err(_) => return OXIDE_ERR_PACKET_DROPPED,
    };

    let route_opt = context
        .runtime
        .block_on(context.engine.process_tun_packet(slice, &mut l1));

    match route_opt {
        Some(target) => {
            if !out_target_node_id.is_null() {
                let id_bytes = target.node_id.as_bytes();
                unsafe {
                    std::ptr::copy_nonoverlapping(id_bytes.as_ptr(), out_target_node_id, 32);
                }
            }
            OXIDE_OK
        }
        None => OXIDE_ERR_NO_ROUTE,
    }
}

/// Query current daemon status as a JSON string
///
/// # Safety
/// Returned string must be freed with `oxide_free_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_get_status(ctx: *mut OxideEngineContext) -> *mut c_char {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }

    let context = unsafe { &*ctx };
    let status_json = serde_json::json!({
        "node_id": context.engine.config().node_id.to_string(),
        "tun_name": context.engine.config().tun_name,
        "mtu": context.engine.config().mtu,
        "overlay_ipv4": context.engine.config().overlay_ipv4.to_string(),
        "overlay_ipv6": context.engine.config().overlay_ipv6.to_string(),
        "status": "ready"
    });

    let s = status_json.to_string();
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string returned by the Oxide FFI
///
/// # Safety
/// `s` must be a valid pointer returned by an Oxide FFI function, or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

// =========================================================================
// Android JNI Bindings (for integration with Kotlin / VpnService)
// =========================================================================

/// JNI Bridge: Create Oxide Native Context
///
/// # Safety
/// `config_json` must be a valid C string pointer or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_oxide_mesh_OxideVpnService_createNative(
    _env: *mut libc::c_void,
    _class: *mut libc::c_void,
    config_json: *const c_char,
) -> *mut OxideEngineContext {
    unsafe { oxide_engine_create(config_json) }
}

/// JNI Bridge: Destroy Oxide Native Context
///
/// # Safety
/// `ctx` must be a valid pointer to OxideEngineContext or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_oxide_mesh_OxideVpnService_destroyNative(
    _env: *mut libc::c_void,
    _class: *mut libc::c_void,
    ctx: *mut OxideEngineContext,
) {
    unsafe { oxide_engine_destroy(ctx) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_lifecycle_and_status() {
        unsafe {
            let ctx = oxide_engine_create(std::ptr::null());
            assert!(!ctx.is_null());

            let status_ptr = oxide_get_status(ctx);
            assert!(!status_ptr.is_null());

            let c_str = CStr::from_ptr(status_ptr);
            let s = c_str.to_str().unwrap();
            assert!(s.contains("overlay_ipv4"));
            assert!(s.contains("100.64.0.42"));

            oxide_free_string(status_ptr);
            oxide_engine_destroy(ctx);
        }
    }

    #[test]
    fn test_ffi_null_safety() {
        unsafe {
            assert_eq!(
                oxide_process_packet(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut()
                ),
                OXIDE_ERR_NULL_PTR
            );
            assert!(oxide_get_status(std::ptr::null_mut()).is_null());
            oxide_free_string(std::ptr::null_mut());
            oxide_engine_destroy(std::ptr::null_mut());
        }
    }
}

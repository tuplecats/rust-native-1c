use std::ptr::NonNull;
use crate::types::Variant;

struct IConnectorVTable {
    _drop: unsafe extern "C" fn(&mut IConnector),
    add_error: unsafe extern "C" fn(&mut IConnector, u16, *const u16, *const u16, i64) -> bool,
    read: unsafe extern "C" fn(&mut IConnector, *const u16, *Variant, *const u64, *const *const u16) -> bool,
    write: unsafe extern "C" fn(&mut IConnector, *const u16, *Variant) -> bool,
    register_profile_as: unsafe extern "C" fn(&mut IConnector, *const u16) -> bool,
    set_event_buffer_depths: unsafe extern "C" fn(&mut IConnector, u64) -> bool,
    get_event_buffer_depths: unsafe extern "C" fn(&mut IConnector) -> u64,
    external_event: unsafe extern "C" fn(&mut IConnector, *const u16, *const u16, *const u16) -> bool,
    clean_event_buffer: unsafe extern "C" fn(&mut IConnector),
    set_status_line: unsafe extern "C" fn(&mut IConnector, *const u16) -> bool,
    reset_status_line: unsafe extern "C" fn(&mut IConnector),
}

struct IConnector {
    vtable: NonNull<IConnectorVTable>
}

impl IConnector {
    fn add_error(code: u16, source: &str, descr: &str, scode: u64) -> bool {
        true
    }
}
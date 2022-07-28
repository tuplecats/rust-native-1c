use std::ffi::c_void;
use std::ptr::NonNull;
use widestring::U16CStr;
use crate::memory_manager;
use crate::types::Variant;

#[repr(C)]
struct IConnectorVTable {
    _drop: unsafe extern "C" fn(&mut IConnector),
    add_error: unsafe extern "C" fn(&mut IConnector, u16, *const u16, *const u16, i64) -> bool,
    read: unsafe extern "C" fn(&mut IConnector, *const u16, *const Variant, *mut u64, *const *const u16) -> bool,
    write: unsafe extern "C" fn(&mut IConnector, *const u16, *const Variant) -> bool,
    register_profile_as: unsafe extern "C" fn(&mut IConnector, *const u16) -> bool,
    set_event_buffer_depths: unsafe extern "C" fn(&mut IConnector, u64) -> bool,
    get_event_buffer_depths: unsafe extern "C" fn(&mut IConnector) -> u64,
    external_event: unsafe extern "C" fn(&mut IConnector, *const u16, *const u16, *const u16) -> bool,
    clean_event_buffer: unsafe extern "C" fn(&mut IConnector),
    set_status_line: unsafe extern "C" fn(&mut IConnector, *const u16) -> bool,
    reset_status_line: unsafe extern "C" fn(&mut IConnector),
}

#[repr(C)]
pub struct IConnector {
    vtable: NonNull<IConnectorVTable>
}

impl IConnector {
    pub fn add_error(&mut self, code: u16, source: &str, descr: &str, scode: i64) -> bool {
        let source = memory_manager().alloc_utf16_str(source);
        let descr = memory_manager().alloc_utf16_str(descr);
        unsafe { (self.vtable.as_mut().add_error)(self, code, source, descr, scode) }
    }

    pub fn read(&mut self, prop_name: &str, value: &mut Variant, error: &mut u64, error_description: &mut String) -> bool {
        let prop_name = memory_manager().alloc_utf16_str(prop_name);
        let value = value as *mut Variant;
        let error = error as *mut u64;
        let mut error_description_ptr = std::ptr::null();

        let result = unsafe { (self.vtable.as_mut().read)(self, prop_name, value, error, &error_description_ptr) };
        if !error_description_ptr.is_null() {
            *error_description = unsafe { U16CStr::from_ptr_str(error_description_ptr).to_string().unwrap() };
            memory_manager().free_memory((&mut error_description_ptr as *mut *const u16) as *mut *const c_void);
        }
        result
    }

    pub fn write(&mut self, prop_name: &str, value: &Variant) -> bool {
        let prop_name = memory_manager().alloc_utf16_str(prop_name);
        unsafe { (self.vtable.as_mut().write)(self, prop_name, value as *const Variant) }
    }

    pub fn register_profile_as(&mut self, profile_name: &str) -> bool {
        let profile_name = memory_manager().alloc_utf16_str(profile_name);
        unsafe { (self.vtable.as_mut().register_profile_as)(self, profile_name) }
    }

    pub fn set_event_buffer_depths(&mut self, depths: u64) -> bool {
        unsafe { (self.vtable.as_mut().set_event_buffer_depths)(self, depths) }
    }

    pub fn get_event_buffer_depths(&mut self) -> u64 {
        unsafe { (self.vtable.as_mut().get_event_buffer_depths)(self) }
    }

    pub fn external_event(&mut self, source: &str, message: &str, data: &str) -> bool {
        let source = memory_manager().alloc_utf16_str(source);
        let message = memory_manager().alloc_utf16_str(message);
        let data = memory_manager().alloc_utf16_str(data);
        unsafe { (self.vtable.as_mut().external_event)(self, source, message, data) }
    }

    pub fn clear_event_buffer(&mut self) {
        unsafe { (self.vtable.as_mut().clean_event_buffer)(self) }
    }

    pub fn set_status_line(&mut self, message: &str) -> bool {
        let message = memory_manager().alloc_utf16_str(message);
        unsafe { (self.vtable.as_mut().set_status_line)(self, message) }
    }

    pub fn reset_status_line(&mut self) {
        unsafe { (self.vtable.as_mut().reset_status_line)(self) }
    }


}
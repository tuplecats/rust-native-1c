use std::ffi::{c_void, c_long};
use std::ptr::NonNull;
use crate::memory::IMemoryManager;
use crate::types::Variant;
use crate::connector::IConnector;

#[repr(C)]
pub struct IInitDoneBaseVTable<T> {
    #[cfg(target_os = "linux")]
    pub offset_linux: u64,
    pub drop: unsafe extern "stdcall" fn(&mut T),
    pub init: unsafe extern "stdcall" fn(&mut T, *mut c_void) -> bool,
    pub set_mem_manager: unsafe extern "stdcall" fn(&mut T, *mut c_void) -> bool,
    pub get_info: unsafe extern "stdcall" fn(&T) -> c_long,
    pub done: unsafe extern "stdcall" fn(&mut T),
}

#[repr(C)]
pub struct ILanguageExtenderBaseVTable<T> {
    #[cfg(target_os = "linux")]
    pub offset_linux: u64,
    pub drop: unsafe extern "stdcall" fn(&mut T),
    pub register_extension_as: unsafe extern "stdcall" fn(&mut T, *mut *const u16) -> bool,
    pub get_n_props: unsafe extern "stdcall" fn(&mut T) -> c_long,
    pub find_prop: unsafe extern "stdcall" fn(&mut T, *const u16) -> c_long,
    pub get_prop_name: unsafe extern "stdcall" fn(&T, c_long, c_long) -> *const u16,
    pub get_prop_val: unsafe extern "stdcall" fn(&mut T, c_long, *mut Variant) -> bool,
    pub set_prop_val: unsafe extern "stdcall" fn(&mut T, c_long, *mut Variant) -> bool,
    pub is_prop_readable: unsafe extern "stdcall" fn(&mut T, c_long) -> bool,
    pub is_prop_writeable: unsafe extern "stdcall" fn(&mut T, c_long) -> bool,
    pub get_n_methods: unsafe extern "stdcall" fn(&mut T) -> c_long,
    pub find_method: unsafe extern "stdcall" fn(&mut T, *const u16) -> c_long,
    pub get_method_name: unsafe extern "stdcall" fn(&T, c_long, c_long) -> *const u16,
    pub get_n_params: unsafe extern "stdcall" fn(&mut T, c_long) -> c_long,
    pub get_param_def_value: unsafe extern "stdcall" fn(&mut T, c_long, c_long, *mut Variant) -> bool,
    pub has_ret_val: unsafe extern "stdcall" fn(&mut T, c_long) -> bool,
    pub call_as_proc: unsafe extern "stdcall" fn(&mut T, c_long, *mut Variant, c_long) -> bool,
    pub call_as_func: unsafe extern "stdcall" fn(&mut T, c_long, *mut Variant, *mut Variant, c_long) -> bool,
}

#[repr(C)]
pub struct LocaleBaseVTable<T> {
    #[cfg(target_os = "linux")]
    pub offset_linux: u64,
    pub drop: unsafe extern "stdcall" fn(&mut T),
    pub set_locale: unsafe extern "stdcall" fn(&mut T, *const u16)
}

#[repr(C)]
pub struct IComponentBaseVTable<T> {
    pub init_done_base_vtable: NonNull<IInitDoneBaseVTable<T>>,
    pub language_extension_vtable: NonNull<ILanguageExtenderBaseVTable<T>>,
    pub locale_base_vtable: NonNull<LocaleBaseVTable<T>>,
}

pub trait IComponentInit where Self: IComponentBase {
    fn set_mem_manager(&mut self, mem: *mut c_void) -> bool;

    fn mem_manager(&self) -> &mut IMemoryManager;

    fn connector(&self) -> &mut IConnector;

    fn set_connector(&mut self, connector: *mut c_void) -> bool;

    fn _init(&mut self, connector: *mut c_void) -> bool {
        if self.set_connector(connector) {
            return IComponentBase::init(self)
        }
        return false
    }

    fn register_extension_as(&mut self, name: *mut *const u16) -> bool;
}

pub trait IComponentBase {
    // IInitDoneBaseVTable
    fn init(&mut self) -> bool;
    fn get_info(&self) -> c_long;
    fn done(&mut self);

    // ILanguageExtenderBaseVTable
    fn get_n_props(&self) -> c_long;
    fn find_prop(&self, prop_name: &str) -> c_long;
    fn get_prop_name(&self, prop_num: c_long, prop_alias: c_long) -> &str;
    fn get_prop_val(&self, prop_num: c_long, var_prop_val: &mut Variant) -> bool;
    fn set_prop_val(&mut self, prop_num: c_long, var_prop_val: &Variant) -> bool;
    fn is_prop_readable(&self, prop_num: c_long) -> bool;
    fn is_prop_writeable(&self, prop_num: c_long) -> bool;
    fn get_n_methods(&self) -> c_long;
    fn find_method(&self, method_name: &str) -> c_long;
    fn get_method_name(&self, method_num: c_long, method_alias: c_long) -> &str;
    fn get_n_params(&self, method_num: c_long) -> c_long;
    fn get_param_def_value(&self, method_num: c_long, param_num: c_long, var_param_def_value: &mut Variant) -> bool;
    fn has_ret_val(&self, method_num: c_long) -> bool;
    fn call_as_proc(&mut self, method_num: c_long, params: Option<&mut [Variant]>) -> bool;
    fn call_as_func(&mut self, method_num: c_long, ret_vals: &mut Variant, params: Option<&mut [Variant]>) -> bool;

    // LocaleBaseVTable
    fn set_locale(&mut self, loc: &str);
}

#[allow(non_camel_case_types)]
#[repr(i8)]
pub enum AppCapabilities {
    #[allow(unused)] eAppCapabilitiesInvalid = -1,
    #[allow(unused)] eAppCapabilities1 = 1,
}
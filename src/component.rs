use std::ffi::c_void;
use std::ptr::NonNull;
use crate::IConnector;
use crate::memory::IMemoryManager;
use crate::types::Variant;

#[repr(C)]
pub struct IInitDoneBaseVTable<T> {
    pub drop: unsafe extern "C" fn(&mut T),
    pub init: unsafe extern "C" fn(&mut T, *mut c_void) -> bool,
    pub set_mem_manager: unsafe extern "C" fn(&mut T, *mut c_void) -> bool,
    pub get_info: unsafe extern "C" fn(&T) -> i64,
    pub done: unsafe extern "C" fn(&mut T),
}

#[repr(C)]
pub struct ILanguageExtenderBaseVTable<T> {
    pub drop: unsafe extern "C" fn(&mut T),
    pub register_extension_as: unsafe extern "C" fn(&mut T, *mut *const u16) -> bool,
    pub get_n_props: unsafe extern "C" fn(&mut T) -> i64,
    pub find_prop: unsafe extern "C" fn(&mut T, *const u16) -> i64,
    pub get_prop_name: unsafe extern "C" fn(&T, i64, i64) -> *const u16,
    pub get_prop_val: unsafe extern "C" fn(&mut T, i64, *mut Variant) -> bool,
    pub set_prop_val: unsafe extern "C" fn(&mut T, i64, *mut Variant) -> bool,
    pub is_prop_readable: unsafe extern "C" fn(&mut T, i64) -> bool,
    pub is_prop_writeable: unsafe extern "C" fn(&mut T, i64) -> bool,
    pub get_n_methods: unsafe extern "C" fn(&mut T) -> i64,
    pub find_method: unsafe extern "C" fn(&mut T, *const u16) -> i64,
    pub get_method_name: unsafe extern "C" fn(&T, i64, i64) -> *const u16,
    pub get_n_params: unsafe extern "C" fn(&mut T, i64) -> i64,
    pub get_param_def_value: unsafe extern "C" fn(&mut T, i64, i64, *mut Variant) -> bool,
    pub has_ret_val: unsafe extern "C" fn(&mut T, i64) -> bool,
    pub call_as_proc: unsafe extern "C" fn(&mut T, i64, *mut Variant, usize) -> bool,
    pub call_as_func: unsafe extern "C" fn(&mut T, i64, *mut Variant, *mut Variant, usize) -> bool,
}

#[repr(C)]
pub struct LocaleBaseVTable<T> {
    pub drop: unsafe extern "C" fn(&mut T),
    pub set_locale: unsafe extern "C" fn(&mut T, *const u16)
}

#[repr(C)]
pub struct IComponentBaseVTable<T> {
    pub init_done_base_vtable: NonNull<IInitDoneBaseVTable<T>>,
    pub language_extension_vtable: NonNull<ILanguageExtenderBaseVTable<T>>,
    pub locale_base_vtable: NonNull<LocaleBaseVTable<T>>,
}

pub trait IComponentInit where Self: IComponentBase {
    fn set_mem_manager(&mut self, mem: *mut c_void) -> bool {
        crate::set_memory_manager(mem as *mut IMemoryManager)
    }

    fn _init(&mut self, connector: *mut c_void) -> bool {
        if crate::set_connector(connector as *mut IConnector) {
            return IComponentBase::init(self)
        }
        return false
    }

    fn register_extension_as(&mut self, name: *mut *const u16) -> bool;
}

pub trait IComponentBase {
    // IInitDoneBaseVTable
    fn init(&mut self) -> bool;
    fn get_info(&self) -> i64;
    fn done(&mut self);

    // ILanguageExtenderBaseVTable
    fn get_n_props(&self) -> i64;
    fn find_prop(&self, prop_name: &str) -> i64;
    fn get_prop_name(&self, prop_num: i64, prop_alias: i64) -> &str;
    fn get_prop_val(&self, prop_num: i64, var_prop_val: &mut Variant) -> bool;
    fn set_prop_val(&mut self, prop_num: i64, var_prop_val: &Variant) -> bool;
    fn is_prop_readable(&self, prop_num: i64) -> bool;
    fn is_prop_writeable(&self, prop_num: i64) -> bool;
    fn get_n_methods(&self) -> i64;
    fn find_method(&self, method_name: &str) -> i64;
    fn get_method_name(&self, method_num: i64, method_alias: i64) -> &str;
    fn get_n_params(&self, method_num: i64) -> i64;
    fn get_param_def_value(&self, method_num: i64, param_num: i64, var_param_def_value: &mut Variant) -> bool;
    fn has_ret_val(&self, method_num: i64) -> bool;
    fn call_as_proc(&mut self, method_num: i64, params: Option<&mut [Variant]>) -> bool;
    fn call_as_func(&mut self, method_num: i64, ret_vals: &mut Variant, params: Option<&mut [Variant]>) -> bool;

    // LocaleBaseVTable
    fn set_locale(&mut self, loc: &str);
}

#[allow(non_camel_case_types)]
#[repr(i8)]
pub enum AppCapabilities {
    #[allow(unused)] eAppCapabilitiesInvalid = -1,
    #[allow(unused)] eAppCapabilities1 = 1,
}
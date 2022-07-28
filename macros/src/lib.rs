use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn native_object(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as ItemStruct);

    let ident = ast.ident.clone();
    let vis = ast.vis.clone();

    match &mut ast.fields {
        syn::Fields::Named(fields) => {
            fields.named
                .insert(0, syn::Field::parse_named.parse2(quote! { inner: Inner }).unwrap());
        }
        _ => {
            ()
        }
    }

    let base_done_vtable_name = quote::format_ident!("{}_BASE_DONE_VTABLE", ident);
    let language_ext_vtable_name = quote::format_ident!("{}_LANGUAGE_EXT_VTABLE", ident);
    let locale_vtable_name = quote::format_ident!("{}_LOCALE_VTABLE", ident);
    let module_name = quote::format_ident!("{}_native_object_mod", ident);
    let ident_str = ident.to_string();

    let data = quote! {

        #[allow(non_upper_case_globals)]
        static #base_done_vtable_name: IInitDoneBaseVTable<#ident> = IInitDoneBaseVTable {
            drop: {
                unsafe extern "C" fn drop<T: IComponentBase>(_0: &mut T) {

                }
                drop::<#ident>
            },
            init: {
                unsafe extern "C" fn init<T: IComponentBase>(_0: &mut T, disp: *const c_void) -> bool {
                    _0.init(disp)
                }
                init::<#ident>
            },
            set_mem_manager: {
                unsafe extern "C" fn set_mem_manager<T: IComponentInit>(_0: &mut T, mem: *mut c_void) -> bool {
                    _0.set_mem_manager(mem)
                }
                set_mem_manager::<#ident>
            },
            get_info: {
                unsafe extern "C" fn get_info<T: IComponentBase>(_0: &T) -> i64 {
                    _0.get_info()
                }
                get_info::<#ident>
            },
            done: {
                unsafe extern "C" fn done<T: IComponentBase>(_0: &mut T) {
                    _0.done();
                }
                done::<#ident>
            }
        };

        #[allow(non_upper_case_globals)]
        static #language_ext_vtable_name: ILanguageExtenderBaseVTable<#ident> = ILanguageExtenderBaseVTable {
            drop: {
                unsafe extern "C" fn drop<T: IComponentBase>(_0: &mut T) {

                }
                drop::<#ident>
            },
            register_extension_as: {
                unsafe extern "C" fn register_extension_as<T: IComponentInit>(_0: &mut T, name: *mut *const u16) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    _0.register_extension_as(name)
                }
                register_extension_as::<#ident>
            },
            get_n_props: {
                unsafe extern "C" fn get_n_props<T: IComponentBase>(_0: &mut T) -> i64 {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    _0.get_n_props()
                }
                get_n_props::<#ident>
            },
            find_prop: {
                unsafe extern "C" fn find_prop<T: IComponentBase>(_0: &mut T, name: *const u16) -> i64 {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    let prop_name = unsafe { widestring::U16CStr::from_ptr_str(name).to_string().unwrap() };
                    _0.find_prop(prop_name.as_str())
                }
                find_prop::<#ident>
            },
            get_prop_name: {
                unsafe extern "C" fn get_prop_name<T: IComponentBase + IComponentInit>(_0: &T, num: i64, alias: i64) -> *const u16 {
                    let _0 = &*((((_0 as *const T) as *const u8) as usize - 8) as *const T);
                    let prop_name = _0.get_prop_name(num, alias);
                    _0.mem_manager().alloc_utf16_str(prop_name)
                }
                get_prop_name::<#ident>
            },
            get_prop_val: {
                unsafe extern "C" fn get_prop_val<T: IComponentBase>(_0: &mut T, num: i64, value: *mut Variant) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    let value = &mut *value;
                    _0.get_prop_val(num, value)
                }
                get_prop_val::<#ident>
            },
            set_prop_val: {
                unsafe extern "C" fn set_prop_val<T: IComponentBase>(_0: &mut T, num: i64, value: *mut Variant) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    let value = &*value;
                    _0.set_prop_val(num, value)
                }
                set_prop_val::<#ident>
            },
            is_prop_readable: {
                unsafe extern "C" fn is_prop_readable<T: IComponentBase>(_0: &mut T, num: i64) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    _0.is_prop_readable(num)
                }
                is_prop_readable::<#ident>
            },
            is_prop_writeable: {
                unsafe extern "C" fn is_prop_writeable<T: IComponentBase>(_0: &mut T, num: i64) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    _0.is_prop_writeable(num)
                }
                is_prop_writeable::<#ident>
            },
            get_n_methods: {
                unsafe extern "C" fn get_n_methods<T: IComponentBase>(_0: &mut T) -> i64 {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    _0.get_n_methods()
                }
                get_n_methods::<#ident>
            },
            find_method: {
                unsafe extern "C" fn find_method<T: IComponentBase>(_0: &mut T, name: *const u16) -> i64 {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    let method_name = unsafe { widestring::U16CStr::from_ptr_str(name).to_string().unwrap() };
                    _0.find_method(method_name.as_str())
                }
                find_method::<#ident>
            },
            get_method_name: {
                unsafe extern "C" fn get_method_name<T: IComponentBase + IComponentInit>(_0: &T, num: i64, alias: i64) -> *const u16 {
                    let _0 = &*((((_0 as *const T) as *const u8) as usize - 8) as *const T);
                    let method_name = _0.get_method_name(num, alias);
                    _0.mem_manager().alloc_utf16_str(method_name)
                }
                get_method_name::<#ident>
            },
            get_n_params: {
                unsafe extern "C" fn get_n_params<T: IComponentBase>(_0: &mut T, num: i64) -> i64 {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    _0.get_n_params(num)
                }
                get_n_params::<#ident>
            },
            get_param_def_value: {
                unsafe extern "C" fn get_param_def_value<T: IComponentBase>(_0: &mut T, num: i64, param_num: i64, value: *mut Variant) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    let value = &mut *value;
                    _0.get_param_def_value(num, param_num, value)
                }
                get_param_def_value::<#ident>
            },
            has_ret_val: {
                unsafe extern "C" fn has_ret_val<T: IComponentBase>(_0: &mut T, num: i64) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    _0.has_ret_val(num)
                }
                has_ret_val::<#ident>
            },
            call_as_proc: {
                unsafe extern "C" fn call_as_proc<T: IComponentBase>(_0: &mut T, num: i64, params: *mut Variant, size: usize) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    let params = match params.is_null() {
                        true => None,
                        false => Some(std::slice::from_raw_parts_mut(params, (&mut *params).cb_elements as usize))
                    };
                    _0.call_as_proc(num, params, size)
                }
                call_as_proc::<#ident>
            },
            call_as_func: {
                unsafe extern "C" fn call_as_func<T: IComponentBase>(_0: &mut T, num: i64, ret: *mut Variant, params: *mut Variant, size: usize) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 8) as *mut T);
                    let params = match params.is_null() {
                        true => None,
                        false => Some(std::slice::from_raw_parts_mut(params, (&mut *params).cb_elements as usize))
                    };
                    let ret = &mut *ret;

                    _0.call_as_func(num, ret, params, size)
                }
                call_as_func::<#ident>
            }
        };

        #[allow(non_upper_case_globals)]
        static #locale_vtable_name: LocaleBaseVTable<#ident> = LocaleBaseVTable {
            drop: {
                unsafe extern "C" fn drop<T: IComponentBase>(_0: &mut T) {

                }
                drop::<#ident>
            },
            set_locale: {
                unsafe extern "C" fn set_locale<T: IComponentBase>(_0: &mut T, loc: *const u16) {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - 16) as *mut T);
                    let locale = unsafe { widestring::U16CStr::from_ptr_str(loc).to_string().unwrap() };
                    _0.set_locale(locale.as_str());
                }
                set_locale::<#ident>
            }
        };

        #[repr(C)]
        struct Inner {
            vtable: IComponentBaseVTable<#ident>,
            mem_manager: *mut IMemoryManager,
        }

        impl IComponentInit for #ident {
            fn mem_manager(&self) -> &mut IMemoryManager {
                unsafe { &mut *self.inner.mem_manager }
            }

            fn set_mem_manager(&mut self, manager: *mut c_void) -> bool {
                self.inner.mem_manager = manager as *mut IMemoryManager;
                true
            }

            fn register_extension_as(&mut self, name: *mut *const u16) -> bool {
                unsafe { *name = self.mem_manager().alloc_utf16_str(#ident_str); }
                true
            }
        }

        impl Default for Inner {
            fn default() -> Self {
                Inner {
                    vtable: IComponentBaseVTable {
                        init_done_base_vtable: NonNull::from(&#base_done_vtable_name),
                        language_extension_vtable: NonNull::from(&#language_ext_vtable_name),
                        locale_base_vtable: NonNull::from(&#locale_vtable_name),
                    },
                    mem_manager: std::ptr::null_mut(),
                }
            }
        }

    };

    let result = quote! {

        #[allow(non_snake_case)]
        #[macro_use]
        mod #module_name {

            use std::ptr::NonNull;
            use std::ffi::c_void;
            use ::native_1c::component::*;
            use ::native_1c::memory::*;
            use ::native_1c::types::*;
            use ::native_1c::Derivative;
            use ::native_1c::widestring;

            #[derive(Derivative)]
            #[derivative(Default)]
            #ast

            #data
        }

        #[macro_use]
        #vis use #module_name::*;
    };

    result.into()
}
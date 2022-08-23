use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input, Visibility, VisPublic};
use syn::parse::Parser;

#[proc_macro_attribute]
pub fn native_object(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as ItemStruct);

    let ident = ast.ident.clone();
    let vis = ast.vis.clone();

    match &mut ast.fields {
        syn::Fields::Named(fields) => {
            fields.named.iter_mut()
                .for_each(|field| field.vis = Visibility::Public(VisPublic { pub_token: Default::default() }));

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
            #[cfg(target_os = "linux")]
            offset_linux: 0,
            drop: {
                unsafe extern "system" fn drop<T: IComponentBase>(_0: &mut T) {

                }
                drop::<#ident>
            },
            init: {
                unsafe extern "system" fn init<T: IComponentInit>(_0: &mut T, disp: *mut c_void) -> bool {
                    _0._init(disp)
                }
                init::<#ident>
            },
            set_mem_manager: {
                unsafe extern "system" fn set_mem_manager<T: IComponentInit>(_0: &mut T, mem: *mut c_void) -> bool {
                    _0.set_mem_manager(mem)
                }
                set_mem_manager::<#ident>
            },
            get_info: {
                unsafe extern "system" fn get_info<T: IComponentBase>(_0: &T) -> std::os::raw::c_long {
                    _0.get_info()
                }
                get_info::<#ident>
            },
            done: {
                unsafe extern "system" fn done<T: IComponentBase>(_0: &mut T) {
                    _0.done();
                }
                done::<#ident>
            }
        };

        #[allow(non_upper_case_globals)]
        static #language_ext_vtable_name: ILanguageExtenderBaseVTable<#ident> = ILanguageExtenderBaseVTable {
            #[cfg(target_os = "linux")]
            offset_linux: 0,
            drop: {
                unsafe extern "system" fn drop<T: IComponentBase>(_0: &mut T) {

                }
                drop::<#ident>
            },
            register_extension_as: {
                unsafe extern "system" fn register_extension_as<T: IComponentInit>(_0: &mut T, name: *mut *const u16) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    _0.register_extension_as(name)
                }
                register_extension_as::<#ident>
            },
            get_n_props: {
                unsafe extern "system" fn get_n_props<T: IComponentBase>(_0: &mut T) -> std::os::raw::c_long {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    _0.get_n_props()
                }
                get_n_props::<#ident>
            },
            find_prop: {
                unsafe extern "system" fn find_prop<T: IComponentBase>(_0: &mut T, name: *const u16) -> std::os::raw::c_long {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    let prop_name = unsafe { widestring::U16CStr::from_ptr_str(name).to_string().unwrap() };
                    _0.find_prop(prop_name.as_str())
                }
                find_prop::<#ident>
            },
            get_prop_name: {
                unsafe extern "system" fn get_prop_name<T: IComponentBase + IComponentInit>(_0: &T, num: std::os::raw::c_long, alias: std::os::raw::c_long) -> *const u16 {
                    let _0 = &*((((_0 as *const T) as *const u8) as usize - std::mem::size_of::<usize>()) as *const T);
                    let prop_name = _0.get_prop_name(num, alias);
                    _0.mem_manager().copy_utf16_str(prop_name)
                }
                get_prop_name::<#ident>
            },
            get_prop_val: {
                unsafe extern "system" fn get_prop_val<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long, value: *mut Variant) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    let value = &mut *value;
                    _0.get_prop_val(num, value)
                }
                get_prop_val::<#ident>
            },
            set_prop_val: {
                unsafe extern "system" fn set_prop_val<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long, value: *mut Variant) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    let value = &*value;
                    _0.set_prop_val(num, value)
                }
                set_prop_val::<#ident>
            },
            is_prop_readable: {
                unsafe extern "system" fn is_prop_readable<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    _0.is_prop_readable(num)
                }
                is_prop_readable::<#ident>
            },
            is_prop_writeable: {
                unsafe extern "system" fn is_prop_writeable<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    _0.is_prop_writeable(num)
                }
                is_prop_writeable::<#ident>
            },
            get_n_methods: {
                unsafe extern "system" fn get_n_methods<T: IComponentBase>(_0: &mut T) -> std::os::raw::c_long {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    _0.get_n_methods()
                }
                get_n_methods::<#ident>
            },
            find_method: {
                unsafe extern "system" fn find_method<T: IComponentBase>(_0: &mut T, name: *const u16) -> std::os::raw::c_long {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    let method_name = unsafe { widestring::U16CStr::from_ptr_str(name).to_string().unwrap() };
                    _0.find_method(method_name.as_str())
                }
                find_method::<#ident>
            },
            get_method_name: {
                unsafe extern "system" fn get_method_name<T: IComponentBase + IComponentInit>(_0: &T, num: std::os::raw::c_long, alias: std::os::raw::c_long) -> *const u16 {
                    let _0 = &*((((_0 as *const T) as *const u8) as usize - std::mem::size_of::<usize>()) as *const T);
                    let method_name = _0.get_method_name(num, alias);
                    _0.mem_manager().copy_utf16_str(method_name)
                }
                get_method_name::<#ident>
            },
            get_n_params: {
                unsafe extern "system" fn get_n_params<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long) -> std::os::raw::c_long {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    _0.get_n_params(num)
                }
                get_n_params::<#ident>
            },
            get_param_def_value: {
                unsafe extern "system" fn get_param_def_value<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long, param_num: std::os::raw::c_long, value: *mut Variant) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    let value = &mut *value;
                    _0.get_param_def_value(num, param_num, value)
                }
                get_param_def_value::<#ident>
            },
            has_ret_val: {
                unsafe extern "system" fn has_ret_val<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    _0.has_ret_val(num)
                }
                has_ret_val::<#ident>
            },
            call_as_proc: {
                unsafe extern "system" fn call_as_proc<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long, params: *mut Variant, size: std::os::raw::c_long) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    let params = match params.is_null() {
                        true => None,
                        false => Some(std::slice::from_raw_parts_mut(params, size as usize))
                    };
                    _0.call_as_proc(num, params)
                }
                call_as_proc::<#ident>
            },
            call_as_func: {
                unsafe extern "system" fn call_as_func<T: IComponentBase>(_0: &mut T, num: std::os::raw::c_long, ret: *mut Variant, params: *mut Variant, size: std::os::raw::c_long) -> bool {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()) as *mut T);
                    let params = match params.is_null() {
                        true => None,
                        false => Some(std::slice::from_raw_parts_mut(params, size as usize))
                    };
                    let ret = &mut *ret;

                    _0.call_as_func(num, ret, params)
                }
                call_as_func::<#ident>
            }
        };

        #[allow(non_upper_case_globals)]
        static #locale_vtable_name: LocaleBaseVTable<#ident> = LocaleBaseVTable {
            #[cfg(target_os = "linux")]
            offset_linux: 0,
            drop: {
                unsafe extern "system" fn drop<T: IComponentBase>(_0: &mut T) {

                }
                drop::<#ident>
            },
            set_locale: {
                unsafe extern "system" fn set_locale<T: IComponentBase>(_0: &mut T, loc: *const u16) {
                    let _0 = &mut *((((_0 as *mut T) as *mut u8) as usize - std::mem::size_of::<usize>()*2) as *mut T);
                    let locale = unsafe { widestring::U16CStr::from_ptr_str(loc).to_string().unwrap() };
                    _0.set_locale(locale.as_str());
                }
                set_locale::<#ident>
            }
        };

        #[repr(C)]
        struct Inner {
            vtable: IComponentBaseVTable<#ident>,
            memory_manager: RefCell<*mut IMemoryManager>,
            connector1c: RefCell<*mut IConnector>,
        }

        impl #ident {
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl IComponentInit for #ident {
            fn register_extension_as(&mut self, name: *mut *const u16) -> bool {
                unsafe { *name = self.mem_manager().copy_utf16_str(#ident_str); }
                true
            }

            fn set_mem_manager(&mut self, mem: *mut c_void) -> bool {
                self.inner.memory_manager = RefCell::new(mem as *mut IMemoryManager);
                true
            }

            fn mem_manager(&self) -> &mut IMemoryManager {
                unsafe { self.inner.memory_manager.borrow_mut().as_mut().unwrap() }
            }

            fn connector(&self) -> &mut IConnector {
                unsafe { self.inner.connector1c.borrow_mut().as_mut().unwrap() }
            }

            fn set_connector(&mut self, connector: *mut c_void) -> bool {
                self.inner.connector1c = RefCell::new(connector as *mut IConnector);
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
                    memory_manager: RefCell::new(std::ptr::null_mut()),
                    connector1c: RefCell::new(std::ptr::null_mut()),
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
            use std::cell::RefCell;
            use ::native_1c::component::*;
            use ::native_1c::memory::*;
            use ::native_1c::connector::*;
            use ::native_1c::types::*;
            use ::native_1c::Derivative;
            use ::native_1c::widestring;
            use super::*;

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
#![feature(let_else)]
use native_1c::OnceCell;
use native_1c::component::AppCapabilities;
use native_1c::widestring::{U16CStr, U16CString};
use crate::addin_new::TestAddIn;

mod addin_new;

/*
#[native_object]
#[repr(C)]
pub struct TestAddInB {
    pub string_prop: String,

    #[derivative(Default(value = "1"))]
    pub num_prop: i32,
}

impl IComponentBase for TestAddInB {
    fn init(&mut self) -> bool {
        true
    }

    fn get_info(&self) -> i64 {
        1000
    }

    fn done(&mut self) {

    }

    fn get_n_props(&self) -> i64 {
        2
    }

    fn find_prop(&self, prop_name: &str) -> i64 {
        match prop_name {
            "ТестовоеСвойство" | "TestProp" => 0,
            "СтроковоеСвойство" | "StringProp" => 1,
            _ => unreachable!()
        }
    }

    fn get_prop_name(&self, prop_num: i64, prop_alias: i64) -> &str {
        match prop_num {
            0 => {
                if prop_alias == 0 { "TestProp" } else { "ТестовоеСвойство" }
            },
            1 => {
                if prop_alias == 0 { "StringProp" } else { "СтроковоеСвойство" }
            }
            _ => unreachable!()
        }
    }

    fn get_prop_val(&self, prop_num: i64, var_prop_val: &mut Variant) -> bool {
        match prop_num {
            0 => {
                *var_prop_val = Variant::from(self.num_prop);
            },
            1 => {
                *var_prop_val = Variant::utf8_string(self, self.string_prop.as_str());
            }
            _ => return false
        }
        true
    }

    fn set_prop_val(&mut self, prop_num: i64, var_prop_val: &Variant) -> bool {
        match prop_num {
            0 => {
                self.num_prop = var_prop_val.as_i32().unwrap();
            },
            1 => {
                self.string_prop = var_prop_val.as_string().unwrap();
            },
            _ => return false,
        }

        true
    }

    fn is_prop_readable(&self, _prop_num: i64) -> bool {
        true
    }

    fn is_prop_writeable(&self, _prop_num: i64) -> bool {
        true
    }

    fn get_n_methods(&self) -> i64 {
        2
    }

    fn find_method(&self, method_name: &str) -> i64 {
        match method_name {
            "МетодКомпоненты" | "ComponentMethod" => 0,
            "ПроцедураКомпоненты" | "ComponentProcedure" => 1,
            _ => unreachable!()
        }
    }

    fn get_method_name(&self, method_num: i64, method_alias: i64) -> &str {
        match method_num {
            0 => {
                if method_alias == 0 { "ComponentMethod" } else { "МетодКомпоненты" }
            },
            1 => {
                if method_alias == 0 { "ComponentProcedure" } else { "ПроцедураКомпоненты" }
            }
            _ => unreachable!()
        }
    }

    fn get_n_params(&self, method_num: i64) -> i64 {
        match method_num {
            1 => 2,
            _ => 0
        }
    }

    fn get_param_def_value(&self, method_num: i64, param_num: i64, var_param_def_value: &mut Variant) -> bool {
        match method_num {
            1 if param_num == 0 => {
                *var_param_def_value = Variant::utf8_string(self, "Привет");
            },
            1 if param_num == 1 => {
                *var_param_def_value = Variant::utf16_string(self, "Привет2");
            },
            _ => return false
        }
        true
    }

    fn has_ret_val(&self, method_num: i64) -> bool {
        match method_num {
            0 => true,
            _ => false
        }
    }

    fn call_as_proc(&mut self, method_num: i64, params: Option<&mut [Variant]>) -> bool {
        match method_num {
            1 => {
                self.string_prop = params.unwrap()[0].as_string().unwrap();
            },
            _ => return false
        }
        true
    }

    fn call_as_func(&mut self, method_num: i64, ret_vals: &mut Variant, _params: Option<&mut [Variant]>) -> bool {
        match method_num {
            0 => {
                *ret_vals = Variant::utf8_string(self, self.string_prop.as_str());
            },
            _ => return false,
        }
        true
    }

    fn set_locale(&mut self, _loc: &str) { }
}
*/

static CLASS_NAMES: OnceCell<Vec<u16>> = OnceCell::new();

#[no_mangle]
unsafe extern "C" fn GetClassObject(_name: *const u16, component: *mut *const u8) -> usize {
    let Ok(name) = U16CStr::from_ptr_str(_name).to_string() else { return 0 };
    *component = match name.as_str() {
        "TestAddIn" => Box::into_raw(Box::new(TestAddIn::new())) as *const u8,
        //"TestAddInB" => Box::into_raw(Box::new(TestAddInB::new())) as *const u8,
        _ => return 0
    };
    component as usize
}

#[no_mangle]
unsafe extern "C" fn DestroyObject(_component: *mut *const u8) -> usize {
    0
}

#[no_mangle]
unsafe extern "C" fn GetClassNames() -> *const u16 {
    CLASS_NAMES.get_or_init(|| U16CString::from_str("TestAddIn").unwrap().as_slice_with_nul().to_vec()).as_ptr()
    //CLASS_NAMES.get_or_init(|| U16CString::from_str("TestAddIn|TestAddInB").unwrap().as_slice_with_nul().to_vec()).as_ptr()
}

#[no_mangle]
unsafe extern "C" fn SetPlatformCapabilities(capabilities: AppCapabilities) -> AppCapabilities {
    capabilities
}
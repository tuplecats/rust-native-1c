use native_1c::component::{IComponentInit, IComponentBase};
use native_1c::native_macro::native_object;
use native_1c::types::Variant;

#[native_object]
#[repr(C)]
pub struct TestAddIn {
    pub string_prop: String,
    #[derivative(Default(value = "1"))]
    pub num_prop: i32,
}

impl IComponentBase for TestAddIn {
    fn init(&mut self) -> bool {
        true
    }

    fn get_info(&self) -> i32 {
        1000
    }

    fn done(&mut self) {}

    fn get_n_props(&self) -> i32 {
        2
    }

    fn find_prop(&self, prop_name: &str) -> i32 {
        match prop_name {
            "ТестовоеСвойство" | "TestProp" => 0,
            "СтроковоеСвойство" | "StringProp" => 1,
            _ => unreachable!(),
        }
    }

    fn get_prop_name(&self, prop_num: i32, prop_alias: i32) -> &str {
        match prop_num {
            0 => {
                if prop_alias == 0 {
                    "TestProp"
                } else {
                    "ТестовоеСвойство"
                }
            }
            1 => {
                if prop_alias == 0 {
                    "StringProp"
                } else {
                    "СтроковоеСвойство"
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_prop_val(&self, prop_num: i32, var_prop_val: &mut Variant) -> bool {
        match prop_num {
            0 => {
                *var_prop_val = Variant::from(self.num_prop);
            }
            1 => {
                *var_prop_val = Variant::utf8_string(self, self.string_prop.as_str());
            }
            _ => return false,
        }
        true
    }

    fn set_prop_val(&mut self, prop_num: i32, var_prop_val: &Variant) -> bool {
        match prop_num {
            0 => match var_prop_val.as_i32() {
                Some(value) => self.num_prop = value,
                _ => return false
            }
            1 => match var_prop_val.as_string() {
                Some(value) => self.string_prop = value,
                _ => return false
            }
            _ => return false,
        }
        true
    }
    fn is_prop_readable(&self, _prop_num: i32) -> bool {
        true
    }

    fn is_prop_writeable(&self, _prop_num: i32) -> bool {
        true
    }

    fn get_n_methods(&self) -> i32 {
        2
    }

    fn find_method(&self, method_name: &str) -> i32 {
        match method_name {
            "ИнформацияОСистеме" | "SystemInfo" => 0,
            "ПоказатьОкноСИнформацией" | "ShowInfoWindow" => 1,
            _ => unreachable!(),
        }
    }
    fn get_method_name(&self, method_num: i32, method_alias: i32) -> &str {
        match method_num {
            0 => {
                if method_alias == 0 {
                    "SystemInfo"
                } else {
                    "ИнформацияОСистеме"
                }
            }
            1 => {
                if method_alias == 0 {
                    "ShowInfoWindow"
                } else {
                    "ПоказатьОкноСИнформацией"
                }
            }
            _ => unreachable!(),
        }
    }
    fn get_n_params(&self, method_num: i32) -> i32 {
        match method_num {
            0 => 3,
            1 => 1,
            _ => 0,
        }
    }
    fn get_param_def_value(
        &self,
        method_num: i32,
        param_num: i32,
        var_param_def_value: &mut Variant,
    ) -> bool {
        match method_num {
            1 if param_num == 0 => {
                *var_param_def_value = Variant::utf8_string(self, "Alert");
            }
            _ => return false,
        }
        true
    }
    fn has_ret_val(&self, _method_num: i32) -> bool {
        true
    }

    fn call_as_proc(&mut self, _method_num: i32, _params: Option<&mut [Variant]>) -> bool {
        true
    }

    fn call_as_func(
        &mut self,
        method_num: i32,
        ret_vals: &mut Variant,
        params: Option<&mut [Variant]>,
    ) -> bool {
        match method_num {
            0 => {
                let app_info = self.connector().platform_info();
                let params_mut = params.unwrap();
                match app_info {
                    Ok(app_info) => {
                        params_mut[0] = Variant::utf8_string(self, app_info.user_agent());
                        params_mut[1] = Variant::utf8_string(self, app_info.version());
                        params_mut[2] = Variant::from(app_info.application() as i8);
                        *ret_vals = Variant::from(true);
                    },
                    _ => *ret_vals = Variant::from(false)
                }
            },
            1 => {
                match self.connector().message_box_alert("Hello world", self.mem_manager()) {
                    Ok(()) => *ret_vals = Variant::from(true),
                    Err(()) => *ret_vals = Variant::from(false),
                }
            }
            _ => return false,
        }
        true
    }

    fn set_locale(&mut self, _loc: &str) {}
}
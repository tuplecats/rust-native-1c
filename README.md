# native-1c

## Создание компоненты с использованием native-1c

Более наглядно представлено в [примере](examples/simple_example).

```toml
# Cargo.toml

[package]
name = "..."
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
native-1c = { git = "https://github.com/tuplecats/rust-native-1c.git" }
```

Для любой компоненты необходимо реализовать extern функции,
которые использует 1С для подключения компоненты:

```rust
unsafe extern "C" fn GetClassObject(_name: *const u16, component: *mut *const u8) -> usize;
unsafe extern "C" fn DestroyObject(_component: *mut *const u8) -> usize;
unsafe extern "C" fn GetClassNames() -> *const u16;
unsafe extern "C" fn SetPlatformCapabilities(capabilities: AppCapabilities) -> AppCapabilities;
```

Любая структура, которая будет использоваться 1С, должна использовать процедурный макрос `native_object`, 
использовать `#[repr(C)]` и реализовывать трейт `IComponentBase`:
```rust
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
```
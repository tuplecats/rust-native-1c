use std::ffi::{c_void, c_long, c_ulong};
use std::ptr::NonNull;


#[repr(C)]
pub struct IMemoryManagerVTable {
    #[cfg(target_os = "linux")]
    offset_linux: u64,
    _drop: unsafe extern "stdcall" fn(&mut IMemoryManager),
    _alloc_memory: unsafe extern "stdcall" fn(&mut IMemoryManager, *mut *const c_void, c_ulong) -> bool,
    _free_memory: unsafe extern "stdcall" fn(&mut IMemoryManager, *mut *const c_void),
}

pub struct IMemoryManager {
    vtable: NonNull<IMemoryManagerVTable>,
}

impl IMemoryManager {
    pub fn alloc_memory(&mut self, ptr: *mut *const c_void, size: c_ulong) -> bool {
        unsafe {
            (self.vtable.as_mut()._alloc_memory)(self, ptr, size)
        }
    }

    pub fn free_memory(&mut self, ptr: *mut *const c_void) {
        unsafe {
            (self.vtable.as_mut()._free_memory)(self, ptr)
        }
    }

    pub fn copy_u8_array<T>(&mut self, value: &[T]) -> *const u8 {
        let mut ptr = std::ptr::null_mut();
        let data_ptr = value.as_ptr() as *const u8;
        let size = value.len() * std::mem::size_of::<T>();
        self.alloc_memory((&mut ptr as *mut *mut u8) as *mut *const c_void, size as c_ulong);
        unsafe {
            if !value.is_empty() {
                std::ptr::copy(data_ptr, ptr, size);
            }
        }
        ptr
    }

    pub fn copy_utf8_str(&mut self, value: &str) -> *const u8 {
        let mut ptr = std::ptr::null_mut();
        let data_ptr = value.as_ptr();
        self.alloc_memory((&mut ptr as *mut *mut u8) as *mut *const c_void, (value.len() as c_ulong) + 1);
        unsafe {
            if !value.is_empty() {
                std::ptr::copy(data_ptr, ptr, value.len() + 1);
            }
            std::ptr::write(((ptr as c_long) + (value.len() as c_long)) as *mut u8, 0x00 as u8);
        }
        ptr
    }

    pub fn copy_utf16_str(&mut self, value: &str) -> *const u16 {
        let data = value.encode_utf16().collect::<Vec<u16>>();
        let data_ptr = data.as_ptr();
        let mut ptr = std::ptr::null_mut();
        self.alloc_memory((&mut ptr as *mut *mut u16) as *mut *const c_void, (data.len() as c_ulong) * 2 + 2);
        unsafe {
            if !value.is_empty() {
                std::ptr::copy(data_ptr, ptr, data.len() + 1);
            }
            std::ptr::write(((ptr as c_long) + (data.len() as c_long) * 2) as *mut u16, 0x0000 as u16);
        }
        ptr
    }
}
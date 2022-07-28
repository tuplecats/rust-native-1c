use std::ffi::c_void;
use std::ptr::NonNull;


#[repr(C)]
pub struct IMemoryManagerVTable {
    _drop: unsafe extern "C" fn(&mut IMemoryManager),
    _alloc_memory: unsafe extern "C" fn(&mut IMemoryManager, *mut *const c_void, usize) -> bool,
    _free_memory: unsafe extern "C" fn(&mut IMemoryManager, *mut *const c_void),
}

pub struct IMemoryManager {
    vtable: NonNull<IMemoryManagerVTable>,
}

impl IMemoryManager {
    pub fn alloc_memory(&mut self, ptr: *mut *const c_void, size: usize) -> bool {
        unsafe {
            (self.vtable.as_mut()._alloc_memory)(self, ptr, size)
        }
    }

    pub fn free_memory(&mut self, ptr: *mut *const c_void) {
        unsafe {
            (self.vtable.as_mut()._free_memory)(self, ptr)
        }
    }

    pub fn alloc_utf16_str(&mut self, value: &str) -> *const u16 {
        let data = value.encode_utf16().collect::<Vec<u16>>();
        let data_ptr = data.as_ptr();
        let mut ptr = std::ptr::null_mut();
        self.alloc_memory((&mut ptr as *mut *mut u16) as *mut *const c_void, data.len() * 2 + 2);
        unsafe {
            if !value.is_empty() {
                std::ptr::copy(data_ptr, ptr, data.len() + 1);
            }
            std::ptr::write(((ptr as usize) + data.len() * 2) as *mut u16, 0x0000 as u16);
        }
        ptr
    }
}
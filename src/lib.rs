extern crate derivative;

pub use once_cell::sync::OnceCell;
pub use derivative::Derivative;
pub use widestring;

pub use native_macro;

pub mod component;
pub mod types;
pub mod memory;
pub mod connector;

use memory::IMemoryManager;
use connector::IConnector;

pub(crate) struct SafePointer<T>(*mut T);
unsafe impl<T> Sync for SafePointer<T> {}
unsafe impl<T> Send for SafePointer<T> {}

static MEMORY_MANAGER: OnceCell<SafePointer<IMemoryManager>> = OnceCell::new();
static CONNECTOR: OnceCell<SafePointer<IConnector>> = OnceCell::new();

pub fn memory_manager() -> &'static mut IMemoryManager {
    unsafe { MEMORY_MANAGER.get().unwrap().0.as_mut().unwrap() }
}

pub(crate) fn set_memory_manager(manager: *mut IMemoryManager) -> bool {
    MEMORY_MANAGER.get_or_init(|| SafePointer(manager));
    true
}

pub fn connector() -> &'static mut IConnector {
    unsafe { CONNECTOR.get().unwrap().0.as_mut().unwrap() }
}

pub(crate) fn set_connector(connector: *mut IConnector) -> bool {
    CONNECTOR.get_or_init(|| SafePointer(connector));
    true
}


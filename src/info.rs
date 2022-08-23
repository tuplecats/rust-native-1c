use std::ptr::NonNull;
use widestring::U16CStr;

#[repr(C)]
pub(crate) struct PlatformInfoVTable {
    pub(crate) get_platform_info: unsafe extern "stdcall" fn(&IPlatformInfo) -> *const AppInfoInner,
}

#[repr(C)]
pub(crate) struct IPlatformInfo {
    pub(crate) vtable: NonNull<PlatformInfoVTable>
}

#[repr(i8)]
#[derive(Copy, Clone)]
pub enum AppType {
    AppUnknown = -1,
    AppThinClient = 0,
    AppThickClient = 1,
    AppWebClient = 2,
    AppServer = 3,
    AppExtConn = 4,
    AppMobileClient = 5,
    AppMobileServer = 6,
}

#[repr(C)]
pub(crate) struct AppInfoInner {
    version: *const u16,
    user_agent: *const u16,
    application: AppType,
}

pub struct AppInfo {
    version: String,
    user_agent: String,
    application: AppType,
}

impl AppInfo {
    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    pub fn user_agent(&self) -> &str {
        self.user_agent.as_str()
    }

    pub fn application(&self) -> AppType {
        self.application
    }
}

impl From<&AppInfoInner> for AppInfo {
    fn from(value: &AppInfoInner) -> Self {
        AppInfo {
            version: {
                match value.version.is_null() {
                    true => String::new(),
                    false => unsafe { U16CStr::from_ptr_str(value.version).to_string_lossy() }
                }
            },
            user_agent: {
                match value.user_agent.is_null() {
                    true => String::new(),
                    false => unsafe { U16CStr::from_ptr_str(value.user_agent).to_string_lossy() }
                }
            },
            application: value.application
        }
    }
}
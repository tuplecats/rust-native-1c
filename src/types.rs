use core::ffi::c_char;
use crate::memory_manager;

#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum VariableType {
    VTYPE_EMPTY = 0,
    VTYPE_NULL = 1,
    VTYPE_I2 = 2,                   //int16_t
    VTYPE_I4 = 3,                   //int32_t
    VTYPE_R4 = 4,                   //float
    VTYPE_R8 = 5,                   //double
    VTYPE_DATE = 6,                 //DATE (double)
    VTYPE_TM = 7,                   //struct tm
    VTYPE_PSTR = 8,                 //struct str    string
    VTYPE_INTERFACE = 9,            //struct iface
    VTYPE_ERROR = 10,                //int32_t errCode
    VTYPE_BOOL = 11,                 //bool
    VTYPE_VARIANT = 12,              //struct _tVariant *
    VTYPE_I1 = 13,                   //int8_t
    VTYPE_UI1 = 14,                  //uint8_t
    VTYPE_UI2 = 15,                  //uint16_t
    VTYPE_UI4 = 16,                  //uint32_t
    VTYPE_I8 = 17,                   //int64_t
    VTYPE_UI8 = 18,                  //uint64_t
    VTYPE_INT = 19,                  //int   Depends on architecture
    VTYPE_UINT = 20,                 //unsigned int  Depends on architecture
    VTYPE_HRESULT = 21,              //long hRes
    VTYPE_PWSTR = 22,                //struct wstr
    VTYPE_BLOB = 23,                 //means in struct str binary data contain
    VTYPE_CLSID = 24,                //UUID
    VTYPE_VECTOR   = 0x1000,
    VTYPE_ARRAY    = 0x2000,
    VTYPE_BYREF    = 0x4000,    //Only with struct _tVariant *
    VTYPE_RESERVED = 0x8000,
    VTYPE_ILLEGAL  = 0xffff,
}

#[repr(C)]
pub union VariantUnion {
    i8val: i8,
    short_val: i16,
    l_val: i32,
    int_val: i32,
    uint_val: u32,
    ll_val: i64,
    ui8val: u8,
    ushort_val: u16,
    ul_val: u32,
    ull_val: u64,
    err_code: i32,
    h_res: i64,
    flt_val: f64,
    dbl_val: f64,
    b_val: bool,
    ch_val: i8,
    wch_val: u16,
    pvar_val: *const Variant,
    pwstr_val: (*const u16, u32),
    pstr_val: (*const c_char, u32),
    nothing: [u8;34]
}

#[repr(C)]
pub struct Variant {
    value: VariantUnion,
    cb_elements: u32,
    vt: VariableType
}

impl Variant {
    pub fn as_string(&self) -> Option<String> {
        match self.vt {
            VariableType::VTYPE_PWSTR => {
                Some(String::from_utf16_lossy(
                    unsafe {
                        std::slice::from_raw_parts(self.value.pwstr_val.0, self.value.pwstr_val.1 as usize)
                    }
                ).to_string())
            },
            VariableType::VTYPE_PSTR => {
                Some(String::from_utf8_lossy(
                    unsafe {
                        std::slice::from_raw_parts(self.value.pstr_val.0 as *const u8, self.value.pstr_val.1 as usize)
                    }
                ).to_string())
            },
            _ => None
        }
    }

    pub fn as_blob(&self) -> Option<Vec<u8>> {
        match self.vt {
            VariableType::VTYPE_BLOB => {
                Some(Vec::from(
                    unsafe {
                        std::slice::from_raw_parts(self.value.pstr_val.0 as *const u8, self.value.pstr_val.0 as usize)
                    }
                ))
            },
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self.vt {
            VariableType::VTYPE_BOOL => Some(unsafe { self.value.b_val }),
            _ => None,
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.i8val }),
            _ => None,
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.short_val }),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.l_val }),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.ll_val }),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.ui8val }),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.ushort_val }),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.ul_val }),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.ull_val }),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self.vt {
            VariableType::VTYPE_I1
            | VariableType::VTYPE_I2
            | VariableType::VTYPE_I4
            | VariableType::VTYPE_I8
            | VariableType::VTYPE_INT
            | VariableType::VTYPE_UI1
            | VariableType::VTYPE_UI2
            | VariableType::VTYPE_UI4
            | VariableType::VTYPE_UI8
            | VariableType::VTYPE_UINT
            | VariableType::VTYPE_R4
            | VariableType::VTYPE_R8 => Some(unsafe { self.value.dbl_val }),
            _ => None,
        }
    }

    pub fn utf8_string(value: &str) -> Variant {
        Variant {
            value: VariantUnion { pstr_val: (memory_manager().copy_utf8_str(value) as *const i8, value.len() as u32 )},
            cb_elements: 0,
            vt: VariableType::VTYPE_PSTR
        }
    }

    pub fn utf16_string(value: &str) -> Variant {
        Variant {
            value: VariantUnion { pwstr_val: (memory_manager().copy_utf16_str(value), value.len() as u32 )},
            cb_elements: 0,
            vt: VariableType::VTYPE_PWSTR
        }
    }

}

impl<T> From<&[T]> for Variant {
    fn from(value: &[T]) -> Self {
        let size = value.len() * std::mem::size_of::<T>();
        Variant {
            value: VariantUnion { pstr_val: (memory_manager().copy_u8_array(value) as *const i8, size as u32 )},
            cb_elements: 0,
            vt: VariableType::VTYPE_BLOB
        }
    }
}

impl From<bool> for Variant {
    fn from(value: bool) -> Self {
        Variant {
            value: VariantUnion { b_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_UI1
        }
    }
}

impl From<u8> for Variant {
    fn from(value: u8) -> Self {
        Variant {
            value: VariantUnion { ui8val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_UI1
        }
    }
}

impl From<u16> for Variant {
    fn from(value: u16) -> Self {
        Variant {
            value: VariantUnion { ushort_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_UI2
        }
    }
}

impl From<u32> for Variant {
    fn from(value: u32) -> Self {
        Variant {
            value: VariantUnion { ul_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_UI4
        }
    }
}

impl From<u64> for Variant {
    fn from(value: u64) -> Self {
        Variant {
            value: VariantUnion { ull_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_UI8
        }
    }
}

impl From<i8> for Variant {
    fn from(value: i8) -> Self {
        Variant {
            value: VariantUnion { i8val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_I1
        }
    }
}

impl From<i16> for Variant {
    fn from(value: i16) -> Self {
        Variant {
            value: VariantUnion { short_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_I2
        }
    }
}

impl From<i32> for Variant {
    fn from(value: i32) -> Self {
        Variant {
            value: VariantUnion { l_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_I4
        }
    }
}

impl From<i64> for Variant {
    fn from(value: i64) -> Self {
        Variant {
            value: VariantUnion { ll_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_I8
        }
    }
}

impl From<f64> for Variant {
    fn from(value: f64) -> Self {
        Variant {
            value: VariantUnion { dbl_val: value },
            cb_elements: 0,
            vt: VariableType::VTYPE_R8
        }
    }
}
use core::ffi::c_char;

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
    pub i8val: i8,
    pub short_val: i16,
    pub l_val: i32,
    pub int_val: i32,
    pub uint_val: u32,
    pub ll_val: i64,
    pub ui8val: u8,
    pub ushort_val: u16,
    pub ul_val: u32,
    pub ull_val: u64,
    pub err_code: i32,
    pub h_res: i64,
    pub flt_val: f64,
    pub dbl_val: f64,
    pub b_val: bool,
    pub ch_val: i8,
    pub wch_val: u16,
    pub pvar_val: *const Variant,
    pub pwstr_val: (*const u16, u32),
    pub pstr_val: (*const c_char, u32),
    nothing: [u8;34]
}

#[repr(C)]
pub struct Variant {
    pub value: VariantUnion,
    pub cb_elements: u32,
    pub vt: VariableType
}
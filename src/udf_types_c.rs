/* automatically generated by rust-bindgen 0.60.1 */

#![allow(non_camel_case_types)]
#![allow(unused)]

pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;

///  not valid for UDFs
pub const Item_result_INVALID_RESULT: Item_result = -1;

///  char *
pub const Item_result_STRING_RESULT: Item_result = 0;

///  double
pub const Item_result_REAL_RESULT: Item_result = 1;

///  long long
pub const Item_result_INT_RESULT: Item_result = 2;

///  not valid for UDFs
pub const Item_result_ROW_RESULT: Item_result = 3;

pub const Item_result_DECIMAL_RESULT: Item_result = 4;

/// Type of the user defined function return slot and arguments
pub type Item_result = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UDF_ARGS {
    /// < Number of arguments
    pub arg_count: ::std::os::raw::c_uint,

    /// < Pointer to item_results
    pub arg_type: *mut Item_result,

    /// < Pointer to argument
    pub args: *mut *mut ::std::os::raw::c_char,

    /// < Length of string arguments
    /// Do not change data
    pub lengths: *mut ::std::os::raw::c_ulong,

    /// < Set to 1 for all maybe_null args
    pub maybe_null: *mut ::std::os::raw::c_char,

    /// < Pointer to attribute name
    pub attributes: *mut *mut ::std::os::raw::c_char,

    /// < Length of attribute arguments
    pub attribute_lengths: *mut ::std::os::raw::c_ulong,
    pub extension: *mut ::std::os::raw::c_void,
}

#[test]
fn bindgen_test_layout_UDF_ARGS() {
    assert_eq!(
        ::std::mem::size_of::<UDF_ARGS>(),
        64usize,
        concat!("Size of: ", stringify!(UDF_ARGS))
    );
    assert_eq!(
        ::std::mem::align_of::<UDF_ARGS>(),
        8usize,
        concat!("Alignment of ", stringify!(UDF_ARGS))
    );
    fn test_field_arg_count() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).arg_count) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(arg_count)
            )
        );
    }
    test_field_arg_count();
    fn test_field_arg_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).arg_type) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(arg_type)
            )
        );
    }
    test_field_arg_type();
    fn test_field_args() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).args) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(args)
            )
        );
    }
    test_field_args();
    fn test_field_lengths() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lengths) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(lengths)
            )
        );
    }
    test_field_lengths();
    fn test_field_maybe_null() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).maybe_null) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(maybe_null)
            )
        );
    }
    test_field_maybe_null();
    fn test_field_attributes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).attributes) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(attributes)
            )
        );
    }
    test_field_attributes();
    fn test_field_attribute_lengths() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).attribute_lengths) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(attribute_lengths)
            )
        );
    }
    test_field_attribute_lengths();
    fn test_field_extension() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_ARGS>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).extension) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_ARGS),
                "::",
                stringify!(extension)
            )
        );
    }
    test_field_extension();
}
/// Information about the result of a user defined function
///
/// @todo add a notion for determinism of the UDF.
///
/// @sa Item_udf_func::update_used_tables()
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UDF_INIT {
    ///  1 if function can return NULL
    pub maybe_null: bool,
    ///  for real functions
    pub decimals: ::std::os::raw::c_uint,
    ///  For string functions
    pub max_length: ::std::os::raw::c_ulong,
    ///  free pointer for function data
    pub ptr: *mut ::std::os::raw::c_char,
    ///  1 if function always returns the same value
    pub const_item: bool,
    pub extension: *mut ::std::os::raw::c_void,
}

#[test]
fn bindgen_test_layout_UDF_INIT() {
    assert_eq!(
        ::std::mem::size_of::<UDF_INIT>(),
        40usize,
        concat!("Size of: ", stringify!(UDF_INIT))
    );
    assert_eq!(
        ::std::mem::align_of::<UDF_INIT>(),
        8usize,
        concat!("Alignment of ", stringify!(UDF_INIT))
    );
    fn test_field_maybe_null() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_INIT>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).maybe_null) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_INIT),
                "::",
                stringify!(maybe_null)
            )
        );
    }
    test_field_maybe_null();
    fn test_field_decimals() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_INIT>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).decimals) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_INIT),
                "::",
                stringify!(decimals)
            )
        );
    }
    test_field_decimals();
    fn test_field_max_length() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_INIT>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).max_length) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_INIT),
                "::",
                stringify!(max_length)
            )
        );
    }
    test_field_max_length();
    fn test_field_ptr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_INIT>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ptr) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_INIT),
                "::",
                stringify!(ptr)
            )
        );
    }
    test_field_ptr();
    fn test_field_const_item() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_INIT>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).const_item) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_INIT),
                "::",
                stringify!(const_item)
            )
        );
    }
    test_field_const_item();
    fn test_field_extension() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UDF_INIT>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).extension) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(UDF_INIT),
                "::",
                stringify!(extension)
            )
        );
    }
    test_field_extension();
}

pub const Item_udftype_UDFTYPE_FUNCTION: Item_udftype = 1;
pub const Item_udftype_UDFTYPE_AGGREGATE: Item_udftype = 2;

pub type Item_udftype = ::std::os::raw::c_uint;
pub type Udf_func_clear = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut UDF_INIT,
        arg2: *mut ::std::os::raw::c_uchar,
        arg3: *mut ::std::os::raw::c_uchar,
    ),
>;
pub type Udf_func_add = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut UDF_INIT,
        arg2: *mut UDF_ARGS,
        arg3: *mut ::std::os::raw::c_uchar,
        arg4: *mut ::std::os::raw::c_uchar,
    ),
>;
pub type Udf_func_deinit = ::std::option::Option<unsafe extern "C" fn(arg1: *mut UDF_INIT)>;
pub type Udf_func_init = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut UDF_INIT,
        arg2: *mut UDF_ARGS,
        arg3: *mut ::std::os::raw::c_char,
    ) -> bool,
>;
pub type Udf_func_any = ::std::option::Option<unsafe extern "C" fn()>;
pub type Udf_func_double = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut UDF_INIT,
        arg2: *mut UDF_ARGS,
        arg3: *mut ::std::os::raw::c_uchar,
        arg4: *mut ::std::os::raw::c_uchar,
    ) -> f64,
>;
pub type Udf_func_longlong = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut UDF_INIT,
        arg2: *mut UDF_ARGS,
        arg3: *mut ::std::os::raw::c_uchar,
        arg4: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_longlong,
>;
pub type Udf_func_string = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut UDF_INIT,
        arg2: *mut UDF_ARGS,
        arg3: *mut ::std::os::raw::c_char,
        arg4: *mut ::std::os::raw::c_ulong,
        arg5: *mut ::std::os::raw::c_uchar,
        arg6: *mut ::std::os::raw::c_uchar,
    ) -> *mut ::std::os::raw::c_char,
>;

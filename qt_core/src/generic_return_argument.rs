/// C++ type: <span style='color: green;'>```QGenericReturnArgument```</span>
#[repr(C)]
pub struct GenericReturnArgument([u8; ::type_sizes::QT_CORE_GENERIC_RETURN_ARGUMENT_GENERIC_RETURN_ARGUMENT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for GenericReturnArgument {
  unsafe fn new_uninitialized() -> GenericReturnArgument {
    GenericReturnArgument(::std::mem::uninitialized())
  }
}

impl GenericReturnArgument {
  /// C++ method: <span style='color: green;'>```[constructor] void QGenericReturnArgument::QGenericReturnArgument()```</span>
  ///
  ///
  pub fn new() -> ::generic_return_argument::GenericReturnArgument {
    {
      let mut object: ::generic_return_argument::GenericReturnArgument =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QGenericReturnArgument_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGenericReturnArgument::QGenericReturnArgument```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::libc::c_char) -> ::generic_return_argument::GenericReturnArgument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGenericReturnArgument::QGenericReturnArgument(const char* aName = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_char, *mut ::libc::c_void)) -> ::generic_return_argument::GenericReturnArgument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGenericReturnArgument::QGenericReturnArgument(const char* aName = ?, void* aData = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::generic_return_argument::GenericReturnArgument
    where Args: overloading::GenericReturnArgumentNewUnsafeArgs
  {
    args.exec()
  }
}

impl Drop for ::generic_return_argument::GenericReturnArgument {
  /// C++ method: <span style='color: green;'>```[destructor] void QGenericReturnArgument::~QGenericReturnArgument()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QGenericReturnArgument_destructor(self as *mut ::generic_return_argument::GenericReturnArgument)
    }
  }
}

impl ::cpp_utils::StaticCast<::generic_argument::GenericArgument> for ::generic_return_argument::GenericReturnArgument {
fn static_cast_mut(&mut self) -> &mut ::generic_argument::GenericArgument {
let ffi_result = unsafe { ::ffi::qt_core_c_QGenericReturnArgument_G_static_cast_QGenericArgument_ptr(self as *mut ::generic_return_argument::GenericReturnArgument) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::generic_argument::GenericArgument {
let ffi_result = unsafe { ::ffi::qt_core_c_QGenericReturnArgument_G_static_cast_QGenericArgument_ptr(self as *const ::generic_return_argument::GenericReturnArgument as *mut ::generic_return_argument::GenericReturnArgument) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::generic_return_argument::GenericReturnArgument> for ::generic_argument::GenericArgument {
unsafe fn static_cast_mut(&mut self) -> &mut ::generic_return_argument::GenericReturnArgument {
let ffi_result = ::ffi::qt_core_c_QGenericReturnArgument_G_static_cast_QGenericReturnArgument_ptr(self as *mut ::generic_argument::GenericArgument);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::generic_return_argument::GenericReturnArgument {
let ffi_result = ::ffi::qt_core_c_QGenericReturnArgument_G_static_cast_QGenericReturnArgument_ptr(self as *const ::generic_argument::GenericArgument as *mut ::generic_argument::GenericArgument);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::generic_return_argument::GenericReturnArgument {
  type Target = ::generic_argument::GenericArgument;
  fn deref(&self) -> &::generic_argument::GenericArgument {
    let ffi_result = unsafe { ::ffi::qt_core_c_QGenericReturnArgument_G_static_cast_QGenericArgument_ptr(self as *const ::generic_return_argument::GenericReturnArgument as *mut ::generic_return_argument::GenericReturnArgument) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::generic_return_argument::GenericReturnArgument {
  fn deref_mut(&mut self) -> &mut ::generic_argument::GenericArgument {
    let ffi_result = unsafe { ::ffi::qt_core_c_QGenericReturnArgument_G_static_cast_QGenericArgument_ptr(self as *mut ::generic_return_argument::GenericReturnArgument) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GenericReturnArgument::new_unsafe](../struct.GenericReturnArgument.html#method.new_unsafe) method.
  pub trait GenericReturnArgumentNewUnsafeArgs {
    unsafe fn exec(self) -> ::generic_return_argument::GenericReturnArgument;
  }
  impl GenericReturnArgumentNewUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::generic_return_argument::GenericReturnArgument {
      let a_name = self;
      {
        let mut object: ::generic_return_argument::GenericReturnArgument =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QGenericReturnArgument_constructor_aName(a_name, &mut object);
        object
      }
    }
  }
  impl GenericReturnArgumentNewUnsafeArgs for (*const ::libc::c_char, *mut ::libc::c_void) {
    unsafe fn exec(self) -> ::generic_return_argument::GenericReturnArgument {
      let a_name = self.0;
      let a_data = self.1;
      {
        let mut object: ::generic_return_argument::GenericReturnArgument =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QGenericReturnArgument_constructor_aName_aData(a_name, a_data, &mut object);
        object
      }
    }
  }
}

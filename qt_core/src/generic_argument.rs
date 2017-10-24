/// C++ type: <span style='color: green;'>```QGenericArgument```</span>
#[repr(C)]
pub struct GenericArgument([u8; ::type_sizes::QT_CORE_GENERIC_ARGUMENT_GENERIC_ARGUMENT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for GenericArgument {
  unsafe fn new_uninitialized() -> GenericArgument {
    GenericArgument(::std::mem::uninitialized())
  }
}

impl GenericArgument {
  /// C++ method: <span style='color: green;'>```void* QGenericArgument::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QGenericArgument_data(self as *const ::generic_argument::GenericArgument) }
  }

  /// C++ method: <span style='color: green;'>```const char* QGenericArgument::name() const```</span>
  ///
  ///
  pub fn name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QGenericArgument_name(self as *const ::generic_argument::GenericArgument) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGenericArgument::QGenericArgument()```</span>
  ///
  ///
  pub fn new() -> ::generic_argument::GenericArgument {
    {
      let mut object: ::generic_argument::GenericArgument =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QGenericArgument_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGenericArgument::QGenericArgument```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::libc::c_char) -> ::generic_argument::GenericArgument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGenericArgument::QGenericArgument(const char* aName = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_char, *const ::libc::c_void)) -> ::generic_argument::GenericArgument```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGenericArgument::QGenericArgument(const char* aName = ?, const void* aData = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::generic_argument::GenericArgument
    where Args: overloading::GenericArgumentNewUnsafeArgs
  {
    args.exec()
  }
}

impl Drop for ::generic_argument::GenericArgument {
  /// C++ method: <span style='color: green;'>```[destructor] void QGenericArgument::~QGenericArgument()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QGenericArgument_destructor(self as *mut ::generic_argument::GenericArgument) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GenericArgument::new_unsafe](../struct.GenericArgument.html#method.new_unsafe) method.
  pub trait GenericArgumentNewUnsafeArgs {
    unsafe fn exec(self) -> ::generic_argument::GenericArgument;
  }
  impl GenericArgumentNewUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::generic_argument::GenericArgument {
      let a_name = self;
      {
        let mut object: ::generic_argument::GenericArgument =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QGenericArgument_constructor_aName(a_name, &mut object);
        object
      }
    }
  }
  impl GenericArgumentNewUnsafeArgs for (*const ::libc::c_char, *const ::libc::c_void) {
    unsafe fn exec(self) -> ::generic_argument::GenericArgument {
      let a_name = self.0;
      let a_data = self.1;
      {
        let mut object: ::generic_argument::GenericArgument =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QGenericArgument_constructor_aName_aData(a_name, a_data, &mut object);
        object
      }
    }
  }
}

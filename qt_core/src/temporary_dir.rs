/// C++ type: <span style='color: green;'>```QTemporaryDir```</span>
#[repr(C)]
pub struct TemporaryDir([u8; ::type_sizes::QT_CORE_TEMPORARY_DIR_TEMPORARY_DIR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TemporaryDir {
  unsafe fn new_uninitialized() -> TemporaryDir {
    TemporaryDir(::std::mem::uninitialized())
  }
}

impl TemporaryDir {
  /// C++ method: <span style='color: green;'>```bool QTemporaryDir::autoRemove() const```</span>
  ///
  ///
  pub fn auto_remove(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTemporaryDir_autoRemove(self as *const ::temporary_dir::TemporaryDir) }
  }

  /// C++ method: <span style='color: green;'>```QString QTemporaryDir::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTemporaryDir_errorString_to_output(self as *const ::temporary_dir::TemporaryDir, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTemporaryDir::filePath(const QString& fileName) const```</span>
  ///
  ///
  pub fn file_path(&self, file_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTemporaryDir_filePath_to_output(self as *const ::temporary_dir::TemporaryDir,
                                                          file_name as *const ::string::String,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTemporaryDir::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTemporaryDir_isValid(self as *const ::temporary_dir::TemporaryDir) }
  }

  /// C++ method: <span style='color: green;'>```QTemporaryDir::QTemporaryDir```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::temporary_dir::TemporaryDir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTemporaryDir::QTemporaryDir()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::temporary_dir::TemporaryDir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTemporaryDir::QTemporaryDir(const QString& templateName)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::temporary_dir::TemporaryDir
    where Args: overloading::TemporaryDirNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString QTemporaryDir::path() const```</span>
  ///
  ///
  pub fn path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTemporaryDir_path_to_output(self as *const ::temporary_dir::TemporaryDir, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTemporaryDir::remove()```</span>
  ///
  ///
  pub fn remove(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QTemporaryDir_remove(self as *mut ::temporary_dir::TemporaryDir) }
  }

  /// C++ method: <span style='color: green;'>```void QTemporaryDir::setAutoRemove(bool b)```</span>
  ///
  ///
  pub fn set_auto_remove(&mut self, b: bool) {
    unsafe { ::ffi::qt_core_c_QTemporaryDir_setAutoRemove(self as *mut ::temporary_dir::TemporaryDir, b) }
  }
}

impl Drop for ::temporary_dir::TemporaryDir {
  /// C++ method: <span style='color: green;'>```[destructor] void QTemporaryDir::~QTemporaryDir()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QTemporaryDir_destructor(self as *mut ::temporary_dir::TemporaryDir) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TemporaryDir::new](../struct.TemporaryDir.html#method.new) method.
  pub trait TemporaryDirNewArgs {
    fn exec(self) -> ::temporary_dir::TemporaryDir;
  }
  impl TemporaryDirNewArgs for () {
    fn exec(self) -> ::temporary_dir::TemporaryDir {

      {
        let mut object: ::temporary_dir::TemporaryDir =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTemporaryDir_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TemporaryDirNewArgs for &'a ::string::String {
    fn exec(self) -> ::temporary_dir::TemporaryDir {
      let template_name = self;
      {
        let mut object: ::temporary_dir::TemporaryDir =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTemporaryDir_constructor_templateName(template_name as *const ::string::String,
                                                                  &mut object);
        }
        object
      }
    }
  }
}

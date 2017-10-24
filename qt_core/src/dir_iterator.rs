/// C++ type: <span style='color: green;'>```QDirIterator```</span>
#[repr(C)]
pub struct DirIterator([u8; ::type_sizes::QT_CORE_DIR_ITERATOR_DIR_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for DirIterator {
  unsafe fn new_uninitialized() -> DirIterator {
    DirIterator(::std::mem::uninitialized())
  }
}

impl DirIterator {
  /// C++ method: <span style='color: green;'>```QFileInfo QDirIterator::fileInfo() const```</span>
  ///
  ///
  pub fn file_info(&self) -> ::file_info::FileInfo {
    {
      let mut object: ::file_info::FileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDirIterator_fileInfo_to_output(self as *const ::dir_iterator::DirIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDirIterator::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDirIterator_fileName_to_output(self as *const ::dir_iterator::DirIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDirIterator::filePath() const```</span>
  ///
  ///
  pub fn file_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDirIterator_filePath_to_output(self as *const ::dir_iterator::DirIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDirIterator::hasNext() const```</span>
  ///
  ///
  pub fn has_next(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDirIterator_hasNext(self as *const ::dir_iterator::DirIterator) }
  }

  /// C++ method: <span style='color: green;'>```QDirIterator::QDirIterator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::dir::Dir) -> ::dir_iterator::DirIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDirIterator::QDirIterator(const QDir& dir)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::dir::Dir, ::flags::Flags<::dir_iterator::IteratorFlag>)) -> ::dir_iterator::DirIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDirIterator::QDirIterator(const QDir& dir, QFlags<QDirIterator::IteratorFlag> flags = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::dir_iterator::DirIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDirIterator::QDirIterator(const QString& path)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::string::String, ::flags::Flags<::dir_iterator::IteratorFlag>)) -> ::dir_iterator::DirIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDirIterator::QDirIterator(const QString& path, QFlags<QDirIterator::IteratorFlag> flags = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::dir_iterator::DirIterator
    where Args: overloading::DirIteratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString QDirIterator::next()```</span>
  ///
  ///
  pub fn next(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDirIterator_next_to_output(self as *mut ::dir_iterator::DirIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDirIterator::path() const```</span>
  ///
  ///
  pub fn path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDirIterator_path_to_output(self as *const ::dir_iterator::DirIterator, &mut object);
      }
      object
    }
  }
}

impl Drop for ::dir_iterator::DirIterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QDirIterator::~QDirIterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDirIterator_destructor(self as *mut ::dir_iterator::DirIterator) }
  }
}

/// C++ type: <span style='color: green;'>```QDirIterator::IteratorFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum IteratorFlag {
  /// C++ enum variant: <span style='color: green;'>```NoIteratorFlags = 0```</span>
  NoIteratorFlags = 0,
  /// C++ enum variant: <span style='color: green;'>```FollowSymlinks = 1```</span>
  FollowSymlinks = 1,
  /// C++ enum variant: <span style='color: green;'>```Subdirectories = 2```</span>
  Subdirectories = 2,
}

impl ::flags::FlaggableEnum for IteratorFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "IteratorFlag"
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DirIterator::new](../struct.DirIterator.html#method.new) method.
  pub trait DirIteratorNewArgs {
    fn exec(self) -> ::dir_iterator::DirIterator;
  }
  impl<'a> DirIteratorNewArgs for &'a ::dir::Dir {
    fn exec(self) -> ::dir_iterator::DirIterator {
      let dir = self;
      {
        let mut object: ::dir_iterator::DirIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDirIterator_constructor_dir(dir as *const ::dir::Dir, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DirIteratorNewArgs for (&'a ::dir::Dir, ::flags::Flags<::dir_iterator::IteratorFlag>) {
    fn exec(self) -> ::dir_iterator::DirIterator {
      let dir = self.0;
      let flags = self.1;
      {
        let mut object: ::dir_iterator::DirIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDirIterator_constructor_dir_flags(dir as *const ::dir::Dir,
                                                              flags.to_int() as ::libc::c_uint,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> DirIteratorNewArgs for &'a ::string::String {
    fn exec(self) -> ::dir_iterator::DirIterator {
      let path = self;
      {
        let mut object: ::dir_iterator::DirIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDirIterator_constructor_path(path as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DirIteratorNewArgs for (&'a ::string::String, ::flags::Flags<::dir_iterator::IteratorFlag>) {
    fn exec(self) -> ::dir_iterator::DirIterator {
      let path = self.0;
      let flags = self.1;
      {
        let mut object: ::dir_iterator::DirIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDirIterator_constructor_path_flags(path as *const ::string::String,
                                                               flags.to_int() as ::libc::c_uint,
                                                               &mut object);
        }
        object
      }
    }
  }
}

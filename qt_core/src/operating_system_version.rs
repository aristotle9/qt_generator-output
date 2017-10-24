/// C++ type: <span style='color: green;'>```QOperatingSystemVersion::OSType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OSType {
  /// C++ enum variant: <span style='color: green;'>```Unknown = 0```</span>
  Unknown = 0,
  /// C++ enum variant: <span style='color: green;'>```Windows = 1```</span>
  Windows = 1,
  /// C++ enum variant: <span style='color: green;'>```MacOS = 2```</span>
  MacOS = 2,
  /// C++ enum variant: <span style='color: green;'>```IOS = 3```</span>
  IOS = 3,
  /// C++ enum variant: <span style='color: green;'>```TvOS = 4```</span>
  TvOS = 4,
  /// C++ enum variant: <span style='color: green;'>```WatchOS = 5```</span>
  WatchOS = 5,
  /// C++ enum variant: <span style='color: green;'>```Android = 6```</span>
  Android = 6,
}

/// C++ type: <span style='color: green;'>```QOperatingSystemVersion```</span>
#[repr(C)]
pub struct OperatingSystemVersion([u8; ::type_sizes::QT_CORE_OPERATING_SYSTEM_VERSION_OPERATING_SYSTEM_VERSION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for OperatingSystemVersion {
  unsafe fn new_uninitialized() -> OperatingSystemVersion {
    OperatingSystemVersion(::std::mem::uninitialized())
  }
}

impl OperatingSystemVersion {
  /// C++ method: <span style='color: green;'>```static QOperatingSystemVersion QOperatingSystemVersion::current()```</span>
  ///
  ///
  pub fn current() -> ::operating_system_version::OperatingSystemVersion {
    {
      let mut object: ::operating_system_version::OperatingSystemVersion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QOperatingSystemVersion_current_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QOperatingSystemVersion::majorVersion() const```</span>
  ///
  ///
  pub fn major_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QOperatingSystemVersion_majorVersion(self as *const ::operating_system_version::OperatingSystemVersion) }
  }

  /// C++ method: <span style='color: green;'>```int QOperatingSystemVersion::microVersion() const```</span>
  ///
  ///
  pub fn micro_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QOperatingSystemVersion_microVersion(self as *const ::operating_system_version::OperatingSystemVersion) }
  }

  /// C++ method: <span style='color: green;'>```int QOperatingSystemVersion::minorVersion() const```</span>
  ///
  ///
  pub fn minor_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QOperatingSystemVersion_minorVersion(self as *const ::operating_system_version::OperatingSystemVersion) }
  }

  /// C++ method: <span style='color: green;'>```QString QOperatingSystemVersion::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QOperatingSystemVersion_name_to_output(self as *const ::operating_system_version::OperatingSystemVersion, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOperatingSystemVersion::QOperatingSystemVersion```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((::operating_system_version::OSType, ::libc::c_int)) -> ::operating_system_version::OperatingSystemVersion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOperatingSystemVersion::QOperatingSystemVersion(QOperatingSystemVersion::OSType osType, int vmajor)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::operating_system_version::OSType, ::libc::c_int, ::libc::c_int)) -> ::operating_system_version::OperatingSystemVersion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOperatingSystemVersion::QOperatingSystemVersion(QOperatingSystemVersion::OSType osType, int vmajor, int vminor = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::operating_system_version::OSType, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::operating_system_version::OperatingSystemVersion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOperatingSystemVersion::QOperatingSystemVersion(QOperatingSystemVersion::OSType osType, int vmajor, int vminor = ?, int vmicro = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::operating_system_version::OperatingSystemVersion) -> ::operating_system_version::OperatingSystemVersion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOperatingSystemVersion::QOperatingSystemVersion(const QOperatingSystemVersion& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::operating_system_version::OperatingSystemVersion
    where Args: overloading::OperatingSystemVersionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QOperatingSystemVersion::segmentCount() const```</span>
  ///
  ///
  pub fn segment_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QOperatingSystemVersion_segmentCount(self as *const ::operating_system_version::OperatingSystemVersion) }
  }

  /// C++ method: <span style='color: green;'>```QOperatingSystemVersion::OSType QOperatingSystemVersion::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::operating_system_version::OSType {
    unsafe {
      ::ffi::qt_core_c_QOperatingSystemVersion_type(self as *const ::operating_system_version::OperatingSystemVersion)
    }
  }
}

impl Drop for ::operating_system_version::OperatingSystemVersion {
  /// C++ method: <span style='color: green;'>```[destructor] void QOperatingSystemVersion::~QOperatingSystemVersion()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QOperatingSystemVersion_destructor(self as *mut ::operating_system_version::OperatingSystemVersion) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OperatingSystemVersion::new](../struct.OperatingSystemVersion.html#method.new) method.
  pub trait OperatingSystemVersionNewArgs {
    fn exec(self) -> ::operating_system_version::OperatingSystemVersion;
  }
  impl OperatingSystemVersionNewArgs for (::operating_system_version::OSType, ::libc::c_int) {
    fn exec(self) -> ::operating_system_version::OperatingSystemVersion {
      let os_type = self.0;
      let vmajor = self.1;
      {
        let mut object: ::operating_system_version::OperatingSystemVersion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor(os_type, vmajor, &mut object);
        }
        object
      }
    }
  }
  impl OperatingSystemVersionNewArgs for (::operating_system_version::OSType, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::operating_system_version::OperatingSystemVersion {
      let os_type = self.0;
      let vmajor = self.1;
      let vminor = self.2;
      {
        let mut object: ::operating_system_version::OperatingSystemVersion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor_vminor(os_type,
                                                                                    vmajor,
                                                                                    vminor,
                                                                                    &mut object);
        }
        object
      }
    }
  }
  impl OperatingSystemVersionNewArgs
    for (::operating_system_version::OSType, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::operating_system_version::OperatingSystemVersion {
      let os_type = self.0;
      let vmajor = self.1;
      let vminor = self.2;
      let vmicro = self.3;
      {
        let mut object: ::operating_system_version::OperatingSystemVersion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QOperatingSystemVersion_constructor_osType_vmajor_vminor_vmicro(os_type,
                                                                                           vmajor,
                                                                                           vminor,
                                                                                           vmicro,
                                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> OperatingSystemVersionNewArgs for &'a ::operating_system_version::OperatingSystemVersion {
    fn exec(self) -> ::operating_system_version::OperatingSystemVersion {
      let other = self;
      {
        let mut object: ::operating_system_version::OperatingSystemVersion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QOperatingSystemVersion_constructor_other(other as *const ::operating_system_version::OperatingSystemVersion, &mut object);
        }
        object
      }
    }
  }
}

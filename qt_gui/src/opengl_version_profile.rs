/// C++ type: <span style='color: green;'>```QOpenGLVersionProfile```</span>
#[repr(C)]
pub struct OpenGLVersionProfile([u8; ::type_sizes::QT_GUI_OPENGL_VERSION_PROFILE_OPEN_G_L_VERSION_PROFILE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for OpenGLVersionProfile {
  unsafe fn new_uninitialized() -> OpenGLVersionProfile {
    OpenGLVersionProfile(::std::mem::uninitialized())
  }
}

impl OpenGLVersionProfile {
  /// C++ method: <span style='color: green;'>```bool QOpenGLVersionProfile::hasProfiles() const```</span>
  ///
  ///
  pub fn has_profiles(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVersionProfile_hasProfiles(self as *const ::opengl_version_profile::OpenGLVersionProfile)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLVersionProfile::isLegacyVersion() const```</span>
  ///
  ///
  pub fn is_legacy_version(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLVersionProfile_isLegacyVersion(self as *const ::opengl_version_profile::OpenGLVersionProfile) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLVersionProfile::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVersionProfile_isValid(self as *const ::opengl_version_profile::OpenGLVersionProfile)
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLVersionProfile::QOpenGLVersionProfile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::opengl_version_profile::OpenGLVersionProfile```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLVersionProfile::QOpenGLVersionProfile()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::opengl_version_profile::OpenGLVersionProfile) -> ::opengl_version_profile::OpenGLVersionProfile```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLVersionProfile::QOpenGLVersionProfile(const QOpenGLVersionProfile& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::surface_format::SurfaceFormat) -> ::opengl_version_profile::OpenGLVersionProfile```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLVersionProfile::QOpenGLVersionProfile(const QSurfaceFormat& format)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::opengl_version_profile::OpenGLVersionProfile
    where Args: overloading::OpenGLVersionProfileNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLVersionProfile& QOpenGLVersionProfile::operator=(const QOpenGLVersionProfile& rhs)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             rhs: &'l1 ::opengl_version_profile::OpenGLVersionProfile)
                             -> &'l0 mut ::opengl_version_profile::OpenGLVersionProfile {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLVersionProfile_operator_assign(self as *mut ::opengl_version_profile::OpenGLVersionProfile, rhs as *const ::opengl_version_profile::OpenGLVersionProfile) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLVersionProfile::setProfile(QSurfaceFormat::OpenGLContextProfile profile)```</span>
  ///
  ///
  pub fn set_profile(&mut self, profile: &::surface_format::OpenGLContextProfile) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVersionProfile_setProfile(self as *mut ::opengl_version_profile::OpenGLVersionProfile,
                                                       profile as *const ::surface_format::OpenGLContextProfile)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLVersionProfile::setVersion(int majorVersion, int minorVersion)```</span>
  ///
  ///
  pub fn set_version(&mut self, major_version: ::libc::c_int, minor_version: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVersionProfile_setVersion(self as *mut ::opengl_version_profile::OpenGLVersionProfile,
                                                       major_version,
                                                       minor_version)
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<int, int> QOpenGLVersionProfile::version() const```</span>
  ///
  ///
  pub fn version(&self) -> ::pair::PairCIntCInt {
    {
      let mut object: ::pair::PairCIntCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLVersionProfile_version_to_output(self as *const ::opengl_version_profile::OpenGLVersionProfile, &mut object);
      }
      object
    }
  }
}

impl Drop for ::opengl_version_profile::OpenGLVersionProfile {
  /// C++ method: <span style='color: green;'>```[destructor] void QOpenGLVersionProfile::~QOpenGLVersionProfile()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVersionProfile_destructor(self as *mut ::opengl_version_profile::OpenGLVersionProfile)
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLVersionProfile::new](../struct.OpenGLVersionProfile.html#method.new) method.
  pub trait OpenGLVersionProfileNewArgs {
    fn exec(self) -> ::opengl_version_profile::OpenGLVersionProfile;
  }
  impl<'a> OpenGLVersionProfileNewArgs for &'a ::surface_format::SurfaceFormat {
    fn exec(self) -> ::opengl_version_profile::OpenGLVersionProfile {
      let format = self;
      {
        let mut object: ::opengl_version_profile::OpenGLVersionProfile =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLVersionProfile_constructor_format(format as *const ::surface_format::SurfaceFormat,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl OpenGLVersionProfileNewArgs for () {
    fn exec(self) -> ::opengl_version_profile::OpenGLVersionProfile {

      {
        let mut object: ::opengl_version_profile::OpenGLVersionProfile =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLVersionProfile_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLVersionProfileNewArgs for &'a ::opengl_version_profile::OpenGLVersionProfile {
    fn exec(self) -> ::opengl_version_profile::OpenGLVersionProfile {
      let other = self;
      {
        let mut object: ::opengl_version_profile::OpenGLVersionProfile =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLVersionProfile_constructor_other(other as *const ::opengl_version_profile::OpenGLVersionProfile, &mut object);
        }
        object
      }
    }
  }
}

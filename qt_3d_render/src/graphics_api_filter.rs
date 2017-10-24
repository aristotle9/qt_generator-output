/// C++ type: <span style='color: green;'>```Qt3DRender::QGraphicsApiFilter::Api```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Api {
  /// C++ enum variant: <span style='color: green;'>```OpenGL = 1```</span>
  OpenGL = 1,
  /// C++ enum variant: <span style='color: green;'>```OpenGLES = 2```</span>
  OpenGLES = 2,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QGraphicsApiFilter```</span>
#[repr(C)]
pub struct GraphicsApiFilter(u8);

impl GraphicsApiFilter {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QGraphicsApiFilter::Api Qt3DRender::QGraphicsApiFilter::api() const```</span>
  ///
  ///
  pub fn api(&self) -> ::graphics_api_filter::Api {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_api(self as *const ::graphics_api_filter::GraphicsApiFilter)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList Qt3DRender::QGraphicsApiFilter::extensions() const```</span>
  ///
  ///
  pub fn extensions(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_extensions_to_output(self as *const ::graphics_api_filter::GraphicsApiFilter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGraphicsApiFilter::majorVersion() const```</span>
  ///
  ///
  pub fn major_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_majorVersion(self as *const ::graphics_api_filter::GraphicsApiFilter) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QGraphicsApiFilter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_metaObject(self as *const ::graphics_api_filter::GraphicsApiFilter) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGraphicsApiFilter::minorVersion() const```</span>
  ///
  ///
  pub fn minor_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_minorVersion(self as *const ::graphics_api_filter::GraphicsApiFilter) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QGraphicsApiFilter::QGraphicsApiFilter()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_api_filter::GraphicsApiFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QGraphicsApiFilter::QGraphicsApiFilter(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_api_filter::GraphicsApiFilter> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QGraphicsApiFilter::OpenGLProfile Qt3DRender::QGraphicsApiFilter::profile() const```</span>
  ///
  ///
  pub fn profile(&self) -> ::graphics_api_filter::OpenGLProfile {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_profile(self as *const ::graphics_api_filter::GraphicsApiFilter) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QGraphicsApiFilter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_qt_metacall(self as *mut ::graphics_api_filter::GraphicsApiFilter, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QGraphicsApiFilter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_qt_metacast(self as *mut ::graphics_api_filter::GraphicsApiFilter, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGraphicsApiFilter::setApi(Qt3DRender::QGraphicsApiFilter::Api api)```</span>
  ///
  ///
  pub fn set_api(&mut self, api: ::graphics_api_filter::Api) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setApi(self as *mut ::graphics_api_filter::GraphicsApiFilter, api)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGraphicsApiFilter::setExtensions(const QStringList& extensions)```</span>
  ///
  ///
  pub fn set_extensions(&mut self, extensions: &::qt_core::string_list::StringList) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setExtensions(self as *mut ::graphics_api_filter::GraphicsApiFilter, extensions as *const ::qt_core::string_list::StringList) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGraphicsApiFilter::setMajorVersion(int majorVersion)```</span>
  ///
  ///
  pub fn set_major_version(&mut self, major_version: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setMajorVersion(self as *mut ::graphics_api_filter::GraphicsApiFilter, major_version) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGraphicsApiFilter::setMinorVersion(int minorVersion)```</span>
  ///
  ///
  pub fn set_minor_version(&mut self, minor_version: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setMinorVersion(self as *mut ::graphics_api_filter::GraphicsApiFilter, minor_version) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGraphicsApiFilter::setProfile(Qt3DRender::QGraphicsApiFilter::OpenGLProfile profile)```</span>
  ///
  ///
  pub fn set_profile(&mut self, profile: ::graphics_api_filter::OpenGLProfile) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setProfile(self as *mut ::graphics_api_filter::GraphicsApiFilter, profile) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGraphicsApiFilter::setVendor(const QString& vendor)```</span>
  ///
  ///
  pub fn set_vendor(&mut self, vendor: &::qt_core::string::String) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_setVendor(self as *mut ::graphics_api_filter::GraphicsApiFilter, vendor as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QGraphicsApiFilter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QGraphicsApiFilter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DRender::QGraphicsApiFilter::vendor() const```</span>
  ///
  ///
  pub fn vendor(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_vendor_to_output(self as *const ::graphics_api_filter::GraphicsApiFilter, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_api_filter::GraphicsApiFilter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QGraphicsApiFilter_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsApiFilter`.
  pub struct Signals<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::vendorChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().vendor_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct VendorChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for VendorChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2vendorChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VendorChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::extensionsChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().extensions_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct ExtensionsChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for ExtensionsChanged<'a> {
    type Arguments = (&'static ::qt_core::string_list::StringList,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2extensionsChanged(const QStringList&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ExtensionsChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::objectNameChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct ObjectNameChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::graphicsApiFilterChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().graphics_api_filter_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct GraphicsApiFilterChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for GraphicsApiFilterChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2graphicsApiFilterChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GraphicsApiFilterChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::majorVersionChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().major_version_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct MajorVersionChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for MajorVersionChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2majorVersionChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MajorVersionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::minorVersionChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().minor_version_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct MinorVersionChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for MinorVersionChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2minorVersionChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MinorVersionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::apiChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().api_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct ApiChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for ApiChanged<'a> {
    type Arguments = (&'static ::graphics_api_filter::Api,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2apiChanged(Qt3DRender::QGraphicsApiFilter::Api)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ApiChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::profileChanged`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.signals().profile_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct ProfileChanged<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for ProfileChanged<'a> {
    type Arguments = (&'static ::graphics_api_filter::OpenGLProfile,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2profileChanged(Qt3DRender::QGraphicsApiFilter::OpenGLProfile)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ProfileChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::vendorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vendor_changed(&self) -> VendorChanged {
      VendorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::extensionsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn extensions_changed(&self) -> ExtensionsChanged {
      ExtensionsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::graphicsApiFilterChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn graphics_api_filter_changed(&self) -> GraphicsApiFilterChanged {
      GraphicsApiFilterChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::majorVersionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn major_version_changed(&self) -> MajorVersionChanged {
      MajorVersionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::minorVersionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minor_version_changed(&self) -> MinorVersionChanged {
      MinorVersionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::apiChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn api_changed(&self) -> ApiChanged {
      ApiChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGraphicsApiFilter::profileChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn profile_changed(&self) -> ProfileChanged {
      ProfileChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsApiFilter`.
  pub struct Slots<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  /// Represents a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setProfile`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.slots().set_profile()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct SetProfile<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for SetProfile<'a> {
    type Arguments = (::graphics_api_filter::OpenGLProfile,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setProfile(Qt3DRender::QGraphicsApiFilter::OpenGLProfile)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setVendor`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.slots().set_vendor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct SetVendor<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for SetVendor<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVendor(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setExtensions`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.slots().set_extensions()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct SetExtensions<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for SetExtensions<'a> {
    type Arguments = (&'static ::qt_core::string_list::StringList,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setExtensions(const QStringList&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setMinorVersion`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.slots().set_minor_version()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct SetMinorVersion<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for SetMinorVersion<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinorVersion(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setMajorVersion`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.slots().set_major_version()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct SetMajorVersion<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for SetMajorVersion<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMajorVersion(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setApi`.
  ///
  /// An object of this type can be created from `GraphicsApiFilter` with `object.slots().set_api()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsApiFilter` object.
  pub struct SetApi<'a>(&'a ::graphics_api_filter::GraphicsApiFilter);
  impl<'a> ::qt_core::connection::Receiver for SetApi<'a> {
    type Arguments = (::graphics_api_filter::Api,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setApi(Qt3DRender::QGraphicsApiFilter::Api)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setProfile`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_profile(&self) -> SetProfile {
      SetProfile(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setVendor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vendor(&self) -> SetVendor {
      SetVendor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setExtensions`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_extensions(&self) -> SetExtensions {
      SetExtensions(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setMinorVersion`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minor_version(&self) -> SetMinorVersion {
      SetMinorVersion(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setMajorVersion`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_major_version(&self) -> SetMajorVersion {
      SetMajorVersion(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGraphicsApiFilter::setApi`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_api(&self) -> SetApi {
      SetApi(self.0)
    }
  }
  impl ::graphics_api_filter::GraphicsApiFilter {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```Qt3DRender::QGraphicsApiFilter::OpenGLProfile```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OpenGLProfile {
  /// C++ enum variant: <span style='color: green;'>```NoProfile = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```CoreProfile = 1```</span>
  Core = 1,
  /// C++ enum variant: <span style='color: green;'>```CompatibilityProfile = 2```</span>
  Compatibility = 2,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_api_filter::GraphicsApiFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGraphicsApiFilter_G_static_cast_QObject_ptr(self as *mut ::graphics_api_filter::GraphicsApiFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGraphicsApiFilter_G_static_cast_QObject_ptr(self as *const ::graphics_api_filter::GraphicsApiFilter as *mut ::graphics_api_filter::GraphicsApiFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_api_filter::GraphicsApiFilter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_api_filter::GraphicsApiFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QGraphicsApiFilter_G_static_cast_Qt3DRender_QGraphicsApiFilter_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_api_filter::GraphicsApiFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QGraphicsApiFilter_G_static_cast_Qt3DRender_QGraphicsApiFilter_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_api_filter::GraphicsApiFilter {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGraphicsApiFilter_G_static_cast_QObject_ptr(self as *const ::graphics_api_filter::GraphicsApiFilter as *mut ::graphics_api_filter::GraphicsApiFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_api_filter::GraphicsApiFilter {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGraphicsApiFilter_G_static_cast_QObject_ptr(self as *mut ::graphics_api_filter::GraphicsApiFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

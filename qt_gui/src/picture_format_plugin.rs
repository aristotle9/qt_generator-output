/// C++ type: <span style='color: green;'>```QPictureFormatPlugin```</span>
#[repr(C)]
pub struct PictureFormatPlugin(u8);

impl PictureFormatPlugin {
  /// C++ method: <span style='color: green;'>```pure virtual bool QPictureFormatPlugin::installIOHandler(const QString& format)```</span>
  ///
  ///
  pub fn install_io_handler(&mut self, format: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPictureFormatPlugin_installIOHandler(self as *mut ::picture_format_plugin::PictureFormatPlugin,
                                                            format as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QPictureFormatPlugin::loadPicture(const QString& format, const QString& filename, QPicture* pic)```</span>
  ///
  ///
  pub unsafe fn load_picture(&mut self,
                             format: &::qt_core::string::String,
                             filename: &::qt_core::string::String,
                             pic: *mut ::picture::Picture)
                             -> bool {
    ::ffi::qt_gui_c_QPictureFormatPlugin_loadPicture(self as *mut ::picture_format_plugin::PictureFormatPlugin,
                                                     format as *const ::qt_core::string::String,
                                                     filename as *const ::qt_core::string::String,
                                                     pic)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPictureFormatPlugin::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_gui_c_QPictureFormatPlugin_metaObject(self as *const ::picture_format_plugin::PictureFormatPlugin)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPictureFormatPlugin::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QPictureFormatPlugin_qt_metacall(self as *mut ::picture_format_plugin::PictureFormatPlugin,
                                                     arg1 as *const ::qt_core::meta_object::Call,
                                                     arg2,
                                                     arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPictureFormatPlugin::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QPictureFormatPlugin_qt_metacast(self as *mut ::picture_format_plugin::PictureFormatPlugin,
                                                     arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QPictureFormatPlugin::savePicture(const QString& format, const QString& filename, const QPicture& pic)```</span>
  ///
  ///
  pub fn save_picture(&mut self,
                      format: &::qt_core::string::String,
                      filename: &::qt_core::string::String,
                      pic: &::picture::Picture)
                      -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPictureFormatPlugin_savePicture(self as *mut ::picture_format_plugin::PictureFormatPlugin,
                                                       format as *const ::qt_core::string::String,
                                                       filename as *const ::qt_core::string::String,
                                                       pic as *const ::picture::Picture)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPictureFormatPlugin::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPictureFormatPlugin_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPictureFormatPlugin::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPictureFormatPlugin_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::picture_format_plugin::PictureFormatPlugin {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPictureFormatPlugin_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PictureFormatPlugin`.
  pub struct Signals<'a>(&'a ::picture_format_plugin::PictureFormatPlugin);
  /// Represents a built-in Qt signal `QPictureFormatPlugin::objectNameChanged`.
  ///
  /// An object of this type can be created from `PictureFormatPlugin` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PictureFormatPlugin` object.
  pub struct ObjectNameChanged<'a>(&'a ::picture_format_plugin::PictureFormatPlugin);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPictureFormatPlugin::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::picture_format_plugin::PictureFormatPlugin {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::picture_format_plugin::PictureFormatPlugin {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPictureFormatPlugin_G_static_cast_QObject_ptr(self as *mut ::picture_format_plugin::PictureFormatPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPictureFormatPlugin_G_static_cast_QObject_ptr(self as *const ::picture_format_plugin::PictureFormatPlugin as *mut ::picture_format_plugin::PictureFormatPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::picture_format_plugin::PictureFormatPlugin> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::picture_format_plugin::PictureFormatPlugin {
    let ffi_result = ::ffi::qt_gui_c_QPictureFormatPlugin_G_static_cast_QPictureFormatPlugin_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::picture_format_plugin::PictureFormatPlugin {
    let ffi_result = ::ffi::qt_gui_c_QPictureFormatPlugin_G_static_cast_QPictureFormatPlugin_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::picture_format_plugin::PictureFormatPlugin {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPictureFormatPlugin_G_static_cast_QObject_ptr(self as *const ::picture_format_plugin::PictureFormatPlugin as *mut ::picture_format_plugin::PictureFormatPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::picture_format_plugin::PictureFormatPlugin {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPictureFormatPlugin_G_static_cast_QObject_ptr(self as *mut ::picture_format_plugin::PictureFormatPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

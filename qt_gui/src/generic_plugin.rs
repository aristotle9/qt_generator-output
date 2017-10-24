/// C++ type: <span style='color: green;'>```QGenericPlugin```</span>
#[repr(C)]
pub struct GenericPlugin(u8);

impl GenericPlugin {
  /// C++ method: <span style='color: green;'>```pure virtual QObject* QGenericPlugin::create(const QString& name, const QString& spec)```</span>
  ///
  ///
  pub fn create(&mut self,
                name: &::qt_core::string::String,
                spec: &::qt_core::string::String)
                -> *mut ::qt_core::object::Object {
    unsafe {
      ::ffi::qt_gui_c_QGenericPlugin_create(self as *mut ::generic_plugin::GenericPlugin,
                                            name as *const ::qt_core::string::String,
                                            spec as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGenericPlugin::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QGenericPlugin_metaObject(self as *const ::generic_plugin::GenericPlugin) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGenericPlugin::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QGenericPlugin_qt_metacall(self as *mut ::generic_plugin::GenericPlugin,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGenericPlugin::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QGenericPlugin_qt_metacast(self as *mut ::generic_plugin::GenericPlugin, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QGenericPlugin::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QGenericPlugin_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGenericPlugin::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QGenericPlugin_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::generic_plugin::GenericPlugin {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QGenericPlugin_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GenericPlugin`.
  pub struct Signals<'a>(&'a ::generic_plugin::GenericPlugin);
  /// Represents a built-in Qt signal `QGenericPlugin::objectNameChanged`.
  ///
  /// An object of this type can be created from `GenericPlugin` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GenericPlugin` object.
  pub struct ObjectNameChanged<'a>(&'a ::generic_plugin::GenericPlugin);
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
    /// Returns an object representing a built-in Qt signal `QGenericPlugin::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::generic_plugin::GenericPlugin {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::generic_plugin::GenericPlugin {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QGenericPlugin_G_static_cast_QObject_ptr(self as *mut ::generic_plugin::GenericPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGenericPlugin_G_static_cast_QObject_ptr(self as *const ::generic_plugin::GenericPlugin as *mut ::generic_plugin::GenericPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::generic_plugin::GenericPlugin> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::generic_plugin::GenericPlugin {
    let ffi_result =
      ::ffi::qt_gui_c_QGenericPlugin_G_static_cast_QGenericPlugin_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::generic_plugin::GenericPlugin {
    let ffi_result = ::ffi::qt_gui_c_QGenericPlugin_G_static_cast_QGenericPlugin_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::generic_plugin::GenericPlugin {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QGenericPlugin_G_static_cast_QObject_ptr(self as *const ::generic_plugin::GenericPlugin as *mut ::generic_plugin::GenericPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::generic_plugin::GenericPlugin {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QGenericPlugin_G_static_cast_QObject_ptr(self as *mut ::generic_plugin::GenericPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

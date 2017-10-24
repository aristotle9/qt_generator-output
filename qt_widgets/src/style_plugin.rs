/// C++ type: <span style='color: green;'>```QStylePlugin```</span>
#[repr(C)]
pub struct StylePlugin(u8);

impl StylePlugin {
  /// C++ method: <span style='color: green;'>```pure virtual QStyle* QStylePlugin::create(const QString& key)```</span>
  ///
  ///
  pub fn create(&mut self, key: &::qt_core::string::String) -> *mut ::style::Style {
    unsafe {
      ::ffi::qt_widgets_c_QStylePlugin_create(self as *mut ::style_plugin::StylePlugin,
                                              key as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStylePlugin::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QStylePlugin_metaObject(self as *const ::style_plugin::StylePlugin) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QStylePlugin::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStylePlugin_qt_metacall(self as *mut ::style_plugin::StylePlugin,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QStylePlugin::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QStylePlugin_qt_metacast(self as *mut ::style_plugin::StylePlugin, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QStylePlugin::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStylePlugin_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStylePlugin::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStylePlugin_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_plugin::StylePlugin {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStylePlugin_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StylePlugin`.
  pub struct Signals<'a>(&'a ::style_plugin::StylePlugin);
  /// Represents a built-in Qt signal `QStylePlugin::objectNameChanged`.
  ///
  /// An object of this type can be created from `StylePlugin` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StylePlugin` object.
  pub struct ObjectNameChanged<'a>(&'a ::style_plugin::StylePlugin);
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
    /// Returns an object representing a built-in Qt signal `QStylePlugin::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::style_plugin::StylePlugin {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::style_plugin::StylePlugin {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStylePlugin_G_static_cast_QObject_ptr(self as *mut ::style_plugin::StylePlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStylePlugin_G_static_cast_QObject_ptr(self as *const ::style_plugin::StylePlugin as *mut ::style_plugin::StylePlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_plugin::StylePlugin> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_plugin::StylePlugin {
    let ffi_result =
      ::ffi::qt_widgets_c_QStylePlugin_G_static_cast_QStylePlugin_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_plugin::StylePlugin {
    let ffi_result = ::ffi::qt_widgets_c_QStylePlugin_G_static_cast_QStylePlugin_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_plugin::StylePlugin {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStylePlugin_G_static_cast_QObject_ptr(self as *const ::style_plugin::StylePlugin as *mut ::style_plugin::StylePlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_plugin::StylePlugin {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStylePlugin_G_static_cast_QObject_ptr(self as *mut ::style_plugin::StylePlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

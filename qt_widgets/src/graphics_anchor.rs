/// C++ type: <span style='color: green;'>```QGraphicsAnchor```</span>
#[repr(C)]
pub struct GraphicsAnchor(u8);

impl GraphicsAnchor {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsAnchor::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchor_metaObject(self as *const ::graphics_anchor::GraphicsAnchor) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsAnchor::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsAnchor_qt_metacall(self as *mut ::graphics_anchor::GraphicsAnchor,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsAnchor::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsAnchor_qt_metacast(self as *mut ::graphics_anchor::GraphicsAnchor, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsAnchor::setSizePolicy(QSizePolicy::Policy policy)```</span>
  ///
  ///
  pub fn set_size_policy(&mut self, policy: &::size_policy::Policy) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsAnchor_setSizePolicy(self as *mut ::graphics_anchor::GraphicsAnchor,
                                                        policy as *const ::size_policy::Policy)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsAnchor::setSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchor_setSpacing(self as *mut ::graphics_anchor::GraphicsAnchor, spacing) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsAnchor::spacing() const```</span>
  ///
  ///
  pub fn spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchor_spacing(self as *const ::graphics_anchor::GraphicsAnchor) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsAnchor::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsAnchor_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsAnchor::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsAnchor_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsAnchor::unsetSpacing()```</span>
  ///
  ///
  pub fn unset_spacing(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchor_unsetSpacing(self as *mut ::graphics_anchor::GraphicsAnchor) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_anchor::GraphicsAnchor {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsAnchor_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsAnchor`.
  pub struct Signals<'a>(&'a ::graphics_anchor::GraphicsAnchor);
  /// Represents a built-in Qt signal `QGraphicsAnchor::objectNameChanged`.
  ///
  /// An object of this type can be created from `GraphicsAnchor` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsAnchor` object.
  pub struct ObjectNameChanged<'a>(&'a ::graphics_anchor::GraphicsAnchor);
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
    /// Returns an object representing a built-in Qt signal `QGraphicsAnchor::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::graphics_anchor::GraphicsAnchor {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_anchor::GraphicsAnchor {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsAnchor_G_static_cast_QObject_ptr(self as *mut ::graphics_anchor::GraphicsAnchor)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchor_G_static_cast_QObject_ptr(self as *const ::graphics_anchor::GraphicsAnchor as *mut ::graphics_anchor::GraphicsAnchor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_anchor::GraphicsAnchor> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_anchor::GraphicsAnchor {
    let ffi_result =
      ::ffi::qt_widgets_c_QGraphicsAnchor_G_static_cast_QGraphicsAnchor_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_anchor::GraphicsAnchor {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsAnchor_G_static_cast_QGraphicsAnchor_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_anchor::GraphicsAnchor {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchor_G_static_cast_QObject_ptr(self as *const ::graphics_anchor::GraphicsAnchor as *mut ::graphics_anchor::GraphicsAnchor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_anchor::GraphicsAnchor {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsAnchor_G_static_cast_QObject_ptr(self as *mut ::graphics_anchor::GraphicsAnchor)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsTransform```</span>
#[repr(C)]
pub struct GraphicsTransform(u8);

impl GraphicsTransform {
  /// C++ method: <span style='color: green;'>```pure virtual void QGraphicsTransform::applyTo(QMatrix4x4* matrix) const```</span>
  ///
  ///
  pub unsafe fn apply_to(&self, matrix: *mut ::qt_gui::matrix_4x4::Matrix4X4) {
    ::ffi::qt_widgets_c_QGraphicsTransform_applyTo(self as *const ::graphics_transform::GraphicsTransform,
                                                   matrix)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsTransform::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsTransform_metaObject(self as *const ::graphics_transform::GraphicsTransform) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsTransform::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsTransform_qt_metacall(self as *mut ::graphics_transform::GraphicsTransform,
                                                       arg1 as *const ::qt_core::meta_object::Call,
                                                       arg2,
                                                       arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsTransform::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsTransform_qt_metacast(self as *mut ::graphics_transform::GraphicsTransform, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsTransform::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsTransform_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsTransform::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsTransform_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_transform::GraphicsTransform {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsTransform_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsTransform`.
  pub struct Signals<'a>(&'a ::graphics_transform::GraphicsTransform);
  /// Represents a built-in Qt signal `QGraphicsTransform::objectNameChanged`.
  ///
  /// An object of this type can be created from `GraphicsTransform` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTransform` object.
  pub struct ObjectNameChanged<'a>(&'a ::graphics_transform::GraphicsTransform);
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
    /// Returns an object representing a built-in Qt signal `QGraphicsTransform::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsTransform`.
  pub struct Slots<'a>(&'a ::graphics_transform::GraphicsTransform);
  /// Represents a built-in Qt slot `QGraphicsTransform::update`.
  ///
  /// An object of this type can be created from `GraphicsTransform` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsTransform` object.
  pub struct Update<'a>(&'a ::graphics_transform::GraphicsTransform);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsTransform::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::graphics_transform::GraphicsTransform {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_transform::GraphicsTransform {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTransform_G_static_cast_QObject_ptr(self as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTransform_G_static_cast_QObject_ptr(self as *const ::graphics_transform::GraphicsTransform as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_transform::GraphicsTransform> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_transform::GraphicsTransform {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTransform_G_static_cast_QGraphicsTransform_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_transform::GraphicsTransform {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsTransform_G_static_cast_QGraphicsTransform_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_transform::GraphicsTransform {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTransform_G_static_cast_QObject_ptr(self as *const ::graphics_transform::GraphicsTransform as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_transform::GraphicsTransform {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsTransform_G_static_cast_QObject_ptr(self as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

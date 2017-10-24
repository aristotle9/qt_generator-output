/// C++ type: <span style='color: green;'>```QTextObject```</span>
#[repr(C)]
pub struct TextObject(u8);

impl TextObject {
  /// C++ method: <span style='color: green;'>```QTextDocument* QTextObject::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *mut ::text_document::TextDocument {
    unsafe { ::ffi::qt_gui_c_QTextObject_document(self as *const ::text_object::TextObject) }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat QTextObject::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::text_format::TextFormat {
    {
      let mut object: ::text_format::TextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextObject_format_to_output(self as *const ::text_object::TextObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextObject::formatIndex() const```</span>
  ///
  ///
  pub fn format_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextObject_formatIndex(self as *const ::text_object::TextObject) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextObject::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QTextObject_metaObject(self as *const ::text_object::TextObject) }
  }

  /// C++ method: <span style='color: green;'>```int QTextObject::objectIndex() const```</span>
  ///
  ///
  pub fn object_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextObject_objectIndex(self as *const ::text_object::TextObject) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextObject::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QTextObject_qt_metacall(self as *mut ::text_object::TextObject,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextObject::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QTextObject_qt_metacast(self as *mut ::text_object::TextObject, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QTextObject::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextObject_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextObject::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextObject_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextObject`.
  pub struct Signals<'a>(&'a ::text_object::TextObject);
  /// Represents a built-in Qt signal `QTextObject::objectNameChanged`.
  ///
  /// An object of this type can be created from `TextObject` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextObject` object.
  pub struct ObjectNameChanged<'a>(&'a ::text_object::TextObject);
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
    /// Returns an object representing a built-in Qt signal `QTextObject::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::text_object::TextObject {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_object::TextObject {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextObject_G_static_cast_QObject_ptr(self as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextObject_G_static_cast_QObject_ptr(self as *const ::text_object::TextObject as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_object::TextObject> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_object::TextObject {
    let ffi_result =
      ::ffi::qt_gui_c_QTextObject_G_static_cast_QTextObject_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_object::TextObject {
    let ffi_result = ::ffi::qt_gui_c_QTextObject_G_static_cast_QTextObject_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_object::TextObject {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextObject_G_static_cast_QObject_ptr(self as *const ::text_object::TextObject as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_object::TextObject {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextObject_G_static_cast_QObject_ptr(self as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

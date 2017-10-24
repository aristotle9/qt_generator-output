/// C++ type: <span style='color: green;'>```QOpenGLContextGroup```</span>
#[repr(C)]
pub struct OpenGLContextGroup(u8);

impl OpenGLContextGroup {
  /// C++ method: <span style='color: green;'>```static QOpenGLContextGroup* QOpenGLContextGroup::currentContextGroup()```</span>
  ///
  ///
  pub fn current_context_group() -> *mut ::opengl_context_group::OpenGLContextGroup {
    unsafe { ::ffi::qt_gui_c_QOpenGLContextGroup_currentContextGroup() }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLContextGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLContextGroup_metaObject(self as *const ::opengl_context_group::OpenGLContextGroup) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLContextGroup::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLContextGroup_qt_metacall(self as *mut ::opengl_context_group::OpenGLContextGroup,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLContextGroup::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLContextGroup_qt_metacast(self as *mut ::opengl_context_group::OpenGLContextGroup,
                                                    arg1)
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*> QOpenGLContextGroup::shares() const```</span>
  ///
  ///
  pub fn shares(&self) -> ::list::ListOpenglContextMutPtr {
    {
      let mut object: ::list::ListOpenglContextMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLContextGroup_shares_to_output(self as *const ::opengl_context_group::OpenGLContextGroup, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLContextGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLContextGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLContextGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLContextGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_context_group::OpenGLContextGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLContextGroup_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLContextGroup`.
  pub struct Signals<'a>(&'a ::opengl_context_group::OpenGLContextGroup);
  /// Represents a built-in Qt signal `QOpenGLContextGroup::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLContextGroup` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLContextGroup` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_context_group::OpenGLContextGroup);
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
    /// Returns an object representing a built-in Qt signal `QOpenGLContextGroup::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::opengl_context_group::OpenGLContextGroup {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_context_group::OpenGLContextGroup {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLContextGroup_G_static_cast_QObject_ptr(self as *mut ::opengl_context_group::OpenGLContextGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLContextGroup_G_static_cast_QObject_ptr(self as *const ::opengl_context_group::OpenGLContextGroup as *mut ::opengl_context_group::OpenGLContextGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_context_group::OpenGLContextGroup> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_context_group::OpenGLContextGroup {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLContextGroup_G_static_cast_QOpenGLContextGroup_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_context_group::OpenGLContextGroup {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLContextGroup_G_static_cast_QOpenGLContextGroup_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_context_group::OpenGLContextGroup {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLContextGroup_G_static_cast_QObject_ptr(self as *const ::opengl_context_group::OpenGLContextGroup as *mut ::opengl_context_group::OpenGLContextGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_context_group::OpenGLContextGroup {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLContextGroup_G_static_cast_QObject_ptr(self as *mut ::opengl_context_group::OpenGLContextGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

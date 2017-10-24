/// C++ type: <span style='color: green;'>```QOpenGLTimerQuery```</span>
#[repr(C)]
pub struct OpenGLTimerQuery(u8);

impl OpenGLTimerQuery {
  /// C++ method: <span style='color: green;'>```void QOpenGLTimerQuery::begin()```</span>
  ///
  ///
  pub fn begin(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_begin(self as *mut ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTimerQuery::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_create(self as *mut ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTimerQuery::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_destroy(self as *mut ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTimerQuery::end()```</span>
  ///
  ///
  pub fn end(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_end(self as *mut ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTimerQuery::isCreated() const```</span>
  ///
  ///
  pub fn is_created(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_isCreated(self as *const ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTimerQuery::isResultAvailable() const```</span>
  ///
  ///
  pub fn is_result_available(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTimerQuery_isResultAvailable(self as *const ::opengl_timer_query::OpenGLTimerQuery)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLTimerQuery::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_metaObject(self as *const ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTimerQuery::QOpenGLTimerQuery()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_timer_query::OpenGLTimerQuery> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTimerQuery::QOpenGLTimerQuery(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::opengl_timer_query::OpenGLTimerQuery> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLTimerQuery_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLTimerQuery::objectId() const```</span>
  ///
  ///
  pub fn object_id(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_objectId(self as *const ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLTimerQuery::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLTimerQuery_qt_metacall(self as *mut ::opengl_timer_query::OpenGLTimerQuery,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLTimerQuery::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLTimerQuery_qt_metacast(self as *mut ::opengl_timer_query::OpenGLTimerQuery, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTimerQuery::recordTimestamp()```</span>
  ///
  ///
  pub fn record_timestamp(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_recordTimestamp(self as *mut ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLTimerQuery::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLTimerQuery_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLTimerQuery::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLTimerQuery_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```GLuint64 QOpenGLTimerQuery::waitForResult() const```</span>
  ///
  ///
  pub fn wait_for_result(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_waitForResult(self as *const ::opengl_timer_query::OpenGLTimerQuery) }
  }

  /// C++ method: <span style='color: green;'>```GLuint64 QOpenGLTimerQuery::waitForTimestamp() const```</span>
  ///
  ///
  pub fn wait_for_timestamp(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_waitForTimestamp(self as *const ::opengl_timer_query::OpenGLTimerQuery) }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_timer_query::OpenGLTimerQuery {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLTimerQuery_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLTimerQuery`.
  pub struct Signals<'a>(&'a ::opengl_timer_query::OpenGLTimerQuery);
  /// Represents a built-in Qt signal `QOpenGLTimerQuery::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLTimerQuery` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLTimerQuery` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_timer_query::OpenGLTimerQuery);
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
    /// Returns an object representing a built-in Qt signal `QOpenGLTimerQuery::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::opengl_timer_query::OpenGLTimerQuery {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_timer_query::OpenGLTimerQuery {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTimerQuery_G_static_cast_QObject_ptr(self as *mut ::opengl_timer_query::OpenGLTimerQuery)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_G_static_cast_QObject_ptr(self as *const ::opengl_timer_query::OpenGLTimerQuery as *mut ::opengl_timer_query::OpenGLTimerQuery) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_timer_query::OpenGLTimerQuery> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_timer_query::OpenGLTimerQuery {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLTimerQuery_G_static_cast_QOpenGLTimerQuery_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_timer_query::OpenGLTimerQuery {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLTimerQuery_G_static_cast_QOpenGLTimerQuery_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_timer_query::OpenGLTimerQuery {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimerQuery_G_static_cast_QObject_ptr(self as *const ::opengl_timer_query::OpenGLTimerQuery as *mut ::opengl_timer_query::OpenGLTimerQuery) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_timer_query::OpenGLTimerQuery {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTimerQuery_G_static_cast_QObject_ptr(self as *mut ::opengl_timer_query::OpenGLTimerQuery)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// C++ type: <span style='color: green;'>```QOpenGLTimeMonitor```</span>
#[repr(C)]
pub struct OpenGLTimeMonitor(u8);

impl OpenGLTimeMonitor {
  /// C++ method: <span style='color: green;'>```bool QOpenGLTimeMonitor::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_create(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTimeMonitor::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_destroy(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTimeMonitor::isCreated() const```</span>
  ///
  ///
  pub fn is_created(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_isCreated(self as *const ::opengl_time_monitor::OpenGLTimeMonitor) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTimeMonitor::isResultAvailable() const```</span>
  ///
  ///
  pub fn is_result_available(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTimeMonitor_isResultAvailable(self as *const ::opengl_time_monitor::OpenGLTimeMonitor)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLTimeMonitor::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_metaObject(self as *const ::opengl_time_monitor::OpenGLTimeMonitor) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTimeMonitor::QOpenGLTimeMonitor()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_time_monitor::OpenGLTimeMonitor> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTimeMonitor::QOpenGLTimeMonitor(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::opengl_time_monitor::OpenGLTimeMonitor> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLTimeMonitor_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint> QOpenGLTimeMonitor::objectIds() const```</span>
  ///
  ///
  pub fn object_ids(&self) -> ::vector::VectorU32 {
    {
      let mut object: ::vector::VectorU32 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTimeMonitor_objectIds_to_output(self as *const ::opengl_time_monitor::OpenGLTimeMonitor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLTimeMonitor::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLTimeMonitor_qt_metacall(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLTimeMonitor::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLTimeMonitor_qt_metacast(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor, arg1)
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTimeMonitor::recordSample()```</span>
  ///
  ///
  pub fn record_sample(&mut self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_recordSample(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTimeMonitor::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_reset(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTimeMonitor::sampleCount() const```</span>
  ///
  ///
  pub fn sample_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_sampleCount(self as *const ::opengl_time_monitor::OpenGLTimeMonitor) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTimeMonitor::setSampleCount(int sampleCount)```</span>
  ///
  ///
  pub fn set_sample_count(&mut self, sample_count: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTimeMonitor_setSampleCount(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor,
                                                        sample_count)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLTimeMonitor::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLTimeMonitor_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLTimeMonitor::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLTimeMonitor_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64> QOpenGLTimeMonitor::waitForIntervals() const```</span>
  ///
  ///
  pub fn wait_for_intervals(&self) -> ::vector::VectorU64 {
    {
      let mut object: ::vector::VectorU64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTimeMonitor_waitForIntervals_to_output(self as *const ::opengl_time_monitor::OpenGLTimeMonitor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64> QOpenGLTimeMonitor::waitForSamples() const```</span>
  ///
  ///
  pub fn wait_for_samples(&self) -> ::vector::VectorU64 {
    {
      let mut object: ::vector::VectorU64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTimeMonitor_waitForSamples_to_output(self as *const ::opengl_time_monitor::OpenGLTimeMonitor, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_time_monitor::OpenGLTimeMonitor {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLTimeMonitor_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLTimeMonitor`.
  pub struct Signals<'a>(&'a ::opengl_time_monitor::OpenGLTimeMonitor);
  /// Represents a built-in Qt signal `QOpenGLTimeMonitor::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLTimeMonitor` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLTimeMonitor` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_time_monitor::OpenGLTimeMonitor);
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
    /// Returns an object representing a built-in Qt signal `QOpenGLTimeMonitor::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::opengl_time_monitor::OpenGLTimeMonitor {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_time_monitor::OpenGLTimeMonitor {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QObject_ptr(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QObject_ptr(self as *const ::opengl_time_monitor::OpenGLTimeMonitor as *mut ::opengl_time_monitor::OpenGLTimeMonitor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_time_monitor::OpenGLTimeMonitor> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_time_monitor::OpenGLTimeMonitor {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QOpenGLTimeMonitor_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_time_monitor::OpenGLTimeMonitor {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QOpenGLTimeMonitor_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_time_monitor::OpenGLTimeMonitor {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QObject_ptr(self as *const ::opengl_time_monitor::OpenGLTimeMonitor as *mut ::opengl_time_monitor::OpenGLTimeMonitor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_time_monitor::OpenGLTimeMonitor {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QObject_ptr(self as *mut ::opengl_time_monitor::OpenGLTimeMonitor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

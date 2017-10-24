/// C++ type: <span style='color: green;'>```QOpenGLVertexArrayObject::Binder```</span>
#[repr(C)]
pub struct Binder([u8; ::type_sizes::QT_GUI_OPENGL_VERTEX_ARRAY_OBJECT_BINDER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Binder {
  unsafe fn new_uninitialized() -> Binder {
    Binder(::std::mem::uninitialized())
  }
}

impl Binder {
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLVertexArrayObject::Binder::Binder(QOpenGLVertexArrayObject* v)```</span>
  ///
  ///
  pub unsafe fn new(v: *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject)
                    -> ::opengl_vertex_array_object::Binder {
    {
      let mut object: ::opengl_vertex_array_object::Binder =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLVertexArrayObject_Binder_constructor(v, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLVertexArrayObject::Binder::rebind()```</span>
  ///
  ///
  pub fn rebind(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_Binder_rebind(self as *mut ::opengl_vertex_array_object::Binder) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLVertexArrayObject::Binder::release()```</span>
  ///
  ///
  pub fn release(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVertexArrayObject_Binder_release(self as *mut ::opengl_vertex_array_object::Binder)
    }
  }
}

impl Drop for ::opengl_vertex_array_object::Binder {
  /// C++ method: <span style='color: green;'>```[destructor] void QOpenGLVertexArrayObject::Binder::~Binder()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVertexArrayObject_Binder_destructor(self as *mut ::opengl_vertex_array_object::Binder)
    }
  }
}

/// C++ type: <span style='color: green;'>```QOpenGLVertexArrayObject```</span>
#[repr(C)]
pub struct OpenGLVertexArrayObject(u8);

impl OpenGLVertexArrayObject {
  /// C++ method: <span style='color: green;'>```void QOpenGLVertexArrayObject::bind()```</span>
  ///
  ///
  pub fn bind(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLVertexArrayObject_bind(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLVertexArrayObject::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_create(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLVertexArrayObject::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_destroy(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLVertexArrayObject::isCreated() const```</span>
  ///
  ///
  pub fn is_created(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_isCreated(self as *const ::opengl_vertex_array_object::OpenGLVertexArrayObject) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLVertexArrayObject::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_metaObject(self as *const ::opengl_vertex_array_object::OpenGLVertexArrayObject) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_vertex_array_object::OpenGLVertexArrayObject> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::opengl_vertex_array_object::OpenGLVertexArrayObject> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLVertexArrayObject_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLVertexArrayObject::objectId() const```</span>
  ///
  ///
  pub fn object_id(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_objectId(self as *const ::opengl_vertex_array_object::OpenGLVertexArrayObject) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLVertexArrayObject::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLVertexArrayObject_qt_metacall(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLVertexArrayObject::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLVertexArrayObject_qt_metacast(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLVertexArrayObject::release()```</span>
  ///
  ///
  pub fn release(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_release(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject) }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLVertexArrayObject::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLVertexArrayObject_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLVertexArrayObject::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLVertexArrayObject_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_vertex_array_object::OpenGLVertexArrayObject {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLVertexArrayObject_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLVertexArrayObject`.
  pub struct Signals<'a>(&'a ::opengl_vertex_array_object::OpenGLVertexArrayObject);
  /// Represents a built-in Qt signal `QOpenGLVertexArrayObject::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLVertexArrayObject` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLVertexArrayObject` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_vertex_array_object::OpenGLVertexArrayObject);
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
    /// Returns an object representing a built-in Qt signal `QOpenGLVertexArrayObject::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::opengl_vertex_array_object::OpenGLVertexArrayObject {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_vertex_array_object::OpenGLVertexArrayObject {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QObject_ptr(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QObject_ptr(self as *const ::opengl_vertex_array_object::OpenGLVertexArrayObject as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_vertex_array_object::OpenGLVertexArrayObject> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_vertex_array_object::OpenGLVertexArrayObject {
let ffi_result = ::ffi::qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QOpenGLVertexArrayObject_ptr(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::opengl_vertex_array_object::OpenGLVertexArrayObject {
let ffi_result = ::ffi::qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QOpenGLVertexArrayObject_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::opengl_vertex_array_object::OpenGLVertexArrayObject {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QObject_ptr(self as *const ::opengl_vertex_array_object::OpenGLVertexArrayObject as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_vertex_array_object::OpenGLVertexArrayObject {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLVertexArrayObject_G_static_cast_QObject_ptr(self as *mut ::opengl_vertex_array_object::OpenGLVertexArrayObject) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// C++ type: <span style='color: green;'>```Qt3DCore::QAbstractAspect```</span>
#[repr(C)]
pub struct AbstractAspect(u8);

impl AbstractAspect {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DCore::QAbstractAspect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_metaObject(self as *const ::abstract_aspect::AbstractAspect) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QAbstractAspect::QAbstractAspect()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::abstract_aspect::AbstractAspect> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QAbstractAspect::QAbstractAspect(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::abstract_aspect::AbstractAspect> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DCore::QAbstractAspect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_qt_metacall(self as *mut ::abstract_aspect::AbstractAspect,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DCore::QAbstractAspect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_qt_metacast(self as *mut ::abstract_aspect::AbstractAspect, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QAbstractAspect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QAbstractAspect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_aspect::AbstractAspect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QAbstractAspect_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractAspect`.
  pub struct Signals<'a>(&'a ::abstract_aspect::AbstractAspect);
  /// Represents a built-in Qt signal `Qt3DCore::QAbstractAspect::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractAspect` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAspect` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_aspect::AbstractAspect);
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
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QAbstractAspect::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::abstract_aspect::AbstractAspect {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_aspect::AbstractAspect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QAbstractAspect_G_static_cast_QObject_ptr(self as *mut ::abstract_aspect::AbstractAspect)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QAbstractAspect_G_static_cast_QObject_ptr(self as *const ::abstract_aspect::AbstractAspect as *mut ::abstract_aspect::AbstractAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_aspect::AbstractAspect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_aspect::AbstractAspect {
    let ffi_result = ::ffi::qt_3d_core_c_QAbstractAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_aspect::AbstractAspect {
    let ffi_result = ::ffi::qt_3d_core_c_QAbstractAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_aspect::AbstractAspect {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QAbstractAspect_G_static_cast_QObject_ptr(self as *const ::abstract_aspect::AbstractAspect as *mut ::abstract_aspect::AbstractAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_aspect::AbstractAspect {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QAbstractAspect_G_static_cast_QObject_ptr(self as *mut ::abstract_aspect::AbstractAspect)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

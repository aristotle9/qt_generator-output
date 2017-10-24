/// C++ type: <span style='color: green;'>```Qt3DCore::QAspectEngine```</span>
#[repr(C)]
pub struct AspectEngine(u8);

impl AspectEngine {
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*> Qt3DCore::QAspectEngine::aspects() const```</span>
  ///
  ///
  pub fn aspects(&self) -> ::vector::VectorAbstractAspectMutPtr {
    {
      let mut object: ::vector::VectorAbstractAspectMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_aspects_to_output(self as *const ::aspect_engine::AspectEngine,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant Qt3DCore::QAspectEngine::executeCommand(const QString& command)```</span>
  ///
  ///
  pub fn execute_command(&mut self, command: &::qt_core::string::String) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_executeCommand_to_output(self as *mut ::aspect_engine::AspectEngine, command as *const ::qt_core::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DCore::QAspectEngine::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_metaObject(self as *const ::aspect_engine::AspectEngine) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QAspectEngine::QAspectEngine()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::aspect_engine::AspectEngine> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QAspectEngine::QAspectEngine(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::aspect_engine::AspectEngine> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DCore::QAspectEngine::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_qt_metacall(self as *mut ::aspect_engine::AspectEngine,
                                                           arg1 as *const ::qt_core::meta_object::Call,
                                                           arg2,
                                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DCore::QAspectEngine::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_qt_metacast(self as *mut ::aspect_engine::AspectEngine, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QAspectEngine::registerAspect(const QString& name)```</span>
  ///
  ///
  pub fn register_aspect(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_registerAspect_name(self as *mut ::aspect_engine::AspectEngine,
                                                                     name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QAspectEngine::registerAspect(Qt3DCore::QAbstractAspect* aspect)```</span>
  ///
  ///
  pub unsafe fn register_aspect_unsafe(&mut self, aspect: *mut ::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_registerAspect_aspect(self as *mut ::aspect_engine::AspectEngine, aspect)
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QEntity> Qt3DCore::QAspectEngine::rootEntity() const```</span>
  ///
  ///
  pub fn root_entity(&self) -> ::shared_pointer::SharedPointerEntity {
    {
      let mut object: ::shared_pointer::SharedPointerEntity =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_rootEntity_to_output(self as *const ::aspect_engine::AspectEngine,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QAspectEngine::setRootEntity(QSharedPointer<Qt3DCore::QEntity> root)```</span>
  ///
  ///
  pub fn set_root_entity(&mut self, root: &::shared_pointer::SharedPointerEntity) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_setRootEntity(self as *mut ::aspect_engine::AspectEngine,
                                                               root as *const ::shared_pointer::SharedPointerEntity)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QAspectEngine::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QAspectEngine::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QAspectEngine::unregisterAspect(const QString& name)```</span>
  ///
  ///
  pub fn unregister_aspect(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_unregisterAspect_name(self as *mut ::aspect_engine::AspectEngine,
                                                                       name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QAspectEngine::unregisterAspect(Qt3DCore::QAbstractAspect* aspect)```</span>
  ///
  ///
  pub unsafe fn unregister_aspect_unsafe(&mut self, aspect: *mut ::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_unregisterAspect_aspect(self as *mut ::aspect_engine::AspectEngine,
                                                                       aspect)
  }
}

impl ::cpp_utils::CppDeletable for ::aspect_engine::AspectEngine {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QAspectEngine_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AspectEngine`.
  pub struct Signals<'a>(&'a ::aspect_engine::AspectEngine);
  /// Represents a built-in Qt signal `Qt3DCore::QAspectEngine::objectNameChanged`.
  ///
  /// An object of this type can be created from `AspectEngine` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AspectEngine` object.
  pub struct ObjectNameChanged<'a>(&'a ::aspect_engine::AspectEngine);
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
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QAspectEngine::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::aspect_engine::AspectEngine {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::aspect_engine::AspectEngine {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QAspectEngine_G_static_cast_QObject_ptr(self as *mut ::aspect_engine::AspectEngine)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QAspectEngine_G_static_cast_QObject_ptr(self as *const ::aspect_engine::AspectEngine as *mut ::aspect_engine::AspectEngine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::aspect_engine::AspectEngine> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::aspect_engine::AspectEngine {
    let ffi_result = ::ffi::qt_3d_core_c_QAspectEngine_G_static_cast_Qt3DCore_QAspectEngine_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::aspect_engine::AspectEngine {
    let ffi_result = ::ffi::qt_3d_core_c_QAspectEngine_G_static_cast_Qt3DCore_QAspectEngine_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::aspect_engine::AspectEngine {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QAspectEngine_G_static_cast_QObject_ptr(self as *const ::aspect_engine::AspectEngine as *mut ::aspect_engine::AspectEngine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::aspect_engine::AspectEngine {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QAspectEngine_G_static_cast_QObject_ptr(self as *mut ::aspect_engine::AspectEngine)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// C++ type: <span style='color: green;'>```Qt3DLogic::QLogicAspect```</span>
#[repr(C)]
pub struct LogicAspect(u8);

impl LogicAspect {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DLogic::QLogicAspect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_metaObject(self as *const ::logic_aspect::LogicAspect) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DLogic::QLogicAspect::QLogicAspect()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::logic_aspect::LogicAspect> {
    let ffi_result = unsafe { ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DLogic::QLogicAspect::QLogicAspect(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::logic_aspect::LogicAspect> {
    let ffi_result = ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DLogic::QLogicAspect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_qt_metacall(self as *mut ::logic_aspect::LogicAspect,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DLogic::QLogicAspect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_qt_metacast(self as *mut ::logic_aspect::LogicAspect, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DLogic::QLogicAspect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DLogic::QLogicAspect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::logic_aspect::LogicAspect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_logic_c_Qt3DLogic_QLogicAspect_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::abstract_aspect::AbstractAspect> for ::logic_aspect::LogicAspect {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *mut ::logic_aspect::LogicAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *const ::logic_aspect::LogicAspect as *mut ::logic_aspect::LogicAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::logic_aspect::LogicAspect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_QObject_ptr(self as *mut ::logic_aspect::LogicAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_QObject_ptr(self as *const ::logic_aspect::LogicAspect as *mut ::logic_aspect::LogicAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::logic_aspect::LogicAspect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::logic_aspect::LogicAspect {
    let ffi_result = ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DLogic_QLogicAspect_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::logic_aspect::LogicAspect {
    let ffi_result = ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DLogic_QLogicAspect_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::logic_aspect::LogicAspect> for ::qt_3d_core::abstract_aspect::AbstractAspect {
  unsafe fn static_cast_mut(&mut self) -> &mut ::logic_aspect::LogicAspect {
    let ffi_result = ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DLogic_QLogicAspect_ptr_Qt3DCore_QAbstractAspect(self as *mut ::qt_3d_core::abstract_aspect::AbstractAspect);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::logic_aspect::LogicAspect {
    let ffi_result = ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DLogic_QLogicAspect_ptr_Qt3DCore_QAbstractAspect(self as *const ::qt_3d_core::abstract_aspect::AbstractAspect as *mut ::qt_3d_core::abstract_aspect::AbstractAspect);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::logic_aspect::LogicAspect {
  type Target = ::qt_3d_core::abstract_aspect::AbstractAspect;
  fn deref(&self) -> &::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *const ::logic_aspect::LogicAspect as *mut ::logic_aspect::LogicAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::logic_aspect::LogicAspect {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_logic_c_QLogicAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *mut ::logic_aspect::LogicAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

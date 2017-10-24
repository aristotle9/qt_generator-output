/// C++ type: <span style='color: green;'>```Qt3DInput::QInputAspect```</span>
#[repr(C)]
pub struct InputAspect(u8);

impl InputAspect {
  /// C++ method: <span style='color: green;'>```QStringList Qt3DInput::QInputAspect::availablePhysicalDevices() const```</span>
  ///
  ///
  pub fn available_physical_devices(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_availablePhysicalDevices_to_output(self as *const ::input_aspect::InputAspect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractPhysicalDevice* Qt3DInput::QInputAspect::createPhysicalDevice(const QString& name)```</span>
  ///
  ///
  pub fn create_physical_device(&mut self,
                                name: &::qt_core::string::String)
                                -> *mut ::abstract_physical_device::AbstractPhysicalDevice {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_createPhysicalDevice(self as *mut ::input_aspect::InputAspect,
                                                                       name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QInputAspect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_metaObject(self as *const ::input_aspect::InputAspect) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QInputAspect::QInputAspect()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::input_aspect::InputAspect> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QInputAspect::QInputAspect(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::input_aspect::InputAspect> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QInputAspect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_qt_metacall(self as *mut ::input_aspect::InputAspect,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QInputAspect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_qt_metacast(self as *mut ::input_aspect::InputAspect, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QInputAspect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QInputAspect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::input_aspect::InputAspect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputAspect_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::abstract_aspect::AbstractAspect> for ::input_aspect::InputAspect {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *mut ::input_aspect::InputAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *const ::input_aspect::InputAspect as *mut ::input_aspect::InputAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::input_aspect::InputAspect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_QObject_ptr(self as *mut ::input_aspect::InputAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_QObject_ptr(self as *const ::input_aspect::InputAspect as *mut ::input_aspect::InputAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_aspect::InputAspect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_aspect::InputAspect {
    let ffi_result = ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_aspect::InputAspect {
    let ffi_result = ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_aspect::InputAspect> for ::qt_3d_core::abstract_aspect::AbstractAspect {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_aspect::InputAspect {
    let ffi_result = ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_Qt3DCore_QAbstractAspect(self as *mut ::qt_3d_core::abstract_aspect::AbstractAspect);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_aspect::InputAspect {
    let ffi_result = ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DInput_QInputAspect_ptr_Qt3DCore_QAbstractAspect(self as *const ::qt_3d_core::abstract_aspect::AbstractAspect as *mut ::qt_3d_core::abstract_aspect::AbstractAspect);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_aspect::InputAspect {
  type Target = ::qt_3d_core::abstract_aspect::AbstractAspect;
  fn deref(&self) -> &::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *const ::input_aspect::InputAspect as *mut ::input_aspect::InputAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_aspect::InputAspect {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *mut ::input_aspect::InputAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// C++ type: <span style='color: green;'>```QPanGesture```</span>
#[repr(C)]
pub struct PanGesture(u8);

impl PanGesture {
  /// C++ method: <span style='color: green;'>```double QPanGesture::acceleration() const```</span>
  ///
  ///
  pub fn acceleration(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QPanGesture_acceleration(self as *const ::pan_gesture::PanGesture) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QPanGesture::delta() const```</span>
  ///
  ///
  pub fn delta(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPanGesture_delta_to_output(self as *const ::pan_gesture::PanGesture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QPanGesture::lastOffset() const```</span>
  ///
  ///
  pub fn last_offset(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPanGesture_lastOffset_to_output(self as *const ::pan_gesture::PanGesture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPanGesture::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QPanGesture_metaObject(self as *const ::pan_gesture::PanGesture) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPanGesture::QPanGesture()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::pan_gesture::PanGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPanGesture_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPanGesture::QPanGesture(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::pan_gesture::PanGesture> {
    let ffi_result = ::ffi::qt_widgets_c_QPanGesture_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QPointF QPanGesture::offset() const```</span>
  ///
  ///
  pub fn offset(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPanGesture_offset_to_output(self as *const ::pan_gesture::PanGesture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPanGesture::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QPanGesture_qt_metacall(self as *mut ::pan_gesture::PanGesture,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPanGesture::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QPanGesture_qt_metacast(self as *mut ::pan_gesture::PanGesture, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QPanGesture::setAcceleration(double value)```</span>
  ///
  ///
  pub fn set_acceleration(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QPanGesture_setAcceleration(self as *mut ::pan_gesture::PanGesture, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPanGesture::setLastOffset(const QPointF& value)```</span>
  ///
  ///
  pub fn set_last_offset(&mut self, value: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QPanGesture_setLastOffset(self as *mut ::pan_gesture::PanGesture,
                                                    value as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPanGesture::setOffset(const QPointF& value)```</span>
  ///
  ///
  pub fn set_offset(&mut self, value: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QPanGesture_setOffset(self as *mut ::pan_gesture::PanGesture,
                                                value as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPanGesture::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPanGesture_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPanGesture::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPanGesture_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::pan_gesture::PanGesture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QPanGesture_delete
  }
}

impl ::cpp_utils::DynamicCast<::pan_gesture::PanGesture> for ::gesture::Gesture {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pan_gesture::PanGesture> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPanGesture_G_dynamic_cast_QPanGesture_ptr(self as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pan_gesture::PanGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPanGesture_G_dynamic_cast_QPanGesture_ptr(self as *const ::gesture::Gesture as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::pan_gesture::PanGesture {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QObject_ptr(self as *mut ::pan_gesture::PanGesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QObject_ptr(self as *const ::pan_gesture::PanGesture as *mut ::pan_gesture::PanGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::gesture::Gesture> for ::pan_gesture::PanGesture {
  fn static_cast_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QGesture_ptr(self as *mut ::pan_gesture::PanGesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QGesture_ptr(self as *const ::pan_gesture::PanGesture as *mut ::pan_gesture::PanGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pan_gesture::PanGesture> for ::gesture::Gesture {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pan_gesture::PanGesture {
    let ffi_result =
      ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QGesture(self as *mut ::gesture::Gesture);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pan_gesture::PanGesture {
    let ffi_result = ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QGesture(self as *const ::gesture::Gesture as *mut ::gesture::Gesture);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pan_gesture::PanGesture> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pan_gesture::PanGesture {
    let ffi_result =
      ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pan_gesture::PanGesture {
    let ffi_result = ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QPanGesture_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::pan_gesture::PanGesture {
  type Target = ::gesture::Gesture;
  fn deref(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QGesture_ptr(self as *const ::pan_gesture::PanGesture as *mut ::pan_gesture::PanGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::pan_gesture::PanGesture {
  fn deref_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPanGesture_G_static_cast_QGesture_ptr(self as *mut ::pan_gesture::PanGesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

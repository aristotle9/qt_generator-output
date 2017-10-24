/// C++ type: <span style='color: green;'>```QTapGesture```</span>
#[repr(C)]
pub struct TapGesture(u8);

impl TapGesture {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTapGesture::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTapGesture_metaObject(self as *const ::tap_gesture::TapGesture) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTapGesture::QTapGesture()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::tap_gesture::TapGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTapGesture_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTapGesture::QTapGesture(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::tap_gesture::TapGesture> {
    let ffi_result = ::ffi::qt_widgets_c_QTapGesture_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QPointF QTapGesture::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTapGesture_position_to_output(self as *const ::tap_gesture::TapGesture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QTapGesture::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTapGesture_qt_metacall(self as *mut ::tap_gesture::TapGesture,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTapGesture::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTapGesture_qt_metacast(self as *mut ::tap_gesture::TapGesture, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTapGesture::setPosition(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_position(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QTapGesture_setPosition(self as *mut ::tap_gesture::TapGesture,
                                                  pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTapGesture::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTapGesture_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTapGesture::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTapGesture_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::tap_gesture::TapGesture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTapGesture_delete
  }
}

impl ::cpp_utils::DynamicCast<::tap_gesture::TapGesture> for ::gesture::Gesture {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tap_gesture::TapGesture> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTapGesture_G_dynamic_cast_QTapGesture_ptr(self as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tap_gesture::TapGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTapGesture_G_dynamic_cast_QTapGesture_ptr(self as *const ::gesture::Gesture as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tap_gesture::TapGesture {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QObject_ptr(self as *mut ::tap_gesture::TapGesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QObject_ptr(self as *const ::tap_gesture::TapGesture as *mut ::tap_gesture::TapGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::gesture::Gesture> for ::tap_gesture::TapGesture {
  fn static_cast_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QGesture_ptr(self as *mut ::tap_gesture::TapGesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QGesture_ptr(self as *const ::tap_gesture::TapGesture as *mut ::tap_gesture::TapGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tap_gesture::TapGesture> for ::gesture::Gesture {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tap_gesture::TapGesture {
    let ffi_result =
      ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QGesture(self as *mut ::gesture::Gesture);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tap_gesture::TapGesture {
    let ffi_result = ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QGesture(self as *const ::gesture::Gesture as *mut ::gesture::Gesture);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tap_gesture::TapGesture> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tap_gesture::TapGesture {
    let ffi_result =
      ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tap_gesture::TapGesture {
    let ffi_result = ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QTapGesture_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tap_gesture::TapGesture {
  type Target = ::gesture::Gesture;
  fn deref(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QGesture_ptr(self as *const ::tap_gesture::TapGesture as *mut ::tap_gesture::TapGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tap_gesture::TapGesture {
  fn deref_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTapGesture_G_static_cast_QGesture_ptr(self as *mut ::tap_gesture::TapGesture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

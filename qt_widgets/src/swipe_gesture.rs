/// C++ type: <span style='color: green;'>```QSwipeGesture::SwipeDirection```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SwipeDirection {
  /// C++ enum variant: <span style='color: green;'>```NoDirection = 0```</span>
  NoDirection = 0,
  /// C++ enum variant: <span style='color: green;'>```Left = 1```</span>
  Left = 1,
  /// C++ enum variant: <span style='color: green;'>```Right = 2```</span>
  Right = 2,
  /// C++ enum variant: <span style='color: green;'>```Up = 3```</span>
  Up = 3,
  /// C++ enum variant: <span style='color: green;'>```Down = 4```</span>
  Down = 4,
}

/// C++ type: <span style='color: green;'>```QSwipeGesture```</span>
#[repr(C)]
pub struct SwipeGesture(u8);

impl SwipeGesture {
  /// C++ method: <span style='color: green;'>```QSwipeGesture::SwipeDirection QSwipeGesture::horizontalDirection() const```</span>
  ///
  ///
  pub fn horizontal_direction(&self) -> ::swipe_gesture::SwipeDirection {
    unsafe { ::ffi::qt_widgets_c_QSwipeGesture_horizontalDirection(self as *const ::swipe_gesture::SwipeGesture) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSwipeGesture::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QSwipeGesture_metaObject(self as *const ::swipe_gesture::SwipeGesture) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSwipeGesture::QSwipeGesture()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::swipe_gesture::SwipeGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSwipeGesture_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSwipeGesture::QSwipeGesture(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::swipe_gesture::SwipeGesture> {
    let ffi_result = ::ffi::qt_widgets_c_QSwipeGesture_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QSwipeGesture::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSwipeGesture_qt_metacall(self as *mut ::swipe_gesture::SwipeGesture,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSwipeGesture::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QSwipeGesture_qt_metacast(self as *mut ::swipe_gesture::SwipeGesture, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSwipeGesture::setSwipeAngle(double value)```</span>
  ///
  ///
  pub fn set_swipe_angle(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QSwipeGesture_setSwipeAngle(self as *mut ::swipe_gesture::SwipeGesture, value) }
  }

  /// C++ method: <span style='color: green;'>```double QSwipeGesture::swipeAngle() const```</span>
  ///
  ///
  pub fn swipe_angle(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QSwipeGesture_swipeAngle(self as *const ::swipe_gesture::SwipeGesture) }
  }

  /// C++ method: <span style='color: green;'>```static QString QSwipeGesture::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSwipeGesture_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSwipeGesture::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSwipeGesture_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSwipeGesture::SwipeDirection QSwipeGesture::verticalDirection() const```</span>
  ///
  ///
  pub fn vertical_direction(&self) -> ::swipe_gesture::SwipeDirection {
    unsafe { ::ffi::qt_widgets_c_QSwipeGesture_verticalDirection(self as *const ::swipe_gesture::SwipeGesture) }
  }
}

impl ::cpp_utils::CppDeletable for ::swipe_gesture::SwipeGesture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSwipeGesture_delete
  }
}

impl ::cpp_utils::DynamicCast<::swipe_gesture::SwipeGesture> for ::gesture::Gesture {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::swipe_gesture::SwipeGesture> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSwipeGesture_G_dynamic_cast_QSwipeGesture_ptr(self as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::swipe_gesture::SwipeGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSwipeGesture_G_dynamic_cast_QSwipeGesture_ptr(self as *const ::gesture::Gesture as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::swipe_gesture::SwipeGesture {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QObject_ptr(self as *mut ::swipe_gesture::SwipeGesture)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QObject_ptr(self as *const ::swipe_gesture::SwipeGesture as *mut ::swipe_gesture::SwipeGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::gesture::Gesture> for ::swipe_gesture::SwipeGesture {
  fn static_cast_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QGesture_ptr(self as *mut ::swipe_gesture::SwipeGesture)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QGesture_ptr(self as *const ::swipe_gesture::SwipeGesture as *mut ::swipe_gesture::SwipeGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::swipe_gesture::SwipeGesture> for ::gesture::Gesture {
  unsafe fn static_cast_mut(&mut self) -> &mut ::swipe_gesture::SwipeGesture {
    let ffi_result =
      ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QGesture(self as *mut ::gesture::Gesture);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::swipe_gesture::SwipeGesture {
    let ffi_result = ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QGesture(self as *const ::gesture::Gesture as *mut ::gesture::Gesture);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::swipe_gesture::SwipeGesture> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::swipe_gesture::SwipeGesture {
    let ffi_result = ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::swipe_gesture::SwipeGesture {
    let ffi_result = ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QSwipeGesture_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::swipe_gesture::SwipeGesture {
  type Target = ::gesture::Gesture;
  fn deref(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QGesture_ptr(self as *const ::swipe_gesture::SwipeGesture as *mut ::swipe_gesture::SwipeGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::swipe_gesture::SwipeGesture {
  fn deref_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSwipeGesture_G_static_cast_QGesture_ptr(self as *mut ::swipe_gesture::SwipeGesture)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

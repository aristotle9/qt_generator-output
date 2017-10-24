/// C++ type: <span style='color: green;'>```QPinchGesture::ChangeFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ChangeFlag {
  /// C++ enum variant: <span style='color: green;'>```ScaleFactorChanged = 1```</span>
  ScaleFactor = 1,
  /// C++ enum variant: <span style='color: green;'>```RotationAngleChanged = 2```</span>
  RotationAngle = 2,
  /// C++ enum variant: <span style='color: green;'>```CenterPointChanged = 4```</span>
  CenterPoint = 4,
}

impl ::qt_core::flags::FlaggableEnum for ChangeFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ChangeFlag"
  }
}

/// C++ type: <span style='color: green;'>```QPinchGesture```</span>
#[repr(C)]
pub struct PinchGesture(u8);

impl PinchGesture {
  /// C++ method: <span style='color: green;'>```QPointF QPinchGesture::centerPoint() const```</span>
  ///
  ///
  pub fn center_point(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPinchGesture_centerPoint_to_output(self as *const ::pinch_gesture::PinchGesture,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QPinchGesture::ChangeFlag> QPinchGesture::changeFlags() const```</span>
  ///
  ///
  pub fn change_flags(&self) -> ::qt_core::flags::Flags<::pinch_gesture::ChangeFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPinchGesture_changeFlags(self as *const ::pinch_gesture::PinchGesture) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QPointF QPinchGesture::lastCenterPoint() const```</span>
  ///
  ///
  pub fn last_center_point(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPinchGesture_lastCenterPoint_to_output(self as *const ::pinch_gesture::PinchGesture,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QPinchGesture::lastRotationAngle() const```</span>
  ///
  ///
  pub fn last_rotation_angle(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_lastRotationAngle(self as *const ::pinch_gesture::PinchGesture) }
  }

  /// C++ method: <span style='color: green;'>```double QPinchGesture::lastScaleFactor() const```</span>
  ///
  ///
  pub fn last_scale_factor(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_lastScaleFactor(self as *const ::pinch_gesture::PinchGesture) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPinchGesture::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_metaObject(self as *const ::pinch_gesture::PinchGesture) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPinchGesture::QPinchGesture()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::pinch_gesture::PinchGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPinchGesture_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPinchGesture::QPinchGesture(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::pinch_gesture::PinchGesture> {
    let ffi_result = ::ffi::qt_widgets_c_QPinchGesture_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QPinchGesture::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QPinchGesture_qt_metacall(self as *mut ::pinch_gesture::PinchGesture,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPinchGesture::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QPinchGesture_qt_metacast(self as *mut ::pinch_gesture::PinchGesture, arg1)
  }

  /// C++ method: <span style='color: green;'>```double QPinchGesture::rotationAngle() const```</span>
  ///
  ///
  pub fn rotation_angle(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_rotationAngle(self as *const ::pinch_gesture::PinchGesture) }
  }

  /// C++ method: <span style='color: green;'>```double QPinchGesture::scaleFactor() const```</span>
  ///
  ///
  pub fn scale_factor(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_scaleFactor(self as *const ::pinch_gesture::PinchGesture) }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setCenterPoint(const QPointF& value)```</span>
  ///
  ///
  pub fn set_center_point(&mut self, value: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QPinchGesture_setCenterPoint(self as *mut ::pinch_gesture::PinchGesture,
                                                       value as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setChangeFlags(QFlags<QPinchGesture::ChangeFlag> value)```</span>
  ///
  ///
  pub fn set_change_flags(&mut self, value: ::qt_core::flags::Flags<::pinch_gesture::ChangeFlag>) {
    unsafe {
      ::ffi::qt_widgets_c_QPinchGesture_setChangeFlags(self as *mut ::pinch_gesture::PinchGesture,
                                                       value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setLastCenterPoint(const QPointF& value)```</span>
  ///
  ///
  pub fn set_last_center_point(&mut self, value: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QPinchGesture_setLastCenterPoint(self as *mut ::pinch_gesture::PinchGesture,
                                                           value as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setLastRotationAngle(double value)```</span>
  ///
  ///
  pub fn set_last_rotation_angle(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_setLastRotationAngle(self as *mut ::pinch_gesture::PinchGesture, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setLastScaleFactor(double value)```</span>
  ///
  ///
  pub fn set_last_scale_factor(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_setLastScaleFactor(self as *mut ::pinch_gesture::PinchGesture, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setRotationAngle(double value)```</span>
  ///
  ///
  pub fn set_rotation_angle(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_setRotationAngle(self as *mut ::pinch_gesture::PinchGesture, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setScaleFactor(double value)```</span>
  ///
  ///
  pub fn set_scale_factor(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_setScaleFactor(self as *mut ::pinch_gesture::PinchGesture, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setStartCenterPoint(const QPointF& value)```</span>
  ///
  ///
  pub fn set_start_center_point(&mut self, value: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_widgets_c_QPinchGesture_setStartCenterPoint(self as *mut ::pinch_gesture::PinchGesture,
                                                            value as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setTotalChangeFlags(QFlags<QPinchGesture::ChangeFlag> value)```</span>
  ///
  ///
  pub fn set_total_change_flags(&mut self, value: ::qt_core::flags::Flags<::pinch_gesture::ChangeFlag>) {
    unsafe {
      ::ffi::qt_widgets_c_QPinchGesture_setTotalChangeFlags(self as *mut ::pinch_gesture::PinchGesture,
                                                            value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setTotalRotationAngle(double value)```</span>
  ///
  ///
  pub fn set_total_rotation_angle(&mut self, value: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QPinchGesture_setTotalRotationAngle(self as *mut ::pinch_gesture::PinchGesture, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPinchGesture::setTotalScaleFactor(double value)```</span>
  ///
  ///
  pub fn set_total_scale_factor(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_setTotalScaleFactor(self as *mut ::pinch_gesture::PinchGesture, value) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QPinchGesture::startCenterPoint() const```</span>
  ///
  ///
  pub fn start_center_point(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPinchGesture_startCenterPoint_to_output(self as *const ::pinch_gesture::PinchGesture,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QPinchGesture::ChangeFlag> QPinchGesture::totalChangeFlags() const```</span>
  ///
  ///
  pub fn total_change_flags(&self) -> ::qt_core::flags::Flags<::pinch_gesture::ChangeFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPinchGesture_totalChangeFlags(self as *const ::pinch_gesture::PinchGesture) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```double QPinchGesture::totalRotationAngle() const```</span>
  ///
  ///
  pub fn total_rotation_angle(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_totalRotationAngle(self as *const ::pinch_gesture::PinchGesture) }
  }

  /// C++ method: <span style='color: green;'>```double QPinchGesture::totalScaleFactor() const```</span>
  ///
  ///
  pub fn total_scale_factor(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QPinchGesture_totalScaleFactor(self as *const ::pinch_gesture::PinchGesture) }
  }

  /// C++ method: <span style='color: green;'>```static QString QPinchGesture::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPinchGesture_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPinchGesture::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPinchGesture_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::pinch_gesture::PinchGesture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QPinchGesture_delete
  }
}

impl ::cpp_utils::DynamicCast<::pinch_gesture::PinchGesture> for ::gesture::Gesture {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pinch_gesture::PinchGesture> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPinchGesture_G_dynamic_cast_QPinchGesture_ptr(self as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pinch_gesture::PinchGesture> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPinchGesture_G_dynamic_cast_QPinchGesture_ptr(self as *const ::gesture::Gesture as *mut ::gesture::Gesture) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::pinch_gesture::PinchGesture {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QObject_ptr(self as *mut ::pinch_gesture::PinchGesture)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QObject_ptr(self as *const ::pinch_gesture::PinchGesture as *mut ::pinch_gesture::PinchGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::gesture::Gesture> for ::pinch_gesture::PinchGesture {
  fn static_cast_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QGesture_ptr(self as *mut ::pinch_gesture::PinchGesture)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QGesture_ptr(self as *const ::pinch_gesture::PinchGesture as *mut ::pinch_gesture::PinchGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pinch_gesture::PinchGesture> for ::gesture::Gesture {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pinch_gesture::PinchGesture {
    let ffi_result =
      ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QGesture(self as *mut ::gesture::Gesture);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pinch_gesture::PinchGesture {
    let ffi_result = ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QGesture(self as *const ::gesture::Gesture as *mut ::gesture::Gesture);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pinch_gesture::PinchGesture> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pinch_gesture::PinchGesture {
    let ffi_result = ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pinch_gesture::PinchGesture {
    let ffi_result = ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QPinchGesture_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::pinch_gesture::PinchGesture {
  type Target = ::gesture::Gesture;
  fn deref(&self) -> &::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QGesture_ptr(self as *const ::pinch_gesture::PinchGesture as *mut ::pinch_gesture::PinchGesture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::pinch_gesture::PinchGesture {
  fn deref_mut(&mut self) -> &mut ::gesture::Gesture {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPinchGesture_G_static_cast_QGesture_ptr(self as *mut ::pinch_gesture::PinchGesture)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

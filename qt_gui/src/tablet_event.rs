/// C++ type: <span style='color: green;'>```QTabletEvent::PointerType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PointerType {
  /// C++ enum variant: <span style='color: green;'>```UnknownPointer = 0```</span>
  UnknownPointer = 0,
  /// C++ enum variant: <span style='color: green;'>```Pen = 1```</span>
  Pen = 1,
  /// C++ enum variant: <span style='color: green;'>```Cursor = 2```</span>
  Cursor = 2,
  /// C++ enum variant: <span style='color: green;'>```Eraser = 3```</span>
  Eraser = 3,
}

/// C++ type: <span style='color: green;'>```QTabletEvent::TabletDevice```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabletDevice {
  /// C++ enum variant: <span style='color: green;'>```NoDevice = 0```</span>
  NoDevice = 0,
  /// C++ enum variant: <span style='color: green;'>```Puck = 1```</span>
  Puck = 1,
  /// C++ enum variant: <span style='color: green;'>```Stylus = 2```</span>
  Stylus = 2,
  /// C++ enum variant: <span style='color: green;'>```Airbrush = 3```</span>
  Airbrush = 3,
  /// C++ enum variant: <span style='color: green;'>```FourDMouse = 4```</span>
  FourDMouse = 4,
  /// C++ enum variant: <span style='color: green;'>```XFreeEraser = 5```</span>
  XFreeEraser = 5,
  /// C++ enum variant: <span style='color: green;'>```RotationStylus = 6```</span>
  RotationStylus = 6,
}

/// C++ type: <span style='color: green;'>```QTabletEvent```</span>
#[repr(C)]
pub struct TabletEvent(u8);

impl TabletEvent {
  /// C++ method: <span style='color: green;'>```QTabletEvent::TabletDevice QTabletEvent::device() const```</span>
  ///
  ///
  pub fn device(&self) -> ::tablet_event::TabletDevice {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_device(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QTabletEvent::globalPos() const```</span>
  ///
  ///
  pub fn global_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTabletEvent_globalPos_to_output(self as *const ::tablet_event::TabletEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QTabletEvent::globalPosF() const```</span>
  ///
  ///
  pub fn global_pos_f<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTabletEvent_globalPosF(self as *const ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QTabletEvent::globalX() const```</span>
  ///
  ///
  pub fn global_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_globalX(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QTabletEvent::globalY() const```</span>
  ///
  ///
  pub fn global_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_globalY(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```double QTabletEvent::hiResGlobalX() const```</span>
  ///
  ///
  pub fn hi_res_global_x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_hiResGlobalX(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```double QTabletEvent::hiResGlobalY() const```</span>
  ///
  ///
  pub fn hi_res_global_y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_hiResGlobalY(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```QTabletEvent::PointerType QTabletEvent::pointerType() const```</span>
  ///
  ///
  pub fn pointer_type(&self) -> ::tablet_event::PointerType {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_pointerType(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QTabletEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTabletEvent_pos_to_output(self as *const ::tablet_event::TabletEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QTabletEvent::posF() const```</span>
  ///
  ///
  pub fn pos_f<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTabletEvent_posF(self as *const ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double QTabletEvent::pressure() const```</span>
  ///
  ///
  pub fn pressure(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_pressure(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```double QTabletEvent::rotation() const```</span>
  ///
  ///
  pub fn rotation(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_rotation(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```double QTabletEvent::tangentialPressure() const```</span>
  ///
  ///
  pub fn tangential_pressure(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_tangentialPressure(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QTabletEvent::uniqueId() const```</span>
  ///
  ///
  pub fn unique_id(&self) -> i64 {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_uniqueId(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QTabletEvent::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_x(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QTabletEvent::xTilt() const```</span>
  ///
  ///
  pub fn x_tilt(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_xTilt(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QTabletEvent::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_y(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QTabletEvent::yTilt() const```</span>
  ///
  ///
  pub fn y_tilt(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_yTilt(self as *const ::tablet_event::TabletEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QTabletEvent::z() const```</span>
  ///
  ///
  pub fn z(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTabletEvent_z(self as *const ::tablet_event::TabletEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::tablet_event::TabletEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTabletEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::tablet_event::TabletEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tablet_event::TabletEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTabletEvent_G_dynamic_cast_QTabletEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tablet_event::TabletEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTabletEvent_G_dynamic_cast_QTabletEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::tablet_event::TabletEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QEvent_ptr(self as *mut ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QEvent_ptr(self as *const ::tablet_event::TabletEvent as *mut ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::tablet_event::TabletEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QInputEvent_ptr(self as *mut ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QInputEvent_ptr(self as *const ::tablet_event::TabletEvent as *mut ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tablet_event::TabletEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tablet_event::TabletEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tablet_event::TabletEvent {
    let ffi_result = ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tablet_event::TabletEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tablet_event::TabletEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tablet_event::TabletEvent {
    let ffi_result = ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QTabletEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tablet_event::TabletEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QInputEvent_ptr(self as *const ::tablet_event::TabletEvent as *mut ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tablet_event::TabletEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTabletEvent_G_static_cast_QInputEvent_ptr(self as *mut ::tablet_event::TabletEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

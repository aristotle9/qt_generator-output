/// C++ type: <span style='color: green;'>```QActionEvent```</span>
#[repr(C)]
pub struct ActionEvent(u8);

impl ::cpp_utils::CppDeletable for ::action_event::ActionEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QActionEvent_delete
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::pointing_device_unique_id::PointingDeviceUniqueId) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(QPointingDeviceUniqueId key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::pointing_device_unique_id::PointingDeviceUniqueId, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(QPointingDeviceUniqueId key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator==(QPointingDeviceUniqueId lhs, QPointingDeviceUniqueId rhs)```</span>
///
///
pub fn op_eq(lhs: &::pointing_device_unique_id::PointingDeviceUniqueId,
             rhs: &::pointing_device_unique_id::PointingDeviceUniqueId)
             -> bool {
  unsafe { ::ffi::qt_gui_c_QActionEvent_G_operator_eq_lhs_rhs(lhs as *const ::pointing_device_unique_id::PointingDeviceUniqueId, rhs as *const ::pointing_device_unique_id::PointingDeviceUniqueId) }
}

/// C++ method: <span style='color: green;'>```operator==```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_eq_unsafe((*mut ::key_event::KeyEvent, &::key_sequence::StandardKey)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(QKeyEvent* e, QKeySequence::StandardKey key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_eq_unsafe((&::key_sequence::StandardKey, *mut ::key_event::KeyEvent)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(QKeySequence::StandardKey key, QKeyEvent* e)```</span>
///
///
pub unsafe fn op_eq_unsafe<Args>(args: Args) -> bool
  where Args: overloading::OpEqUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator!=(QPointingDeviceUniqueId lhs, QPointingDeviceUniqueId rhs)```</span>
///
///
pub fn op_neq(lhs: &::pointing_device_unique_id::PointingDeviceUniqueId,
              rhs: &::pointing_device_unique_id::PointingDeviceUniqueId)
              -> bool {
  unsafe {
    ::ffi::qt_gui_c_QActionEvent_G_operator_neq(lhs as *const ::pointing_device_unique_id::PointingDeviceUniqueId,
                                                rhs as *const ::pointing_device_unique_id::PointingDeviceUniqueId)
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTouchEvent::TouchPoint& arg2)```</span>
///
///
pub fn op_shl(arg1: &::qt_core::debug::Debug, arg2: &::touch_event::TouchPoint) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QActionEvent_G_operator_shl_to_output_QDebug_QTouchEvent_TouchPoint(arg1 as *const ::qt_core::debug::Debug, arg2 as *const ::touch_event::TouchPoint, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QEvent* arg2)```</span>
///
///
pub unsafe fn op_shl_unsafe(arg1: &::qt_core::debug::Debug,
                            arg2: *const ::qt_core::event::Event)
                            -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_gui_c_QActionEvent_G_operator_shl_to_output_QDebug_QEvent(arg1 as *const ::qt_core::debug::Debug,
                                                                        arg2,
                                                                        &mut object);
    object
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::action_event::ActionEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QActionEvent_G_static_cast_QEvent_ptr(self as *mut ::action_event::ActionEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QActionEvent_G_static_cast_QEvent_ptr(self as *const ::action_event::ActionEvent as *mut ::action_event::ActionEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::action_event::ActionEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::action_event::ActionEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QActionEvent_G_static_cast_QActionEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::action_event::ActionEvent {
    let ffi_result = ::ffi::qt_gui_c_QActionEvent_G_static_cast_QActionEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::action_event::ActionEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QActionEvent_G_static_cast_QEvent_ptr(self as *const ::action_event::ActionEvent as *mut ::action_event::ActionEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::action_event::ActionEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QActionEvent_G_static_cast_QEvent_ptr(self as *mut ::action_event::ActionEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::pointing_device_unique_id::PointingDeviceUniqueId {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe {
        ::ffi::qt_gui_c_QActionEvent_G_qHash_key(key as *const ::pointing_device_unique_id::PointingDeviceUniqueId)
      }
    }
  }
  impl<'a> HashArgs for (&'a ::pointing_device_unique_id::PointingDeviceUniqueId, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe {
        ::ffi::qt_gui_c_QActionEvent_G_qHash_key_seed(key as *const ::pointing_device_unique_id::PointingDeviceUniqueId, seed)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_eq_unsafe](../fn.op_eq_unsafe.html) method.
  pub trait OpEqUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> OpEqUnsafeArgs for (*mut ::key_event::KeyEvent, &'a ::key_sequence::StandardKey) {
    unsafe fn exec(self) -> bool {
      let e = self.0;
      let key = self.1;
      ::ffi::qt_gui_c_QActionEvent_G_operator_eq_e_key(e, key as *const ::key_sequence::StandardKey)
    }
  }
  impl<'a> OpEqUnsafeArgs for (&'a ::key_sequence::StandardKey, *mut ::key_event::KeyEvent) {
    unsafe fn exec(self) -> bool {
      let key = self.0;
      let e = self.1;
      ::ffi::qt_gui_c_QActionEvent_G_operator_eq_key_e(key as *const ::key_sequence::StandardKey, e)
    }
  }
}

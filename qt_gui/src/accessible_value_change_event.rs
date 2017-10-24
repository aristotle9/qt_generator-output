/// C++ type: <span style='color: green;'>```QAccessibleValueChangeEvent```</span>
#[repr(C)]
pub struct AccessibleValueChangeEvent(u8);

impl AccessibleValueChangeEvent {
  /// C++ method: <span style='color: green;'>```QAccessibleValueChangeEvent::QAccessibleValueChangeEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, &::qt_core::variant::Variant)) -> ::cpp_utils::CppBox<::accessible_value_change_event::AccessibleValueChangeEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleValueChangeEvent::QAccessibleValueChangeEvent(QAccessibleInterface* iface, const QVariant& val)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, &::qt_core::variant::Variant)) -> ::cpp_utils::CppBox<::accessible_value_change_event::AccessibleValueChangeEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleValueChangeEvent::QAccessibleValueChangeEvent(QObject* obj, const QVariant& val)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args)
                          -> ::cpp_utils::CppBox<::accessible_value_change_event::AccessibleValueChangeEvent>
    where Args: overloading::AccessibleValueChangeEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QAccessibleValueChangeEvent::setValue(const QVariant& val)```</span>
  ///
  ///
  pub fn set_value(&mut self, val: &::qt_core::variant::Variant) {
    unsafe { ::ffi::qt_gui_c_QAccessibleValueChangeEvent_setValue(self as *mut ::accessible_value_change_event::AccessibleValueChangeEvent, val as *const ::qt_core::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QAccessibleValueChangeEvent::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleValueChangeEvent_value_to_output(self as *const ::accessible_value_change_event::AccessibleValueChangeEvent, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_value_change_event::AccessibleValueChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleValueChangeEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::accessible_value_change_event::AccessibleValueChangeEvent> for ::accessible_event::AccessibleEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_value_change_event::AccessibleValueChangeEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_dynamic_cast_QAccessibleValueChangeEvent_ptr(self as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_value_change_event::AccessibleValueChangeEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_dynamic_cast_QAccessibleValueChangeEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::accessible_event::AccessibleEvent> for ::accessible_value_change_event::AccessibleValueChangeEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_value_change_event::AccessibleValueChangeEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_value_change_event::AccessibleValueChangeEvent as *mut ::accessible_value_change_event::AccessibleValueChangeEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_value_change_event::AccessibleValueChangeEvent> for ::accessible_event::AccessibleEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_value_change_event::AccessibleValueChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleValueChangeEvent_ptr(self as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_value_change_event::AccessibleValueChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleValueChangeEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_value_change_event::AccessibleValueChangeEvent {
  type Target = ::accessible_event::AccessibleEvent;
  fn deref(&self) -> &::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_value_change_event::AccessibleValueChangeEvent as *mut ::accessible_value_change_event::AccessibleValueChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_value_change_event::AccessibleValueChangeEvent {
  fn deref_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleValueChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_value_change_event::AccessibleValueChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleValueChangeEvent::new](../struct.AccessibleValueChangeEvent.html#method.new) method.
  pub trait AccessibleValueChangeEventNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_value_change_event::AccessibleValueChangeEvent>;
  }
  impl<'a> AccessibleValueChangeEventNewArgs
    for (*mut ::accessible_interface::AccessibleInterface, &'a ::qt_core::variant::Variant) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_value_change_event::AccessibleValueChangeEvent> {
      let iface = self.0;
      let val = self.1;
      let ffi_result =
        ::ffi::qt_gui_c_QAccessibleValueChangeEvent_new_iface_val(iface, val as *const ::qt_core::variant::Variant);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AccessibleValueChangeEventNewArgs for (*mut ::qt_core::object::Object, &'a ::qt_core::variant::Variant) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_value_change_event::AccessibleValueChangeEvent> {
      let obj = self.0;
      let val = self.1;
      let ffi_result =
        ::ffi::qt_gui_c_QAccessibleValueChangeEvent_new_obj_val(obj, val as *const ::qt_core::variant::Variant);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

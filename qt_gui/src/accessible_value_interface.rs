/// C++ type: <span style='color: green;'>```QAccessibleValueInterface```</span>
#[repr(C)]
pub struct AccessibleValueInterface(u8);

impl AccessibleValueInterface {
  /// C++ method: <span style='color: green;'>```pure virtual QVariant QAccessibleValueInterface::currentValue() const```</span>
  ///
  ///
  pub fn current_value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleValueInterface_currentValue_to_output(self as *const ::accessible_value_interface::AccessibleValueInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QVariant QAccessibleValueInterface::maximumValue() const```</span>
  ///
  ///
  pub fn maximum_value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleValueInterface_maximumValue_to_output(self as *const ::accessible_value_interface::AccessibleValueInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QVariant QAccessibleValueInterface::minimumStepSize() const```</span>
  ///
  ///
  pub fn minimum_step_size(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleValueInterface_minimumStepSize_to_output(self as *const ::accessible_value_interface::AccessibleValueInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QVariant QAccessibleValueInterface::minimumValue() const```</span>
  ///
  ///
  pub fn minimum_value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleValueInterface_minimumValue_to_output(self as *const ::accessible_value_interface::AccessibleValueInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleValueInterface::setCurrentValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_current_value(&mut self, value: &::qt_core::variant::Variant) {
    unsafe { ::ffi::qt_gui_c_QAccessibleValueInterface_setCurrentValue(self as *mut ::accessible_value_interface::AccessibleValueInterface, value as *const ::qt_core::variant::Variant) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_value_interface::AccessibleValueInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleValueInterface_delete
  }
}

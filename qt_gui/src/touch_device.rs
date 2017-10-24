/// C++ type: <span style='color: green;'>```QTouchDevice::CapabilityFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CapabilityFlag {
  /// C++ enum variant: <span style='color: green;'>```Position = 1```</span>
  Position = 1,
  /// C++ enum variant: <span style='color: green;'>```Area = 2```</span>
  Area = 2,
  /// C++ enum variant: <span style='color: green;'>```Pressure = 4```</span>
  Pressure = 4,
  /// C++ enum variant: <span style='color: green;'>```Velocity = 8```</span>
  Velocity = 8,
  /// C++ enum variant: <span style='color: green;'>```RawPositions = 16```</span>
  RawPositions = 16,
  /// C++ enum variant: <span style='color: green;'>```NormalizedPosition = 32```</span>
  NormalizedPosition = 32,
  /// C++ enum variant: <span style='color: green;'>```MouseEmulation = 64```</span>
  MouseEmulation = 64,
}

impl ::qt_core::flags::FlaggableEnum for CapabilityFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "CapabilityFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTouchDevice::DeviceType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DeviceType {
  /// C++ enum variant: <span style='color: green;'>```TouchScreen = 0```</span>
  Screen = 0,
  /// C++ enum variant: <span style='color: green;'>```TouchPad = 1```</span>
  Pad = 1,
}

/// C++ type: <span style='color: green;'>```QTouchDevice```</span>
#[repr(C)]
pub struct TouchDevice(u8);

impl TouchDevice {
  /// C++ method: <span style='color: green;'>```QFlags<QTouchDevice::CapabilityFlag> QTouchDevice::capabilities() const```</span>
  ///
  ///
  pub fn capabilities(&self) -> ::qt_core::flags::Flags<::touch_device::CapabilityFlag> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchDevice_capabilities(self as *const ::touch_device::TouchDevice) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```static QList<const QTouchDevice*> QTouchDevice::devices()```</span>
  ///
  ///
  pub fn devices() -> ::list::ListTouchDevicePtr {
    {
      let mut object: ::list::ListTouchDevicePtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchDevice_devices_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTouchDevice::maximumTouchPoints() const```</span>
  ///
  ///
  pub fn maximum_touch_points(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTouchDevice_maximumTouchPoints(self as *const ::touch_device::TouchDevice) }
  }

  /// C++ method: <span style='color: green;'>```QString QTouchDevice::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchDevice_name_to_output(self as *const ::touch_device::TouchDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTouchDevice::QTouchDevice()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::touch_device::TouchDevice> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchDevice_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchDevice::setCapabilities(QFlags<QTouchDevice::CapabilityFlag> caps)```</span>
  ///
  ///
  pub fn set_capabilities(&mut self, caps: ::qt_core::flags::Flags<::touch_device::CapabilityFlag>) {
    unsafe {
      ::ffi::qt_gui_c_QTouchDevice_setCapabilities(self as *mut ::touch_device::TouchDevice,
                                                   caps.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchDevice::setMaximumTouchPoints(int max)```</span>
  ///
  ///
  pub fn set_maximum_touch_points(&mut self, max: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTouchDevice_setMaximumTouchPoints(self as *mut ::touch_device::TouchDevice, max) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchDevice::setName(const QString& name)```</span>
  ///
  ///
  pub fn set_name(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTouchDevice_setName(self as *mut ::touch_device::TouchDevice,
                                           name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchDevice::setType(QTouchDevice::DeviceType devType)```</span>
  ///
  ///
  pub fn set_type(&mut self, dev_type: ::touch_device::DeviceType) {
    unsafe { ::ffi::qt_gui_c_QTouchDevice_setType(self as *mut ::touch_device::TouchDevice, dev_type) }
  }

  /// C++ method: <span style='color: green;'>```QTouchDevice::DeviceType QTouchDevice::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::touch_device::DeviceType {
    unsafe { ::ffi::qt_gui_c_QTouchDevice_type(self as *const ::touch_device::TouchDevice) }
  }
}

impl ::cpp_utils::CppDeletable for ::touch_device::TouchDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTouchDevice_delete
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QTouchDevice* arg2)```</span>
///
///
pub unsafe fn op_shl(arg1: &::qt_core::debug::Debug,
                     arg2: *const ::touch_device::TouchDevice)
                     -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_gui_c_QTouchDevice_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug, arg2, &mut object);
    object
  }
}

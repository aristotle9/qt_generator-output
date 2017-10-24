/// C++ type: <span style='color: green;'>```QPointingDeviceUniqueId```</span>
#[repr(C)]
pub struct PointingDeviceUniqueId([u8; ::type_sizes::QT_GUI_POINTING_DEVICE_UNIQUE_ID_POINTING_DEVICE_UNIQUE_ID]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PointingDeviceUniqueId {
  unsafe fn new_uninitialized() -> PointingDeviceUniqueId {
    PointingDeviceUniqueId(::std::mem::uninitialized())
  }
}

impl PointingDeviceUniqueId {
  /// C++ method: <span style='color: green;'>```static QPointingDeviceUniqueId QPointingDeviceUniqueId::fromNumericId(qint64 id)```</span>
  ///
  ///
  pub fn from_numeric_id(id: i64) -> ::pointing_device_unique_id::PointingDeviceUniqueId {
    {
      let mut object: ::pointing_device_unique_id::PointingDeviceUniqueId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPointingDeviceUniqueId_fromNumericId_to_output(id, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPointingDeviceUniqueId::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPointingDeviceUniqueId_isValid(self as *const ::pointing_device_unique_id::PointingDeviceUniqueId) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPointingDeviceUniqueId::QPointingDeviceUniqueId()```</span>
  ///
  ///
  pub fn new() -> ::pointing_device_unique_id::PointingDeviceUniqueId {
    {
      let mut object: ::pointing_device_unique_id::PointingDeviceUniqueId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPointingDeviceUniqueId_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QPointingDeviceUniqueId::numericId() const```</span>
  ///
  ///
  pub fn numeric_id(&self) -> i64 {
    unsafe { ::ffi::qt_gui_c_QPointingDeviceUniqueId_numericId(self as *const ::pointing_device_unique_id::PointingDeviceUniqueId) }
  }
}

impl Drop for ::pointing_device_unique_id::PointingDeviceUniqueId {
  /// C++ method: <span style='color: green;'>```[destructor] void QPointingDeviceUniqueId::~QPointingDeviceUniqueId()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPointingDeviceUniqueId_destructor(self as *mut ::pointing_device_unique_id::PointingDeviceUniqueId) }
  }
}

/// C++ type: <span style='color: green;'>```QPaintDevice```</span>
#[repr(C)]
pub struct PaintDevice(u8);

impl PaintDevice {
  /// C++ method: <span style='color: green;'>```int QPaintDevice::colorCount() const```</span>
  ///
  ///
  pub fn color_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_colorCount(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_depth(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPaintDevice::devType() const```</span>
  ///
  ///
  pub fn dev_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_devType(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::devicePixelRatio() const```</span>
  ///
  ///
  pub fn device_pixel_ratio(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_devicePixelRatio(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```double QPaintDevice::devicePixelRatioF() const```</span>
  ///
  ///
  pub fn device_pixel_ratio_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_devicePixelRatioF(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```static double QPaintDevice::devicePixelRatioFScale()```</span>
  ///
  ///
  pub fn device_pixel_ratio_f_scale() -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_devicePixelRatioFScale() }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_height(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::heightMM() const```</span>
  ///
  ///
  pub fn height_m_m(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_heightMM(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::logicalDpiX() const```</span>
  ///
  ///
  pub fn logical_dpi_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_logicalDpiX(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::logicalDpiY() const```</span>
  ///
  ///
  pub fn logical_dpi_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_logicalDpiY(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QPaintEngine* QPaintDevice::paintEngine() const```</span>
  ///
  ///
  pub fn paint_engine(&self) -> *mut ::paint_engine::PaintEngine {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_paintEngine(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```bool QPaintDevice::paintingActive() const```</span>
  ///
  ///
  pub fn painting_active(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_paintingActive(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::physicalDpiX() const```</span>
  ///
  ///
  pub fn physical_dpi_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_physicalDpiX(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::physicalDpiY() const```</span>
  ///
  ///
  pub fn physical_dpi_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_physicalDpiY(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_width(self as *const ::paint_device::PaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QPaintDevice::widthMM() const```</span>
  ///
  ///
  pub fn width_m_m(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPaintDevice_widthMM(self as *const ::paint_device::PaintDevice) }
  }
}

impl ::cpp_utils::CppDeletable for ::paint_device::PaintDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPaintDevice_delete
  }
}

/// C++ type: <span style='color: green;'>```QPaintDevice::PaintDeviceMetric```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PaintDeviceMetric {
  /// C++ enum variant: <span style='color: green;'>```PdmWidth = 1```</span>
  Width = 1,
  /// C++ enum variant: <span style='color: green;'>```PdmHeight = 2```</span>
  Height = 2,
  /// C++ enum variant: <span style='color: green;'>```PdmWidthMM = 3```</span>
  WidthMM = 3,
  /// C++ enum variant: <span style='color: green;'>```PdmHeightMM = 4```</span>
  HeightMM = 4,
  /// C++ enum variant: <span style='color: green;'>```PdmNumColors = 5```</span>
  NumColors = 5,
  /// C++ enum variant: <span style='color: green;'>```PdmDepth = 6```</span>
  Depth = 6,
  /// C++ enum variant: <span style='color: green;'>```PdmDpiX = 7```</span>
  DpiX = 7,
  /// C++ enum variant: <span style='color: green;'>```PdmDpiY = 8```</span>
  DpiY = 8,
  /// C++ enum variant: <span style='color: green;'>```PdmPhysicalDpiX = 9```</span>
  PhysicalDpiX = 9,
  /// C++ enum variant: <span style='color: green;'>```PdmPhysicalDpiY = 10```</span>
  PhysicalDpiY = 10,
  /// C++ enum variant: <span style='color: green;'>```PdmDevicePixelRatio = 11```</span>
  DevicePixelRatio = 11,
  /// C++ enum variant: <span style='color: green;'>```PdmDevicePixelRatioScaled = 12```</span>
  DevicePixelRatioScaled = 12,
}

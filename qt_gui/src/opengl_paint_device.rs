/// C++ type: <span style='color: green;'>```QOpenGLPaintDevice```</span>
#[repr(C)]
pub struct OpenGLPaintDevice(u8);

impl OpenGLPaintDevice {
  /// C++ method: <span style='color: green;'>```QOpenGLContext* QOpenGLPaintDevice::context() const```</span>
  ///
  ///
  pub fn context(&self) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_context(self as *const ::opengl_paint_device::OpenGLPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLPaintDevice::devType() const```</span>
  ///
  ///
  pub fn dev_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_devType(self as *const ::opengl_paint_device::OpenGLPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```double QOpenGLPaintDevice::dotsPerMeterX() const```</span>
  ///
  ///
  pub fn dots_per_meter_x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_dotsPerMeterX(self as *const ::opengl_paint_device::OpenGLPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```double QOpenGLPaintDevice::dotsPerMeterY() const```</span>
  ///
  ///
  pub fn dots_per_meter_y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_dotsPerMeterY(self as *const ::opengl_paint_device::OpenGLPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QOpenGLPaintDevice::ensureActiveTarget()```</span>
  ///
  ///
  pub fn ensure_active_target(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLPaintDevice_ensureActiveTarget(self as *mut ::opengl_paint_device::OpenGLPaintDevice)
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLPaintDevice::QOpenGLPaintDevice```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLPaintDevice::QOpenGLPaintDevice()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::size::Size) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLPaintDevice::QOpenGLPaintDevice(const QSize& size)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLPaintDevice::QOpenGLPaintDevice(int width, int height)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice>
    where Args: overloading::OpenGLPaintDeviceNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPaintEngine* QOpenGLPaintDevice::paintEngine() const```</span>
  ///
  ///
  pub fn paint_engine(&self) -> *mut ::paint_engine::PaintEngine {
    unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_paintEngine(self as *const ::opengl_paint_device::OpenGLPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLPaintDevice::paintFlipped() const```</span>
  ///
  ///
  pub fn paint_flipped(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_paintFlipped(self as *const ::opengl_paint_device::OpenGLPaintDevice) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPaintDevice::setDevicePixelRatio(double devicePixelRatio)```</span>
  ///
  ///
  pub fn set_device_pixel_ratio(&mut self, device_pixel_ratio: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLPaintDevice_setDevicePixelRatio(self as *mut ::opengl_paint_device::OpenGLPaintDevice,
                                                             device_pixel_ratio)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPaintDevice::setDotsPerMeterX(double arg1)```</span>
  ///
  ///
  pub fn set_dots_per_meter_x(&mut self, arg1: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLPaintDevice_setDotsPerMeterX(self as *mut ::opengl_paint_device::OpenGLPaintDevice, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPaintDevice::setDotsPerMeterY(double arg1)```</span>
  ///
  ///
  pub fn set_dots_per_meter_y(&mut self, arg1: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLPaintDevice_setDotsPerMeterY(self as *mut ::opengl_paint_device::OpenGLPaintDevice, arg1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPaintDevice::setPaintFlipped(bool flipped)```</span>
  ///
  ///
  pub fn set_paint_flipped(&mut self, flipped: bool) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLPaintDevice_setPaintFlipped(self as *mut ::opengl_paint_device::OpenGLPaintDevice,
                                                         flipped)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPaintDevice::setSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLPaintDevice_setSize(self as *mut ::opengl_paint_device::OpenGLPaintDevice,
                                                 size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QOpenGLPaintDevice::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLPaintDevice_size_to_output(self as *const ::opengl_paint_device::OpenGLPaintDevice,
                                                          &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_paint_device::OpenGLPaintDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLPaintDevice_delete
  }
}

impl ::cpp_utils::DynamicCast<::opengl_paint_device::OpenGLPaintDevice> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::opengl_paint_device::OpenGLPaintDevice> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_G_dynamic_cast_QOpenGLPaintDevice_ptr(self as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::opengl_paint_device::OpenGLPaintDevice> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_G_dynamic_cast_QOpenGLPaintDevice_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::opengl_paint_device::OpenGLPaintDevice {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_G_static_cast_QPaintDevice_ptr(self as *mut ::opengl_paint_device::OpenGLPaintDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_G_static_cast_QPaintDevice_ptr(self as *const ::opengl_paint_device::OpenGLPaintDevice as *mut ::opengl_paint_device::OpenGLPaintDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_paint_device::OpenGLPaintDevice> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_paint_device::OpenGLPaintDevice {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLPaintDevice_G_static_cast_QOpenGLPaintDevice_ptr(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_paint_device::OpenGLPaintDevice {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLPaintDevice_G_static_cast_QOpenGLPaintDevice_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_paint_device::OpenGLPaintDevice {
  type Target = ::paint_device::PaintDevice;
  fn deref(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_G_static_cast_QPaintDevice_ptr(self as *const ::opengl_paint_device::OpenGLPaintDevice as *mut ::opengl_paint_device::OpenGLPaintDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_paint_device::OpenGLPaintDevice {
  fn deref_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_G_static_cast_QPaintDevice_ptr(self as *mut ::opengl_paint_device::OpenGLPaintDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLPaintDevice::new](../struct.OpenGLPaintDevice.html#method.new) method.
  pub trait OpenGLPaintDeviceNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice>;
  }
  impl OpenGLPaintDeviceNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLPaintDeviceNewArgs for &'a ::qt_core::size::Size {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice> {
      let size = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_new_size(size as *const ::qt_core::size::Size) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLPaintDeviceNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_paint_device::OpenGLPaintDevice> {
      let width = self.0;
      let height = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPaintDevice_new_width_height(width, height) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}

/// C++ type: <span style='color: green;'>```QRasterWindow```</span>
#[repr(C)]
pub struct RasterWindow(u8);

impl RasterWindow {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QRasterWindow::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QRasterWindow_metaObject(self as *const ::raster_window::RasterWindow) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QRasterWindow::QRasterWindow()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::raster_window::RasterWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QRasterWindow::QRasterWindow(QWindow* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::window::Window) -> ::cpp_utils::CppBox<::raster_window::RasterWindow> {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QRasterWindow::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QRasterWindow_qt_metacall(self as *mut ::raster_window::RasterWindow,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QRasterWindow::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QRasterWindow_qt_metacast(self as *mut ::raster_window::RasterWindow, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QRasterWindow::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QRasterWindow_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QRasterWindow::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QRasterWindow_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::raster_window::RasterWindow {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QRasterWindow_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt slots of `RasterWindow`.
  pub struct Slots<'a>(&'a ::raster_window::RasterWindow);
  /// Represents a built-in Qt slot `QRasterWindow::update`.
  ///
  /// An object of this type can be created from `RasterWindow` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RasterWindow` object.
  pub struct Update<'a>(&'a ::raster_window::RasterWindow);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QRasterWindow::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::raster_window::RasterWindow {
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::raster_window::RasterWindow> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::raster_window::RasterWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::raster_window::RasterWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::raster_window::RasterWindow> for ::paint_device_window::PaintDeviceWindow {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::raster_window::RasterWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QPaintDeviceWindow(self as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::raster_window::RasterWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QPaintDeviceWindow(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::raster_window::RasterWindow> for ::surface::Surface {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::raster_window::RasterWindow> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QSurface(self as *mut ::surface::Surface)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::raster_window::RasterWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::raster_window::RasterWindow> for ::window::Window {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::raster_window::RasterWindow> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QWindow(self as *mut ::window::Window) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::raster_window::RasterWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_dynamic_cast_QRasterWindow_ptr_QWindow(self as *const ::window::Window as *mut ::window::Window) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::raster_window::RasterWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QObject_ptr(self as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QObject_ptr(self as *const ::raster_window::RasterWindow as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::raster_window::RasterWindow {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QPaintDevice_ptr(self as *mut ::raster_window::RasterWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QPaintDevice_ptr(self as *const ::raster_window::RasterWindow as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::paint_device_window::PaintDeviceWindow> for ::raster_window::RasterWindow {
  fn static_cast_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *mut ::raster_window::RasterWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *const ::raster_window::RasterWindow as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::surface::Surface> for ::raster_window::RasterWindow {
  fn static_cast_mut(&mut self) -> &mut ::surface::Surface {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QSurface_ptr(self as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::surface::Surface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QSurface_ptr(self as *const ::raster_window::RasterWindow as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::window::Window> for ::raster_window::RasterWindow {
  fn static_cast_mut(&mut self) -> &mut ::window::Window {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QWindow_ptr(self as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QWindow_ptr(self as *const ::raster_window::RasterWindow as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::raster_window::RasterWindow> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::raster_window::RasterWindow {
    let ffi_result =
      ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::raster_window::RasterWindow {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::raster_window::RasterWindow> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::raster_window::RasterWindow {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::raster_window::RasterWindow {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::raster_window::RasterWindow> for ::paint_device_window::PaintDeviceWindow {
  unsafe fn static_cast_mut(&mut self) -> &mut ::raster_window::RasterWindow {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QPaintDeviceWindow(self as *mut ::paint_device_window::PaintDeviceWindow);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::raster_window::RasterWindow {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QPaintDeviceWindow(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::raster_window::RasterWindow> for ::surface::Surface {
  unsafe fn static_cast_mut(&mut self) -> &mut ::raster_window::RasterWindow {
    let ffi_result =
      ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QSurface(self as *mut ::surface::Surface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::raster_window::RasterWindow {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::raster_window::RasterWindow> for ::window::Window {
  unsafe fn static_cast_mut(&mut self) -> &mut ::raster_window::RasterWindow {
    let ffi_result =
      ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QWindow(self as *mut ::window::Window);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::raster_window::RasterWindow {
    let ffi_result = ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QRasterWindow_ptr_QWindow(self as *const ::window::Window as *mut ::window::Window);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::raster_window::RasterWindow {
  type Target = ::paint_device_window::PaintDeviceWindow;
  fn deref(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *const ::raster_window::RasterWindow as *mut ::raster_window::RasterWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::raster_window::RasterWindow {
  fn deref_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRasterWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *mut ::raster_window::RasterWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

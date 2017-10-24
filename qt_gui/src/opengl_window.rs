/// C++ type: <span style='color: green;'>```QOpenGLWindow```</span>
#[repr(C)]
pub struct OpenGLWindow(u8);

impl OpenGLWindow {
  /// C++ method: <span style='color: green;'>```QOpenGLContext* QOpenGLWindow::context() const```</span>
  ///
  ///
  pub fn context(&self) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_context(self as *const ::opengl_window::OpenGLWindow) }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLWindow::defaultFramebufferObject() const```</span>
  ///
  ///
  pub fn default_framebuffer_object(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_defaultFramebufferObject(self as *const ::opengl_window::OpenGLWindow) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLWindow::doneCurrent()```</span>
  ///
  ///
  pub fn done_current(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_doneCurrent(self as *mut ::opengl_window::OpenGLWindow) }
  }

  /// C++ method: <span style='color: green;'>```QImage QOpenGLWindow::grabFramebuffer()```</span>
  ///
  ///
  pub fn grab_framebuffer(&mut self) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLWindow_grabFramebuffer_as_ptr(self as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLWindow::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_isValid(self as *const ::opengl_window::OpenGLWindow) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLWindow::makeCurrent()```</span>
  ///
  ///
  pub fn make_current(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_makeCurrent(self as *mut ::opengl_window::OpenGLWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLWindow::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_metaObject(self as *const ::opengl_window::OpenGLWindow) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLWindow::QOpenGLWindow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLWindow::QOpenGLWindow()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::opengl_window::UpdateBehavior) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLWindow::QOpenGLWindow(QOpenGLWindow::UpdateBehavior updateBehavior = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>
    where Args: overloading::OpenGLWindowNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLWindow::QOpenGLWindow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::opengl_context::OpenGLContext) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLWindow::QOpenGLWindow(QOpenGLContext* shareContext)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::opengl_context::OpenGLContext, ::opengl_window::UpdateBehavior)) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLWindow::QOpenGLWindow(QOpenGLContext* shareContext, QOpenGLWindow::UpdateBehavior updateBehavior = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::opengl_context::OpenGLContext, ::opengl_window::UpdateBehavior, *mut ::window::Window)) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLWindow::QOpenGLWindow(QOpenGLContext* shareContext, QOpenGLWindow::UpdateBehavior updateBehavior = ?, QWindow* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((::opengl_window::UpdateBehavior, *mut ::window::Window)) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLWindow::QOpenGLWindow(QOpenGLWindow::UpdateBehavior updateBehavior = ?, QWindow* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>
    where Args: overloading::OpenGLWindowNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QOpenGLWindow::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLWindow_qt_metacall(self as *mut ::opengl_window::OpenGLWindow,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLWindow::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLWindow_qt_metacast(self as *mut ::opengl_window::OpenGLWindow, arg1)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* QOpenGLWindow::shareContext() const```</span>
  ///
  ///
  pub fn share_context(&self) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_shareContext(self as *const ::opengl_window::OpenGLWindow) }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLWindow::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLWindow_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLWindow::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLWindow_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLWindow::UpdateBehavior QOpenGLWindow::updateBehavior() const```</span>
  ///
  ///
  pub fn update_behavior(&self) -> ::opengl_window::UpdateBehavior {
    unsafe { ::ffi::qt_gui_c_QOpenGLWindow_updateBehavior(self as *const ::opengl_window::OpenGLWindow) }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_window::OpenGLWindow {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLWindow_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLWindow`.
  pub struct Signals<'a>(&'a ::opengl_window::OpenGLWindow);
  /// Represents a built-in Qt signal `QOpenGLWindow::frameSwapped`.
  ///
  /// An object of this type can be created from `OpenGLWindow` with `object.signals().frame_swapped()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWindow` object.
  pub struct FrameSwapped<'a>(&'a ::opengl_window::OpenGLWindow);
  impl<'a> ::qt_core::connection::Receiver for FrameSwapped<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2frameSwapped()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FrameSwapped<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QOpenGLWindow::frameSwapped`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn frame_swapped(&self) -> FrameSwapped {
      FrameSwapped(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `OpenGLWindow`.
  pub struct Slots<'a>(&'a ::opengl_window::OpenGLWindow);
  /// Represents a built-in Qt slot `QOpenGLWindow::update`.
  ///
  /// An object of this type can be created from `OpenGLWindow` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLWindow` object.
  pub struct Update<'a>(&'a ::opengl_window::OpenGLWindow);
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
    /// Returns an object representing a built-in Qt slot `QOpenGLWindow::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::opengl_window::OpenGLWindow {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QOpenGLWindow::UpdateBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UpdateBehavior {
  /// C++ enum variant: <span style='color: green;'>```NoPartialUpdate = 0```</span>
  NoPartialUpdate = 0,
  /// C++ enum variant: <span style='color: green;'>```PartialUpdateBlit = 1```</span>
  PartialUpdateBlit = 1,
  /// C++ enum variant: <span style='color: green;'>```PartialUpdateBlend = 2```</span>
  PartialUpdateBlend = 2,
}

impl ::cpp_utils::DynamicCast<::opengl_window::OpenGLWindow> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::opengl_window::OpenGLWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::opengl_window::OpenGLWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::opengl_window::OpenGLWindow> for ::paint_device_window::PaintDeviceWindow {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::opengl_window::OpenGLWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(self as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::opengl_window::OpenGLWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::opengl_window::OpenGLWindow> for ::surface::Surface {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::opengl_window::OpenGLWindow> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QSurface(self as *mut ::surface::Surface)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::opengl_window::OpenGLWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::opengl_window::OpenGLWindow> for ::window::Window {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::opengl_window::OpenGLWindow> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QWindow(self as *mut ::window::Window) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::opengl_window::OpenGLWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QWindow(self as *const ::window::Window as *mut ::window::Window) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_window::OpenGLWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QObject_ptr(self as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QObject_ptr(self as *const ::opengl_window::OpenGLWindow as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::opengl_window::OpenGLWindow {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDevice_ptr(self as *mut ::opengl_window::OpenGLWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDevice_ptr(self as *const ::opengl_window::OpenGLWindow as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::paint_device_window::PaintDeviceWindow> for ::opengl_window::OpenGLWindow {
  fn static_cast_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *mut ::opengl_window::OpenGLWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *const ::opengl_window::OpenGLWindow as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::surface::Surface> for ::opengl_window::OpenGLWindow {
  fn static_cast_mut(&mut self) -> &mut ::surface::Surface {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QSurface_ptr(self as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::surface::Surface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QSurface_ptr(self as *const ::opengl_window::OpenGLWindow as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::window::Window> for ::opengl_window::OpenGLWindow {
  fn static_cast_mut(&mut self) -> &mut ::window::Window {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QWindow_ptr(self as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QWindow_ptr(self as *const ::opengl_window::OpenGLWindow as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_window::OpenGLWindow> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_window::OpenGLWindow {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_window::OpenGLWindow {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_window::OpenGLWindow> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_window::OpenGLWindow {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_window::OpenGLWindow {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_window::OpenGLWindow> for ::paint_device_window::PaintDeviceWindow {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_window::OpenGLWindow {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(self as *mut ::paint_device_window::PaintDeviceWindow);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_window::OpenGLWindow {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_window::OpenGLWindow> for ::surface::Surface {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_window::OpenGLWindow {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QSurface(self as *mut ::surface::Surface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_window::OpenGLWindow {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_window::OpenGLWindow> for ::window::Window {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_window::OpenGLWindow {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QWindow(self as *mut ::window::Window);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_window::OpenGLWindow {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QWindow(self as *const ::window::Window as *mut ::window::Window);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_window::OpenGLWindow {
  type Target = ::paint_device_window::PaintDeviceWindow;
  fn deref(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *const ::opengl_window::OpenGLWindow as *mut ::opengl_window::OpenGLWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_window::OpenGLWindow {
  fn deref_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDeviceWindow_ptr(self as *mut ::opengl_window::OpenGLWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLWindow::new](../struct.OpenGLWindow.html#method.new) method.
  pub trait OpenGLWindowNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>;
  }
  impl OpenGLWindowNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLWindowNewArgs for ::opengl_window::UpdateBehavior {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow> {
      let update_behavior = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLWindow_new_updateBehavior(update_behavior) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLWindow::new_unsafe](../struct.OpenGLWindow.html#method.new_unsafe) method.
  pub trait OpenGLWindowNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow>;
  }
  impl OpenGLWindowNewUnsafeArgs for *mut ::opengl_context::OpenGLContext {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow> {
      let share_context = self;
      let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_new_shareContext(share_context);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl OpenGLWindowNewUnsafeArgs for (*mut ::opengl_context::OpenGLContext, ::opengl_window::UpdateBehavior) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow> {
      let share_context = self.0;
      let update_behavior = self.1;
      let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_new_shareContext_updateBehavior(share_context, update_behavior);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl OpenGLWindowNewUnsafeArgs
    for (*mut ::opengl_context::OpenGLContext, ::opengl_window::UpdateBehavior, *mut ::window::Window) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow> {
      let share_context = self.0;
      let update_behavior = self.1;
      let parent = self.2;
      let ffi_result =
        ::ffi::qt_gui_c_QOpenGLWindow_new_shareContext_updateBehavior_parent(share_context, update_behavior, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl OpenGLWindowNewUnsafeArgs for (::opengl_window::UpdateBehavior, *mut ::window::Window) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::opengl_window::OpenGLWindow> {
      let update_behavior = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_gui_c_QOpenGLWindow_new_updateBehavior_parent(update_behavior, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

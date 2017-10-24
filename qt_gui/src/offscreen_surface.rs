/// C++ type: <span style='color: green;'>```QOffscreenSurface```</span>
#[repr(C)]
pub struct OffscreenSurface(u8);

impl OffscreenSurface {
  /// C++ method: <span style='color: green;'>```void QOffscreenSurface::create()```</span>
  ///
  ///
  pub fn create(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOffscreenSurface_create(self as *mut ::offscreen_surface::OffscreenSurface) }
  }

  /// C++ method: <span style='color: green;'>```void QOffscreenSurface::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOffscreenSurface_destroy(self as *mut ::offscreen_surface::OffscreenSurface) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSurfaceFormat QOffscreenSurface::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::surface_format::SurfaceFormat {
    {
      let mut object: ::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOffscreenSurface_format_to_output(self as *const ::offscreen_surface::OffscreenSurface,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOffscreenSurface::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOffscreenSurface_isValid(self as *const ::offscreen_surface::OffscreenSurface) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOffscreenSurface::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOffscreenSurface_metaObject(self as *const ::offscreen_surface::OffscreenSurface) }
  }

  /// C++ method: <span style='color: green;'>```void* QOffscreenSurface::nativeHandle() const```</span>
  ///
  ///
  pub fn native_handle(&self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_gui_c_QOffscreenSurface_nativeHandle(self as *const ::offscreen_surface::OffscreenSurface) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOffscreenSurface::QOffscreenSurface()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::offscreen_surface::OffscreenSurface> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOffscreenSurface_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOffscreenSurface::QOffscreenSurface(QScreen* screen = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(screen: *mut ::screen::Screen)
                           -> ::cpp_utils::CppBox<::offscreen_surface::OffscreenSurface> {
    let ffi_result = ::ffi::qt_gui_c_QOffscreenSurface_new_screen(screen);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QOffscreenSurface::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOffscreenSurface_qt_metacall(self as *mut ::offscreen_surface::OffscreenSurface,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOffscreenSurface::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOffscreenSurface_qt_metacast(self as *mut ::offscreen_surface::OffscreenSurface, arg1)
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat QOffscreenSurface::requestedFormat() const```</span>
  ///
  ///
  pub fn requested_format(&self) -> ::surface_format::SurfaceFormat {
    {
      let mut object: ::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOffscreenSurface_requestedFormat_to_output(self as *const ::offscreen_surface::OffscreenSurface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QScreen* QOffscreenSurface::screen() const```</span>
  ///
  ///
  pub fn screen(&self) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QOffscreenSurface_screen(self as *const ::offscreen_surface::OffscreenSurface) }
  }

  /// C++ method: <span style='color: green;'>```void QOffscreenSurface::setFormat(const QSurfaceFormat& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::surface_format::SurfaceFormat) {
    unsafe {
      ::ffi::qt_gui_c_QOffscreenSurface_setFormat(self as *mut ::offscreen_surface::OffscreenSurface,
                                                  format as *const ::surface_format::SurfaceFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOffscreenSurface::setNativeHandle(void* handle)```</span>
  ///
  ///
  pub unsafe fn set_native_handle(&mut self, handle: *mut ::libc::c_void) {
    ::ffi::qt_gui_c_QOffscreenSurface_setNativeHandle(self as *mut ::offscreen_surface::OffscreenSurface, handle)
  }

  /// C++ method: <span style='color: green;'>```void QOffscreenSurface::setScreen(QScreen* screen)```</span>
  ///
  ///
  pub unsafe fn set_screen(&mut self, screen: *mut ::screen::Screen) {
    ::ffi::qt_gui_c_QOffscreenSurface_setScreen(self as *mut ::offscreen_surface::OffscreenSurface, screen)
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QOffscreenSurface::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOffscreenSurface_size_to_output(self as *const ::offscreen_surface::OffscreenSurface,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSurface::SurfaceType QOffscreenSurface::surfaceType() const```</span>
  ///
  ///
  pub fn surface_type(&self) -> ::surface::SurfaceType {
    unsafe { ::ffi::qt_gui_c_QOffscreenSurface_surfaceType(self as *const ::offscreen_surface::OffscreenSurface) }
  }

  /// C++ method: <span style='color: green;'>```static QString QOffscreenSurface::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOffscreenSurface_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOffscreenSurface::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOffscreenSurface_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::offscreen_surface::OffscreenSurface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOffscreenSurface_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OffscreenSurface`.
  pub struct Signals<'a>(&'a ::offscreen_surface::OffscreenSurface);
  /// Represents a built-in Qt signal `QOffscreenSurface::objectNameChanged`.
  ///
  /// An object of this type can be created from `OffscreenSurface` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OffscreenSurface` object.
  pub struct ObjectNameChanged<'a>(&'a ::offscreen_surface::OffscreenSurface);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QOffscreenSurface::screenChanged`.
  ///
  /// An object of this type can be created from `OffscreenSurface` with `object.signals().screen_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OffscreenSurface` object.
  pub struct ScreenChanged<'a>(&'a ::offscreen_surface::OffscreenSurface);
  impl<'a> ::qt_core::connection::Receiver for ScreenChanged<'a> {
    type Arguments = (*mut ::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenChanged(QScreen*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QOffscreenSurface::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOffscreenSurface::screenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_changed(&self) -> ScreenChanged {
      ScreenChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `OffscreenSurface`.
  pub struct Slots<'a>(&'a ::offscreen_surface::OffscreenSurface);
  /// Represents a built-in Qt slot `QOffscreenSurface::screenDestroyed`.
  ///
  /// An object of this type can be created from `OffscreenSurface` with `object.slots().screen_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OffscreenSurface` object.
  pub struct ScreenDestroyed<'a>(&'a ::offscreen_surface::OffscreenSurface);
  impl<'a> ::qt_core::connection::Receiver for ScreenDestroyed<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1screenDestroyed(QObject*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QOffscreenSurface::screenDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_destroyed(&self) -> ScreenDestroyed {
      ScreenDestroyed(self.0)
    }
  }
  impl ::offscreen_surface::OffscreenSurface {
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

impl ::cpp_utils::DynamicCast<::offscreen_surface::OffscreenSurface> for ::surface::Surface {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::offscreen_surface::OffscreenSurface> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOffscreenSurface_G_dynamic_cast_QOffscreenSurface_ptr(self as *mut ::surface::Surface)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::offscreen_surface::OffscreenSurface> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOffscreenSurface_G_dynamic_cast_QOffscreenSurface_ptr(self as *const ::surface::Surface as *mut ::surface::Surface) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::offscreen_surface::OffscreenSurface {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QObject_ptr(self as *mut ::offscreen_surface::OffscreenSurface)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QObject_ptr(self as *const ::offscreen_surface::OffscreenSurface as *mut ::offscreen_surface::OffscreenSurface) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::surface::Surface> for ::offscreen_surface::OffscreenSurface {
  fn static_cast_mut(&mut self) -> &mut ::surface::Surface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QSurface_ptr(self as *mut ::offscreen_surface::OffscreenSurface)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::surface::Surface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QSurface_ptr(self as *const ::offscreen_surface::OffscreenSurface as *mut ::offscreen_surface::OffscreenSurface) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::offscreen_surface::OffscreenSurface> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::offscreen_surface::OffscreenSurface {
    let ffi_result = ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::offscreen_surface::OffscreenSurface {
    let ffi_result = ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::offscreen_surface::OffscreenSurface> for ::surface::Surface {
  unsafe fn static_cast_mut(&mut self) -> &mut ::offscreen_surface::OffscreenSurface {
    let ffi_result =
      ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QSurface(self as *mut ::surface::Surface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::offscreen_surface::OffscreenSurface {
    let ffi_result = ::ffi::qt_gui_c_QOffscreenSurface_G_static_cast_QOffscreenSurface_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

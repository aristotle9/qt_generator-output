/// C++ type: <span style='color: green;'>```QScreen```</span>
#[repr(C)]
pub struct Screen(u8);

impl Screen {
  /// C++ method: <span style='color: green;'>```int QScreen::angleBetween(Qt::ScreenOrientation a, Qt::ScreenOrientation b) const```</span>
  ///
  ///
  pub fn angle_between(&self,
                       a: &::qt_core::qt::ScreenOrientation,
                       b: &::qt_core::qt::ScreenOrientation)
                       -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QScreen_angleBetween(self as *const ::screen::Screen,
                                           a as *const ::qt_core::qt::ScreenOrientation,
                                           b as *const ::qt_core::qt::ScreenOrientation)
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QScreen::availableGeometry() const```</span>
  ///
  ///
  pub fn available_geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_availableGeometry_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QScreen::availableSize() const```</span>
  ///
  ///
  pub fn available_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_availableSize_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QScreen::availableVirtualGeometry() const```</span>
  ///
  ///
  pub fn available_virtual_geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_availableVirtualGeometry_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QScreen::availableVirtualSize() const```</span>
  ///
  ///
  pub fn available_virtual_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_availableVirtualSize_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QScreen::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QScreen_depth(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```double QScreen::devicePixelRatio() const```</span>
  ///
  ///
  pub fn device_pixel_ratio(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_devicePixelRatio(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```QRect QScreen::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_geometry_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QScreen::grabWindow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn grab_window(&mut self, ::libc::c_ulonglong) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QScreen::grabWindow(unsigned long long window)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn grab_window(&mut self, (::libc::c_ulonglong, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QScreen::grabWindow(unsigned long long window, int x = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn grab_window(&mut self, (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QScreen::grabWindow(unsigned long long window, int x = ?, int y = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn grab_window(&mut self, (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QScreen::grabWindow(unsigned long long window, int x = ?, int y = ?, int w = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn grab_window(&mut self, (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::pixmap::Pixmap>```<br>
  /// C++ method: <span style='color: green;'>```QPixmap QScreen::grabWindow(unsigned long long window, int x = ?, int y = ?, int w = ?, int h = ?)```</span>
  ///
  ///
  pub fn grab_window<'largs, Args>(&'largs mut self, args: Args) -> ::cpp_utils::CppBox<::pixmap::Pixmap>
    where Args: overloading::ScreenGrabWindowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QScreen::isLandscape(Qt::ScreenOrientation orientation) const```</span>
  ///
  ///
  pub fn is_landscape(&self, orientation: &::qt_core::qt::ScreenOrientation) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QScreen_isLandscape(self as *const ::screen::Screen,
                                          orientation as *const ::qt_core::qt::ScreenOrientation)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QScreen::isPortrait(Qt::ScreenOrientation orientation) const```</span>
  ///
  ///
  pub fn is_portrait(&self, orientation: &::qt_core::qt::ScreenOrientation) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QScreen_isPortrait(self as *const ::screen::Screen,
                                         orientation as *const ::qt_core::qt::ScreenOrientation)
    }
  }

  /// C++ method: <span style='color: green;'>```double QScreen::logicalDotsPerInch() const```</span>
  ///
  ///
  pub fn logical_dots_per_inch(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_logicalDotsPerInch(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```double QScreen::logicalDotsPerInchX() const```</span>
  ///
  ///
  pub fn logical_dots_per_inch_x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_logicalDotsPerInchX(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```double QScreen::logicalDotsPerInchY() const```</span>
  ///
  ///
  pub fn logical_dots_per_inch_y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_logicalDotsPerInchY(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```QString QScreen::manufacturer() const```</span>
  ///
  ///
  pub fn manufacturer(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_manufacturer_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QScreen::mapBetween(Qt::ScreenOrientation a, Qt::ScreenOrientation b, const QRect& rect) const```</span>
  ///
  ///
  pub fn map_between(&self,
                     a: &::qt_core::qt::ScreenOrientation,
                     b: &::qt_core::qt::ScreenOrientation,
                     rect: &::qt_core::rect::Rect)
                     -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_mapBetween_to_output(self as *const ::screen::Screen,
                                                     a as *const ::qt_core::qt::ScreenOrientation,
                                                     b as *const ::qt_core::qt::ScreenOrientation,
                                                     rect as *const ::qt_core::rect::Rect,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QScreen::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QScreen_metaObject(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```QString QScreen::model() const```</span>
  ///
  ///
  pub fn model(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_model_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QScreen::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_name_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QScreen::physicalDotsPerInch() const```</span>
  ///
  ///
  pub fn physical_dots_per_inch(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_physicalDotsPerInch(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```double QScreen::physicalDotsPerInchX() const```</span>
  ///
  ///
  pub fn physical_dots_per_inch_x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_physicalDotsPerInchX(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```double QScreen::physicalDotsPerInchY() const```</span>
  ///
  ///
  pub fn physical_dots_per_inch_y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_physicalDotsPerInchY(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QScreen::physicalSize() const```</span>
  ///
  ///
  pub fn physical_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_physicalSize_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QScreen::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QScreen_qt_metacall(self as *mut ::screen::Screen,
                                        arg1 as *const ::qt_core::meta_object::Call,
                                        arg2,
                                        arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QScreen::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QScreen_qt_metacast(self as *mut ::screen::Screen, arg1)
  }

  /// C++ method: <span style='color: green;'>```double QScreen::refreshRate() const```</span>
  ///
  ///
  pub fn refresh_rate(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QScreen_refreshRate(self as *const ::screen::Screen) }
  }

  /// C++ method: <span style='color: green;'>```QString QScreen::serialNumber() const```</span>
  ///
  ///
  pub fn serial_number(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_serialNumber_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QScreen::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_size_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QScreen::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QScreen_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QScreen::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QScreen_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QScreen::transformBetween(Qt::ScreenOrientation a, Qt::ScreenOrientation b, const QRect& target) const```</span>
  ///
  ///
  pub fn transform_between(&self,
                           a: &::qt_core::qt::ScreenOrientation,
                           b: &::qt_core::qt::ScreenOrientation,
                           target: &::qt_core::rect::Rect)
                           -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_transformBetween_to_output(self as *const ::screen::Screen,
                                                           a as *const ::qt_core::qt::ScreenOrientation,
                                                           b as *const ::qt_core::qt::ScreenOrientation,
                                                           target as *const ::qt_core::rect::Rect,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QScreen::virtualGeometry() const```</span>
  ///
  ///
  pub fn virtual_geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_virtualGeometry_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*> QScreen::virtualSiblings() const```</span>
  ///
  ///
  pub fn virtual_siblings(&self) -> ::list::ListScreenMutPtr {
    {
      let mut object: ::list::ListScreenMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_virtualSiblings_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QScreen::virtualSize() const```</span>
  ///
  ///
  pub fn virtual_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScreen_virtualSize_to_output(self as *const ::screen::Screen, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::screen::Screen {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QScreen_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Screen`.
  pub struct Signals<'a>(&'a ::screen::Screen);
  /// Represents a built-in Qt signal `QScreen::refreshRateChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().refresh_rate_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct RefreshRateChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for RefreshRateChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2refreshRateChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RefreshRateChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::availableGeometryChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().available_geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct AvailableGeometryChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for AvailableGeometryChanged<'a> {
    type Arguments = (&'static ::qt_core::rect::Rect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2availableGeometryChanged(const QRect&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AvailableGeometryChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::physicalSizeChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().physical_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct PhysicalSizeChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for PhysicalSizeChanged<'a> {
    type Arguments = (&'static ::qt_core::size_f::SizeF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2physicalSizeChanged(const QSizeF&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PhysicalSizeChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::orientationChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().orientation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct OrientationChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for OrientationChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::ScreenOrientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2orientationChanged(Qt::ScreenOrientation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OrientationChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::physicalDotsPerInchChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().physical_dots_per_inch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct PhysicalDotsPerInchChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for PhysicalDotsPerInchChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2physicalDotsPerInchChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PhysicalDotsPerInchChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::virtualGeometryChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().virtual_geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct VirtualGeometryChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for VirtualGeometryChanged<'a> {
    type Arguments = (&'static ::qt_core::rect::Rect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2virtualGeometryChanged(const QRect&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VirtualGeometryChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::logicalDotsPerInchChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().logical_dots_per_inch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct LogicalDotsPerInchChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for LogicalDotsPerInchChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2logicalDotsPerInchChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LogicalDotsPerInchChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::objectNameChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct ObjectNameChanged<'a>(&'a ::screen::Screen);
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
  /// Represents a built-in Qt signal `QScreen::primaryOrientationChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().primary_orientation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct PrimaryOrientationChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for PrimaryOrientationChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::ScreenOrientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2primaryOrientationChanged(Qt::ScreenOrientation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PrimaryOrientationChanged<'a> {}
  /// Represents a built-in Qt signal `QScreen::geometryChanged`.
  ///
  /// An object of this type can be created from `Screen` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Screen` object.
  pub struct GeometryChanged<'a>(&'a ::screen::Screen);
  impl<'a> ::qt_core::connection::Receiver for GeometryChanged<'a> {
    type Arguments = (&'static ::qt_core::rect::Rect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2geometryChanged(const QRect&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GeometryChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QScreen::refreshRateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn refresh_rate_changed(&self) -> RefreshRateChanged {
      RefreshRateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::availableGeometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn available_geometry_changed(&self) -> AvailableGeometryChanged {
      AvailableGeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::physicalSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn physical_size_changed(&self) -> PhysicalSizeChanged {
      PhysicalSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::orientationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn orientation_changed(&self) -> OrientationChanged {
      OrientationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::physicalDotsPerInchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn physical_dots_per_inch_changed(&self) -> PhysicalDotsPerInchChanged {
      PhysicalDotsPerInchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::virtualGeometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn virtual_geometry_changed(&self) -> VirtualGeometryChanged {
      VirtualGeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::logicalDotsPerInchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn logical_dots_per_inch_changed(&self) -> LogicalDotsPerInchChanged {
      LogicalDotsPerInchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::primaryOrientationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primary_orientation_changed(&self) -> PrimaryOrientationChanged {
      PrimaryOrientationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScreen::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
  }
  impl ::screen::Screen {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QScreen* arg2)```</span>
///
///
pub unsafe fn op_shl(arg1: &::qt_core::debug::Debug, arg2: *const ::screen::Screen) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_gui_c_QScreen_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug, arg2, &mut object);
    object
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::screen::Screen {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScreen_G_static_cast_QObject_ptr(self as *mut ::screen::Screen) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QScreen_G_static_cast_QObject_ptr(self as *const ::screen::Screen as *mut ::screen::Screen)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::screen::Screen> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::screen::Screen {
    let ffi_result = ::ffi::qt_gui_c_QScreen_G_static_cast_QScreen_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::screen::Screen {
    let ffi_result = ::ffi::qt_gui_c_QScreen_G_static_cast_QScreen_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::screen::Screen {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QScreen_G_static_cast_QObject_ptr(self as *const ::screen::Screen as *mut ::screen::Screen)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::screen::Screen {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScreen_G_static_cast_QObject_ptr(self as *mut ::screen::Screen) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Screen::grab_window](../struct.Screen.html#method.grab_window) method.
  pub trait ScreenGrabWindowArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::screen::Screen) -> ::cpp_utils::CppBox<::pixmap::Pixmap>;
  }
  impl<'largs> ScreenGrabWindowArgs<'largs> for ::libc::c_ulonglong {
    fn exec(self, original_self: &'largs mut ::screen::Screen) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QScreen_grabWindow_as_ptr_window(original_self as *mut ::screen::Screen, window) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ScreenGrabWindowArgs<'largs> for (::libc::c_ulonglong, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::screen::Screen) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self.0;
      let x = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QScreen_grabWindow_as_ptr_window_x(original_self as *mut ::screen::Screen, window, x)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ScreenGrabWindowArgs<'largs> for (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::screen::Screen) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self.0;
      let x = self.1;
      let y = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y(original_self as *mut ::screen::Screen, window, x, y)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ScreenGrabWindowArgs<'largs> for (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::screen::Screen) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y_w(original_self as *mut ::screen::Screen, window, x, y, w)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ScreenGrabWindowArgs<'largs>
    for (::libc::c_ulonglong, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::screen::Screen) -> ::cpp_utils::CppBox<::pixmap::Pixmap> {
      let window = self.0;
      let x = self.1;
      let y = self.2;
      let w = self.3;
      let h = self.4;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QScreen_grabWindow_as_ptr_window_x_y_w_h(original_self as *mut ::screen::Screen,
                                                                   window,
                                                                   x,
                                                                   y,
                                                                   w,
                                                                   h)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}

/// C++ type: <span style='color: green;'>```QPaintDeviceWindow```</span>
#[repr(C)]
pub struct PaintDeviceWindow(u8);

impl PaintDeviceWindow {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPaintDeviceWindow::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_metaObject(self as *const ::paint_device_window::PaintDeviceWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QPaintDeviceWindow::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QPaintDeviceWindow_qt_metacall(self as *mut ::paint_device_window::PaintDeviceWindow,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPaintDeviceWindow::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QPaintDeviceWindow_qt_metacast(self as *mut ::paint_device_window::PaintDeviceWindow, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QPaintDeviceWindow::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPaintDeviceWindow_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPaintDeviceWindow::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPaintDeviceWindow_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPaintDeviceWindow::update```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn update(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QPaintDeviceWindow::update()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn update(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPaintDeviceWindow::update(const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn update(&mut self, &::region::Region) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPaintDeviceWindow::update(const QRegion& region)```</span>
  ///
  ///
  pub fn update<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaintDeviceWindowUpdateArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::paint_device_window::PaintDeviceWindow {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPaintDeviceWindow_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PaintDeviceWindow`.
  pub struct Signals<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  /// Represents a built-in Qt signal `QPaintDeviceWindow::yChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct YChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for YChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::heightChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct HeightChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::visibleChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().visible_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct VisibleChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for VisibleChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibleChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibleChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::minimumWidthChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().minimum_width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct MinimumWidthChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for MinimumWidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2minimumWidthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MinimumWidthChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::xChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct XChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for XChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::contentOrientationChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().content_orientation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ContentOrientationChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for ContentOrientationChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::ScreenOrientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2contentOrientationChanged(Qt::ScreenOrientation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ContentOrientationChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::modalityChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().modality_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ModalityChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for ModalityChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::WindowModality,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modalityChanged(Qt::WindowModality)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ModalityChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::visibilityChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().visibility_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct VisibilityChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for VisibilityChanged<'a> {
    type Arguments = (&'static ::window::Visibility,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibilityChanged(QWindow::Visibility)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibilityChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::widthChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct WidthChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::minimumHeightChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().minimum_height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct MinimumHeightChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for MinimumHeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2minimumHeightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MinimumHeightChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::opacityChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().opacity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct OpacityChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for OpacityChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2opacityChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OpacityChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::maximumHeightChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().maximum_height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct MaximumHeightChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for MaximumHeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2maximumHeightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MaximumHeightChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::maximumWidthChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().maximum_width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct MaximumWidthChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for MaximumWidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2maximumWidthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MaximumWidthChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::windowStateChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().window_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct WindowStateChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for WindowStateChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::WindowState,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowStateChanged(Qt::WindowState)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowStateChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::windowTitleChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct WindowTitleChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for WindowTitleChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowTitleChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowTitleChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::activeChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().active_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ActiveChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for ActiveChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activeChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActiveChanged<'a> {}
  /// Represents a built-in Qt signal `QPaintDeviceWindow::screenChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().screen_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ScreenChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
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
  /// Represents a built-in Qt signal `QPaintDeviceWindow::focusObjectChanged`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.signals().focus_object_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct FocusObjectChanged<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for FocusObjectChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusObjectChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusObjectChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::yChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_changed(&self) -> YChanged {
      YChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::visibleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visible_changed(&self) -> VisibleChanged {
      VisibleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::minimumWidthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minimum_width_changed(&self) -> MinimumWidthChanged {
      MinimumWidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::xChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_changed(&self) -> XChanged {
      XChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::contentOrientationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn content_orientation_changed(&self) -> ContentOrientationChanged {
      ContentOrientationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::modalityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn modality_changed(&self) -> ModalityChanged {
      ModalityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::visibilityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visibility_changed(&self) -> VisibilityChanged {
      VisibilityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::minimumHeightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minimum_height_changed(&self) -> MinimumHeightChanged {
      MinimumHeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::opacityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_changed(&self) -> OpacityChanged {
      OpacityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::maximumHeightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_height_changed(&self) -> MaximumHeightChanged {
      MaximumHeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::maximumWidthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_width_changed(&self) -> MaximumWidthChanged {
      MaximumWidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::windowStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_state_changed(&self) -> WindowStateChanged {
      WindowStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::activeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_changed(&self) -> ActiveChanged {
      ActiveChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::screenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_changed(&self) -> ScreenChanged {
      ScreenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPaintDeviceWindow::focusObjectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_object_changed(&self) -> FocusObjectChanged {
      FocusObjectChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PaintDeviceWindow`.
  pub struct Slots<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  /// Represents a built-in Qt slot `QPaintDeviceWindow::requestUpdate`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().request_update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct RequestUpdate<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for RequestUpdate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1requestUpdate()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setMaximumHeight`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_maximum_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetMaximumHeight<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::raise`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct Raise<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setMaximumWidth`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_maximum_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetMaximumWidth<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::alert`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().alert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct Alert<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for Alert<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1alert(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::hide`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct Hide<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::lower`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct Lower<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::requestActivate`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().request_activate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct RequestActivate<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for RequestActivate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1requestActivate()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::showMaximized`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ShowMaximized<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setWidth`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetWidth<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setHeight`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetHeight<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setY`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_y()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetY<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetY<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setY(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::showFullScreen`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ShowFullScreen<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setMinimumHeight`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_minimum_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetMinimumHeight<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMinimumHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimumHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setVisible`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetVisible<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::show`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct Show<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::showNormal`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ShowNormal<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::close`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct Close<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setX`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_x()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetX<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetX<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setX(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setMinimumWidth`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_minimum_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetMinimumWidth<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetMinimumWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimumWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::update`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct Update<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::showMinimized`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct ShowMinimized<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QPaintDeviceWindow::setTitle`.
  ///
  /// An object of this type can be created from `PaintDeviceWindow` with `object.slots().set_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintDeviceWindow` object.
  pub struct SetTitle<'a>(&'a ::paint_device_window::PaintDeviceWindow);
  impl<'a> ::qt_core::connection::Receiver for SetTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTitle(const QString&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::requestUpdate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn request_update(&self) -> RequestUpdate {
      RequestUpdate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setMaximumHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_height(&self) -> SetMaximumHeight {
      SetMaximumHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setMaximumWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_width(&self) -> SetMaximumWidth {
      SetMaximumWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::alert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn alert(&self) -> Alert {
      Alert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::requestActivate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn request_activate(&self) -> RequestActivate {
      RequestActivate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setY`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y(&self) -> SetY {
      SetY(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setMinimumHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum_height(&self) -> SetMinimumHeight {
      SetMinimumHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setX`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x(&self) -> SetX {
      SetX(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setMinimumWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum_width(&self) -> SetMinimumWidth {
      SetMinimumWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPaintDeviceWindow::setTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_title(&self) -> SetTitle {
      SetTitle(self.0)
    }
  }
  impl ::paint_device_window::PaintDeviceWindow {
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

impl ::cpp_utils::DynamicCast<::paint_device_window::PaintDeviceWindow> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::paint_device_window::PaintDeviceWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::paint_device_window::PaintDeviceWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::paint_device_window::PaintDeviceWindow> for ::surface::Surface {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::paint_device_window::PaintDeviceWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QSurface(self as *mut ::surface::Surface) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::paint_device_window::PaintDeviceWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::paint_device_window::PaintDeviceWindow> for ::window::Window {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::paint_device_window::PaintDeviceWindow> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QWindow(self as *mut ::window::Window)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::paint_device_window::PaintDeviceWindow> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_dynamic_cast_QPaintDeviceWindow_ptr_QWindow(self as *const ::window::Window as *mut ::window::Window) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::paint_device_window::PaintDeviceWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QObject_ptr(self as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QObject_ptr(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::paint_device_window::PaintDeviceWindow {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDevice_ptr(self as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDevice_ptr(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::surface::Surface> for ::paint_device_window::PaintDeviceWindow {
  fn static_cast_mut(&mut self) -> &mut ::surface::Surface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QSurface_ptr(self as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::surface::Surface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QSurface_ptr(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::window::Window> for ::paint_device_window::PaintDeviceWindow {
  fn static_cast_mut(&mut self) -> &mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QWindow_ptr(self as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QWindow_ptr(self as *const ::paint_device_window::PaintDeviceWindow as *mut ::paint_device_window::PaintDeviceWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::paint_device_window::PaintDeviceWindow> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result = ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::paint_device_window::PaintDeviceWindow> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result = ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QPaintDevice(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QPaintDevice(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::paint_device_window::PaintDeviceWindow> for ::surface::Surface {
  unsafe fn static_cast_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result = ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QSurface(self as *mut ::surface::Surface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QSurface(self as *const ::surface::Surface as *mut ::surface::Surface);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::paint_device_window::PaintDeviceWindow> for ::window::Window {
  unsafe fn static_cast_mut(&mut self) -> &mut ::paint_device_window::PaintDeviceWindow {
    let ffi_result =
      ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QWindow(self as *mut ::window::Window);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::paint_device_window::PaintDeviceWindow {
    let ffi_result = ::ffi::qt_gui_c_QPaintDeviceWindow_G_static_cast_QPaintDeviceWindow_ptr_QWindow(self as *const ::window::Window as *mut ::window::Window);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PaintDeviceWindow::update](../struct.PaintDeviceWindow.html#method.update) method.
  pub trait PaintDeviceWindowUpdateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::paint_device_window::PaintDeviceWindow) -> ();
  }
  impl<'largs> PaintDeviceWindowUpdateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::paint_device_window::PaintDeviceWindow) -> () {

      unsafe { ::ffi::qt_gui_c_QPaintDeviceWindow_update_no_args(original_self as *mut ::paint_device_window::PaintDeviceWindow) }
    }
  }
  impl<'largs> PaintDeviceWindowUpdateArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::paint_device_window::PaintDeviceWindow) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QPaintDeviceWindow_update_rect(original_self as *mut ::paint_device_window::PaintDeviceWindow,
                                                       rect as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PaintDeviceWindowUpdateArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs mut ::paint_device_window::PaintDeviceWindow) -> () {
      let region = self;
      unsafe {
        ::ffi::qt_gui_c_QPaintDeviceWindow_update_region(original_self as *mut ::paint_device_window::PaintDeviceWindow, region as *const ::region::Region)
      }
    }
  }
}

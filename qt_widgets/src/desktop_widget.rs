/// C++ type: <span style='color: green;'>```QDesktopWidget```</span>
#[repr(C)]
pub struct DesktopWidget(u8);

impl DesktopWidget {
  /// C++ method: <span style='color: green;'>```QDesktopWidget::availableGeometry```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn available_geometry(&self, ()) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::availableGeometry() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn available_geometry(&self, &::qt_core::point::Point) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::availableGeometry(const QPoint& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn available_geometry(&self, ::libc::c_int) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::availableGeometry(int screen = ?) const```</span>
  ///
  ///
  pub fn available_geometry<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::DesktopWidgetAvailableGeometryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::availableGeometry(const QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn available_geometry_unsafe(&self, widget: *const ::widget::Widget) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDesktopWidget_availableGeometry_to_output_widget(self as *const ::desktop_widget::DesktopWidget, widget, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDesktopWidget::isVirtualDesktop() const```</span>
  ///
  ///
  pub fn is_virtual_desktop(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDesktopWidget_isVirtualDesktop(self as *const ::desktop_widget::DesktopWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDesktopWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDesktopWidget_metaObject(self as *const ::desktop_widget::DesktopWidget) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDesktopWidget::QDesktopWidget()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::desktop_widget::DesktopWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDesktopWidget_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QDesktopWidget::numScreens() const```</span>
  ///
  ///
  pub fn num_screens(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDesktopWidget_numScreens(self as *const ::desktop_widget::DesktopWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QDesktopWidget::primaryScreen() const```</span>
  ///
  ///
  pub fn primary_screen(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDesktopWidget_primaryScreen(self as *const ::desktop_widget::DesktopWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDesktopWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDesktopWidget_qt_metacall(self as *mut ::desktop_widget::DesktopWidget,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDesktopWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDesktopWidget_qt_metacast(self as *mut ::desktop_widget::DesktopWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```QDesktopWidget::screen```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn screen(&mut self, ()) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QDesktopWidget::screen()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn screen(&mut self, ::libc::c_int) -> *mut ::widget::Widget```<br>
  /// C++ method: <span style='color: green;'>```QWidget* QDesktopWidget::screen(int screen = ?)```</span>
  ///
  ///
  pub fn screen<'largs, Args>(&'largs mut self, args: Args) -> *mut ::widget::Widget
    where Args: overloading::DesktopWidgetScreenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QDesktopWidget::screenCount() const```</span>
  ///
  ///
  pub fn screen_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDesktopWidget_screenCount(self as *const ::desktop_widget::DesktopWidget) }
  }

  /// C++ method: <span style='color: green;'>```QDesktopWidget::screenGeometry```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn screen_geometry(&self, ()) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::screenGeometry() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn screen_geometry(&self, &::qt_core::point::Point) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::screenGeometry(const QPoint& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn screen_geometry(&self, ::libc::c_int) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::screenGeometry(int screen = ?) const```</span>
  ///
  ///
  pub fn screen_geometry<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::DesktopWidgetScreenGeometryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRect QDesktopWidget::screenGeometry(const QWidget* widget) const```</span>
  ///
  ///
  pub unsafe fn screen_geometry_unsafe(&self, widget: *const ::widget::Widget) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDesktopWidget_screenGeometry_to_output_widget(self as *const ::desktop_widget::DesktopWidget, widget, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDesktopWidget::screenNumber```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn screen_number(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QDesktopWidget::screenNumber() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn screen_number(&self, &::qt_core::point::Point) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QDesktopWidget::screenNumber(const QPoint& arg1) const```</span>
  ///
  ///
  pub fn screen_number<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::DesktopWidgetScreenNumberArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QDesktopWidget::screenNumber(const QWidget* widget = ?) const```</span>
  ///
  ///
  pub unsafe fn screen_number_unsafe(&self, widget: *const ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDesktopWidget_screenNumber_widget(self as *const ::desktop_widget::DesktopWidget, widget)
  }

  /// C++ method: <span style='color: green;'>```static QString QDesktopWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDesktopWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDesktopWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDesktopWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::desktop_widget::DesktopWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDesktopWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DesktopWidget`.
  pub struct Signals<'a>(&'a ::desktop_widget::DesktopWidget);
  /// Represents a built-in Qt signal `QDesktopWidget::primaryScreenChanged`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().primary_screen_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct PrimaryScreenChanged<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for PrimaryScreenChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2primaryScreenChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PrimaryScreenChanged<'a> {}
  /// Represents a built-in Qt signal `QDesktopWidget::screenCountChanged`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().screen_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct ScreenCountChanged<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for ScreenCountChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2screenCountChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScreenCountChanged<'a> {}
  /// Represents a built-in Qt signal `QDesktopWidget::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct WindowIconTextChanged<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for WindowIconTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconTextChanged<'a> {}
  /// Represents a built-in Qt signal `QDesktopWidget::windowTitleChanged`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct WindowTitleChanged<'a>(&'a ::desktop_widget::DesktopWidget);
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
  /// Represents a built-in Qt signal `QDesktopWidget::workAreaResized`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().work_area_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct WorkAreaResized<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for WorkAreaResized<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2workAreaResized(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorkAreaResized<'a> {}
  /// Represents a built-in Qt signal `QDesktopWidget::windowIconChanged`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct WindowIconChanged<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for WindowIconChanged<'a> {
    type Arguments = (&'static ::qt_gui::icon::Icon,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2windowIconChanged(const QIcon&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WindowIconChanged<'a> {}
  /// Represents a built-in Qt signal `QDesktopWidget::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for CustomContextMenuRequested<'a> {
    type Arguments = (&'static ::qt_core::point::Point,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2customContextMenuRequested(const QPoint&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CustomContextMenuRequested<'a> {}
  /// Represents a built-in Qt signal `QDesktopWidget::resized`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.signals().resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Resized<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Resized<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2resized(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Resized<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::primaryScreenChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primary_screen_changed(&self) -> PrimaryScreenChanged {
      PrimaryScreenChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::screenCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn screen_count_changed(&self) -> ScreenCountChanged {
      ScreenCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::workAreaResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn work_area_resized(&self) -> WorkAreaResized {
      WorkAreaResized(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDesktopWidget::resized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resized(&self) -> Resized {
      Resized(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DesktopWidget`.
  pub struct Slots<'a>(&'a ::desktop_widget::DesktopWidget);
  /// Represents a built-in Qt slot `QDesktopWidget::setEnabled`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetEnabled<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::showMaximized`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct ShowMaximized<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::updateMicroFocus`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct UpdateMicroFocus<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::showNormal`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct ShowNormal<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::setStyleSheet`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetStyleSheet<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::showMinimized`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct ShowMinimized<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::setHidden`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetHidden<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::repaint`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Repaint<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::setWindowTitle`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetWindowTitle<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::setVisible`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetVisible<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::update`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Update<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::lower`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Lower<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::hide`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Hide<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::show`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Show<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::setFocus`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetFocus<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::setDisabled`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetDisabled<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::raise`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Raise<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::close`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct Close<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::setWindowModified`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct SetWindowModified<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDesktopWidget::showFullScreen`.
  ///
  /// An object of this type can be created from `DesktopWidget` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DesktopWidget` object.
  pub struct ShowFullScreen<'a>(&'a ::desktop_widget::DesktopWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDesktopWidget::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
  }
  impl ::desktop_widget::DesktopWidget {
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

impl ::cpp_utils::DynamicCast<::desktop_widget::DesktopWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::desktop_widget::DesktopWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDesktopWidget_G_dynamic_cast_QDesktopWidget_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::desktop_widget::DesktopWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDesktopWidget_G_dynamic_cast_QDesktopWidget_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::desktop_widget::DesktopWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QObject_ptr(self as *mut ::desktop_widget::DesktopWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QObject_ptr(self as *const ::desktop_widget::DesktopWidget as *mut ::desktop_widget::DesktopWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::desktop_widget::DesktopWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::desktop_widget::DesktopWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QPaintDevice_ptr(self as *const ::desktop_widget::DesktopWidget as *mut ::desktop_widget::DesktopWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::desktop_widget::DesktopWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QWidget_ptr(self as *mut ::desktop_widget::DesktopWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QWidget_ptr(self as *const ::desktop_widget::DesktopWidget as *mut ::desktop_widget::DesktopWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::desktop_widget::DesktopWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::desktop_widget::DesktopWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::desktop_widget::DesktopWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::desktop_widget::DesktopWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::desktop_widget::DesktopWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::desktop_widget::DesktopWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::desktop_widget::DesktopWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::desktop_widget::DesktopWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::desktop_widget::DesktopWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QDesktopWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::desktop_widget::DesktopWidget {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QWidget_ptr(self as *const ::desktop_widget::DesktopWidget as *mut ::desktop_widget::DesktopWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::desktop_widget::DesktopWidget {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_G_static_cast_QWidget_ptr(self as *mut ::desktop_widget::DesktopWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DesktopWidget::available_geometry](../struct.DesktopWidget.html#method.available_geometry) method.
  pub trait DesktopWidgetAvailableGeometryArgs<'largs> {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect;
  }
  impl<'largs> DesktopWidgetAvailableGeometryArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect {

      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDesktopWidget_availableGeometry_to_output_no_args(original_self as *const ::desktop_widget::DesktopWidget, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DesktopWidgetAvailableGeometryArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect {
      let point = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDesktopWidget_availableGeometry_to_output_point(original_self as *const ::desktop_widget::DesktopWidget, point as *const ::qt_core::point::Point, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DesktopWidgetAvailableGeometryArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect {
      let screen = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDesktopWidget_availableGeometry_to_output_screen(original_self as *const ::desktop_widget::DesktopWidget, screen, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DesktopWidget::screen](../struct.DesktopWidget.html#method.screen) method.
  pub trait DesktopWidgetScreenArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::desktop_widget::DesktopWidget) -> *mut ::widget::Widget;
  }
  impl<'largs> DesktopWidgetScreenArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::desktop_widget::DesktopWidget) -> *mut ::widget::Widget {

      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_screen_no_args(original_self as *mut ::desktop_widget::DesktopWidget)
      }
    }
  }
  impl<'largs> DesktopWidgetScreenArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::desktop_widget::DesktopWidget) -> *mut ::widget::Widget {
      let screen = self;
      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_screen_screen(original_self as *mut ::desktop_widget::DesktopWidget,
                                                         screen)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DesktopWidget::screen_geometry](../struct.DesktopWidget.html#method.screen_geometry) method.
  pub trait DesktopWidgetScreenGeometryArgs<'largs> {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect;
  }
  impl<'largs> DesktopWidgetScreenGeometryArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect {

      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDesktopWidget_screenGeometry_to_output_no_args(original_self as *const ::desktop_widget::DesktopWidget, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DesktopWidgetScreenGeometryArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect {
      let point = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDesktopWidget_screenGeometry_to_output_point(original_self as *const ::desktop_widget::DesktopWidget, point as *const ::qt_core::point::Point, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DesktopWidgetScreenGeometryArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::qt_core::rect::Rect {
      let screen = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDesktopWidget_screenGeometry_to_output_screen(original_self as *const ::desktop_widget::DesktopWidget, screen, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DesktopWidget::screen_number](../struct.DesktopWidget.html#method.screen_number) method.
  pub trait DesktopWidgetScreenNumberArgs<'largs> {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::libc::c_int;
  }
  impl<'largs> DesktopWidgetScreenNumberArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_screenNumber_arg1(original_self as *const ::desktop_widget::DesktopWidget,
                                                             arg1 as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> DesktopWidgetScreenNumberArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::desktop_widget::DesktopWidget) -> ::libc::c_int {

      unsafe {
        ::ffi::qt_widgets_c_QDesktopWidget_screenNumber_no_args(original_self as *const ::desktop_widget::DesktopWidget)
      }
    }
  }
}

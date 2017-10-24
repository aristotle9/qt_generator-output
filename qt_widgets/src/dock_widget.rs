/// C++ type: <span style='color: green;'>```QDockWidget```</span>
#[repr(C)]
pub struct DockWidget(u8);

impl DockWidget {
  /// C++ method: <span style='color: green;'>```QFlags<QDockWidget::DockWidgetFeature> QDockWidget::features() const```</span>
  ///
  ///
  pub fn features(&self) -> ::qt_core::flags::Flags<::dock_widget::DockWidgetFeature> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDockWidget_features(self as *const ::dock_widget::DockWidget) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```bool QDockWidget::isAreaAllowed(Qt::DockWidgetArea area) const```</span>
  ///
  ///
  pub fn is_area_allowed(&self, area: &::qt_core::qt::DockWidgetArea) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QDockWidget_isAreaAllowed(self as *const ::dock_widget::DockWidget,
                                                    area as *const ::qt_core::qt::DockWidgetArea)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDockWidget::isFloating() const```</span>
  ///
  ///
  pub fn is_floating(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDockWidget_isFloating(self as *const ::dock_widget::DockWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDockWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDockWidget_metaObject(self as *const ::dock_widget::DockWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDockWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDockWidget_qt_metacall(self as *mut ::dock_widget::DockWidget,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDockWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDockWidget_qt_metacast(self as *mut ::dock_widget::DockWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDockWidget::setFeatures(QFlags<QDockWidget::DockWidgetFeature> features)```</span>
  ///
  ///
  pub fn set_features(&mut self, features: ::qt_core::flags::Flags<::dock_widget::DockWidgetFeature>) {
    unsafe {
      ::ffi::qt_widgets_c_QDockWidget_setFeatures(self as *mut ::dock_widget::DockWidget,
                                                  features.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDockWidget::setFloating(bool floating)```</span>
  ///
  ///
  pub fn set_floating(&mut self, floating: bool) {
    unsafe { ::ffi::qt_widgets_c_QDockWidget_setFloating(self as *mut ::dock_widget::DockWidget, floating) }
  }

  /// C++ method: <span style='color: green;'>```void QDockWidget::setTitleBarWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_title_bar_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QDockWidget_setTitleBarWidget(self as *mut ::dock_widget::DockWidget, widget)
  }

  /// C++ method: <span style='color: green;'>```void QDockWidget::setWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QDockWidget_setWidget(self as *mut ::dock_widget::DockWidget, widget)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QDockWidget::titleBarWidget() const```</span>
  ///
  ///
  pub fn title_bar_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QDockWidget_titleBarWidget(self as *const ::dock_widget::DockWidget) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QDockWidget::toggleViewAction() const```</span>
  ///
  ///
  pub fn toggle_view_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QDockWidget_toggleViewAction(self as *const ::dock_widget::DockWidget) }
  }

  /// C++ method: <span style='color: green;'>```static QString QDockWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDockWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDockWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDockWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QDockWidget::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QDockWidget_widget(self as *const ::dock_widget::DockWidget) }
  }
}

impl ::cpp_utils::CppDeletable for ::dock_widget::DockWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDockWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DockWidget`.
  pub struct Signals<'a>(&'a ::dock_widget::DockWidget);
  /// Represents a built-in Qt signal `QDockWidget::topLevelChanged`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.signals().top_level_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct TopLevelChanged<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for TopLevelChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2topLevelChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TopLevelChanged<'a> {}
  /// Represents a built-in Qt signal `QDockWidget::windowTitleChanged`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct WindowTitleChanged<'a>(&'a ::dock_widget::DockWidget);
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
  /// Represents a built-in Qt signal `QDockWidget::visibilityChanged`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.signals().visibility_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct VisibilityChanged<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for VisibilityChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2visibilityChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VisibilityChanged<'a> {}
  /// Represents a built-in Qt signal `QDockWidget::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct WindowIconTextChanged<'a>(&'a ::dock_widget::DockWidget);
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
  /// Represents a built-in Qt signal `QDockWidget::dockLocationChanged`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.signals().dock_location_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct DockLocationChanged<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for DockLocationChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::DockWidgetArea,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dockLocationChanged(Qt::DockWidgetArea)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DockLocationChanged<'a> {}
  /// Represents a built-in Qt signal `QDockWidget::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::dock_widget::DockWidget);
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
  /// Represents a built-in Qt signal `QDockWidget::windowIconChanged`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct WindowIconChanged<'a>(&'a ::dock_widget::DockWidget);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDockWidget::topLevelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_level_changed(&self) -> TopLevelChanged {
      TopLevelChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDockWidget::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDockWidget::visibilityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn visibility_changed(&self) -> VisibilityChanged {
      VisibilityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDockWidget::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDockWidget::dockLocationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn dock_location_changed(&self) -> DockLocationChanged {
      DockLocationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDockWidget::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDockWidget::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DockWidget`.
  pub struct Slots<'a>(&'a ::dock_widget::DockWidget);
  /// Represents a built-in Qt slot `QDockWidget::show`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct Show<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::showFullScreen`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct ShowFullScreen<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setEnabled`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetEnabled<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::showMinimized`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct ShowMinimized<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setStyleSheet`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetStyleSheet<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::updateMicroFocus`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct UpdateMicroFocus<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::lower`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct Lower<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setDisabled`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetDisabled<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::showMaximized`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct ShowMaximized<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setVisible`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetVisible<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::close`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct Close<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::raise`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct Raise<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::hide`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct Hide<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setWindowModified`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetWindowModified<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setHidden`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetHidden<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::update`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct Update<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::repaint`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct Repaint<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::showNormal`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct ShowNormal<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setWindowTitle`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetWindowTitle<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDockWidget::setFocus`.
  ///
  /// An object of this type can be created from `DockWidget` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DockWidget` object.
  pub struct SetFocus<'a>(&'a ::dock_widget::DockWidget);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDockWidget::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDockWidget::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
  }
  impl ::dock_widget::DockWidget {
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

/// C++ type: <span style='color: green;'>```QDockWidget::DockWidgetFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DockWidgetFeature {
  /// C++ enum variant: <span style='color: green;'>```NoDockWidgetFeatures = 0```</span>
  NoDockWidgetFeatures = 0,
  /// C++ enum variant: <span style='color: green;'>```DockWidgetClosable = 1```</span>
  DockWidgetClosable = 1,
  /// C++ enum variant: <span style='color: green;'>```DockWidgetMovable = 2```</span>
  DockWidgetMovable = 2,
  /// C++ enum variant: <span style='color: green;'>```DockWidgetFloatable = 4```</span>
  DockWidgetFloatable = 4,
  /// C++ enum variant: <span style='color: green;'>```AllDockWidgetFeatures = 7```</span>
  AllDockWidgetFeatures = 7,
  /// C++ enum variant: <span style='color: green;'>```DockWidgetVerticalTitleBar = 8```</span>
  DockWidgetVerticalTitleBar = 8,
  /// C++ enum variant: <span style='color: green;'>```DockWidgetFeatureMask = 15```</span>
  DockWidgetFeatureMask = 15,
  /// C++ enum variant: <span style='color: green;'>```Reserved = 255```</span>
  Reserved = 255,
}

impl ::qt_core::flags::FlaggableEnum for DockWidgetFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "DockWidgetFeature"
  }
}

impl ::cpp_utils::DynamicCast<::dock_widget::DockWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dock_widget::DockWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDockWidget_G_dynamic_cast_QDockWidget_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::dock_widget::DockWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDockWidget_G_dynamic_cast_QDockWidget_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::dock_widget::DockWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QObject_ptr(self as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QObject_ptr(self as *const ::dock_widget::DockWidget as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::dock_widget::DockWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QPaintDevice_ptr(self as *const ::dock_widget::DockWidget as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::dock_widget::DockWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QWidget_ptr(self as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QWidget_ptr(self as *const ::dock_widget::DockWidget as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dock_widget::DockWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dock_widget::DockWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dock_widget::DockWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dock_widget::DockWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dock_widget::DockWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dock_widget::DockWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dock_widget::DockWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dock_widget::DockWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dock_widget::DockWidget {
    let ffi_result = ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QDockWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::dock_widget::DockWidget {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QWidget_ptr(self as *const ::dock_widget::DockWidget as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dock_widget::DockWidget {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDockWidget_G_static_cast_QWidget_ptr(self as *mut ::dock_widget::DockWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

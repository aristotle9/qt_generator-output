/// C++ type: <span style='color: green;'>```QMdiSubWindow```</span>
#[repr(C)]
pub struct MdiSubWindow(u8);

impl MdiSubWindow {
  /// C++ method: <span style='color: green;'>```bool QMdiSubWindow::isShaded() const```</span>
  ///
  ///
  pub fn is_shaded(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_isShaded(self as *const ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```int QMdiSubWindow::keyboardPageStep() const```</span>
  ///
  ///
  pub fn keyboard_page_step(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_keyboardPageStep(self as *const ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```int QMdiSubWindow::keyboardSingleStep() const```</span>
  ///
  ///
  pub fn keyboard_single_step(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_keyboardSingleStep(self as *const ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMdiSubWindow::maximizedButtonsWidget() const```</span>
  ///
  ///
  pub fn maximized_buttons_widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_maximizedButtonsWidget(self as *const ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMdiSubWindow::maximizedSystemMenuIconWidget() const```</span>
  ///
  ///
  pub fn maximized_system_menu_icon_widget(&self) -> *mut ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QMdiSubWindow_maximizedSystemMenuIconWidget(self as *const ::mdi_sub_window::MdiSubWindow)
    }
  }

  /// C++ method: <span style='color: green;'>```QMdiArea* QMdiSubWindow::mdiArea() const```</span>
  ///
  ///
  pub fn mdi_area(&self) -> *mut ::mdi_area::MdiArea {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_mdiArea(self as *const ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QMdiSubWindow::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_metaObject(self as *const ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QMdiSubWindow::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_minimumSizeHint_to_output(self as *const ::mdi_sub_window::MdiSubWindow,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QMdiSubWindow::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QMdiSubWindow_qt_metacall(self as *mut ::mdi_sub_window::MdiSubWindow,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QMdiSubWindow::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QMdiSubWindow_qt_metacast(self as *mut ::mdi_sub_window::MdiSubWindow, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QMdiSubWindow::setKeyboardPageStep(int step)```</span>
  ///
  ///
  pub fn set_keyboard_page_step(&mut self, step: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_setKeyboardPageStep(self as *mut ::mdi_sub_window::MdiSubWindow, step) }
  }

  /// C++ method: <span style='color: green;'>```void QMdiSubWindow::setKeyboardSingleStep(int step)```</span>
  ///
  ///
  pub fn set_keyboard_single_step(&mut self, step: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QMdiSubWindow_setKeyboardSingleStep(self as *mut ::mdi_sub_window::MdiSubWindow, step)
    }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::mdi_sub_window::SubWindowOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMdiSubWindow::setOption(QMdiSubWindow::SubWindowOption option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::mdi_sub_window::SubWindowOption, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMdiSubWindow::setOption(QMdiSubWindow::SubWindowOption option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::MdiSubWindowSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QMdiSubWindow::setSystemMenu(QMenu* systemMenu)```</span>
  ///
  ///
  pub unsafe fn set_system_menu(&mut self, system_menu: *mut ::menu::Menu) {
    ::ffi::qt_widgets_c_QMdiSubWindow_setSystemMenu(self as *mut ::mdi_sub_window::MdiSubWindow, system_menu)
  }

  /// C++ method: <span style='color: green;'>```void QMdiSubWindow::setWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QMdiSubWindow_setWidget(self as *mut ::mdi_sub_window::MdiSubWindow, widget)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiSubWindow::showShaded()```</span>
  ///
  ///
  pub fn show_shaded(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_showShaded(self as *mut ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QMdiSubWindow::showSystemMenu()```</span>
  ///
  ///
  pub fn show_system_menu(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_showSystemMenu(self as *mut ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QMdiSubWindow::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_sizeHint_to_output(self as *const ::mdi_sub_window::MdiSubWindow,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMenu* QMdiSubWindow::systemMenu() const```</span>
  ///
  ///
  pub fn system_menu(&self) -> *mut ::menu::Menu {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_systemMenu(self as *const ::mdi_sub_window::MdiSubWindow) }
  }

  /// C++ method: <span style='color: green;'>```bool QMdiSubWindow::testOption(QMdiSubWindow::SubWindowOption arg1) const```</span>
  ///
  ///
  pub fn test_option(&self, arg1: ::mdi_sub_window::SubWindowOption) -> bool {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_testOption(self as *const ::mdi_sub_window::MdiSubWindow, arg1) }
  }

  /// C++ method: <span style='color: green;'>```static QString QMdiSubWindow::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMdiSubWindow_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QMdiSubWindow::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QMdiSubWindow_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QMdiSubWindow::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_widget(self as *const ::mdi_sub_window::MdiSubWindow) }
  }
}

impl ::cpp_utils::CppDeletable for ::mdi_sub_window::MdiSubWindow {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QMdiSubWindow_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MdiSubWindow`.
  pub struct Signals<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  /// Represents a built-in Qt signal `QMdiSubWindow::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::mdi_sub_window::MdiSubWindow);
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
  /// Represents a built-in Qt signal `QMdiSubWindow::windowIconChanged`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct WindowIconChanged<'a>(&'a ::mdi_sub_window::MdiSubWindow);
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
  /// Represents a built-in Qt signal `QMdiSubWindow::aboutToActivate`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.signals().about_to_activate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct AboutToActivate<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for AboutToActivate<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToActivate()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AboutToActivate<'a> {}
  /// Represents a built-in Qt signal `QMdiSubWindow::windowTitleChanged`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct WindowTitleChanged<'a>(&'a ::mdi_sub_window::MdiSubWindow);
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
  /// Represents a built-in Qt signal `QMdiSubWindow::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct WindowIconTextChanged<'a>(&'a ::mdi_sub_window::MdiSubWindow);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QMdiSubWindow::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMdiSubWindow::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMdiSubWindow::aboutToActivate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_activate(&self) -> AboutToActivate {
      AboutToActivate(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMdiSubWindow::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QMdiSubWindow::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MdiSubWindow`.
  pub struct Slots<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  /// Represents a built-in Qt slot `QMdiSubWindow::updateMicroFocus`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct UpdateMicroFocus<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setHidden`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetHidden<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::showSystemMenu`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().show_system_menu()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct ShowSystemMenu<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowSystemMenu<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showSystemMenu()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::repaint`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct Repaint<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setFocus`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetFocus<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::raise`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct Raise<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setEnabled`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetEnabled<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::show`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct Show<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::update`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct Update<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::hide`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct Hide<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setVisible`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetVisible<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setStyleSheet`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetStyleSheet<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setDisabled`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetDisabled<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::showShaded`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().show_shaded()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct ShowShaded<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowShaded<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showShaded()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::showFullScreen`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct ShowFullScreen<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::showNormal`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct ShowNormal<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setWindowModified`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetWindowModified<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::setWindowTitle`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct SetWindowTitle<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::showMaximized`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct ShowMaximized<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::close`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct Close<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::showMinimized`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct ShowMinimized<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QMdiSubWindow::lower`.
  ///
  /// An object of this type can be created from `MdiSubWindow` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MdiSubWindow` object.
  pub struct Lower<'a>(&'a ::mdi_sub_window::MdiSubWindow);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::showSystemMenu`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_system_menu(&self) -> ShowSystemMenu {
      ShowSystemMenu(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::showShaded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_shaded(&self) -> ShowShaded {
      ShowShaded(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QMdiSubWindow::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
  }
  impl ::mdi_sub_window::MdiSubWindow {
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

/// C++ type: <span style='color: green;'>```QMdiSubWindow::SubWindowOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SubWindowOption {
  /// C++ enum variant: <span style='color: green;'>```AllowOutsideAreaHorizontally = 1```</span>
  AllowOutsideAreaHorizontally = 1,
  /// C++ enum variant: <span style='color: green;'>```AllowOutsideAreaVertically = 2```</span>
  AllowOutsideAreaVertically = 2,
  /// C++ enum variant: <span style='color: green;'>```RubberBandResize = 4```</span>
  RubberBandResize = 4,
  /// C++ enum variant: <span style='color: green;'>```RubberBandMove = 8```</span>
  RubberBandMove = 8,
}

impl ::cpp_utils::DynamicCast<::mdi_sub_window::MdiSubWindow> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mdi_sub_window::MdiSubWindow> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_G_dynamic_cast_QMdiSubWindow_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mdi_sub_window::MdiSubWindow> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_G_dynamic_cast_QMdiSubWindow_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::mdi_sub_window::MdiSubWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QObject_ptr(self as *mut ::mdi_sub_window::MdiSubWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QObject_ptr(self as *const ::mdi_sub_window::MdiSubWindow as *mut ::mdi_sub_window::MdiSubWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::mdi_sub_window::MdiSubWindow {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QPaintDevice_ptr(self as *mut ::mdi_sub_window::MdiSubWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QPaintDevice_ptr(self as *const ::mdi_sub_window::MdiSubWindow as *mut ::mdi_sub_window::MdiSubWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::mdi_sub_window::MdiSubWindow {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QWidget_ptr(self as *mut ::mdi_sub_window::MdiSubWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QWidget_ptr(self as *const ::mdi_sub_window::MdiSubWindow as *mut ::mdi_sub_window::MdiSubWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_sub_window::MdiSubWindow> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_sub_window::MdiSubWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_sub_window::MdiSubWindow> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_sub_window::MdiSubWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mdi_sub_window::MdiSubWindow> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mdi_sub_window::MdiSubWindow {
    let ffi_result = ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QMdiSubWindow_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mdi_sub_window::MdiSubWindow {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QWidget_ptr(self as *const ::mdi_sub_window::MdiSubWindow as *mut ::mdi_sub_window::MdiSubWindow) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mdi_sub_window::MdiSubWindow {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_G_static_cast_QWidget_ptr(self as *mut ::mdi_sub_window::MdiSubWindow)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MdiSubWindow::set_option](../struct.MdiSubWindow.html#method.set_option) method.
  pub trait MdiSubWindowSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::mdi_sub_window::MdiSubWindow) -> ();
  }
  impl<'largs> MdiSubWindowSetOptionArgs<'largs> for ::mdi_sub_window::SubWindowOption {
    fn exec(self, original_self: &'largs mut ::mdi_sub_window::MdiSubWindow) -> () {
      let option = self;
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_setOption_option(original_self as *mut ::mdi_sub_window::MdiSubWindow, option)
      }
    }
  }
  impl<'largs> MdiSubWindowSetOptionArgs<'largs> for (::mdi_sub_window::SubWindowOption, bool) {
    fn exec(self, original_self: &'largs mut ::mdi_sub_window::MdiSubWindow) -> () {
      let option = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QMdiSubWindow_setOption_option_on(original_self as *mut ::mdi_sub_window::MdiSubWindow,
                                                              option,
                                                              on)
      }
    }
  }
}

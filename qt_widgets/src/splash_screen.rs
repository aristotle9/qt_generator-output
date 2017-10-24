/// C++ type: <span style='color: green;'>```QSplashScreen```</span>
#[repr(C)]
pub struct SplashScreen(u8);

impl SplashScreen {
  /// C++ method: <span style='color: green;'>```[slot] void QSplashScreen::clearMessage()```</span>
  ///
  ///
  pub fn clear_message(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QSplashScreen_clearMessage(self as *mut ::splash_screen::SplashScreen) }
  }

  /// C++ method: <span style='color: green;'>```void QSplashScreen::finish(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn finish(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QSplashScreen_finish(self as *mut ::splash_screen::SplashScreen, w)
  }

  /// C++ method: <span style='color: green;'>```QString QSplashScreen::message() const```</span>
  ///
  ///
  pub fn message(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSplashScreen_message_to_output(self as *const ::splash_screen::SplashScreen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSplashScreen::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QSplashScreen_metaObject(self as *const ::splash_screen::SplashScreen) }
  }

  /// C++ method: <span style='color: green;'>```const QPixmap QSplashScreen::pixmap() const```</span>
  ///
  ///
  pub fn pixmap(&self) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplashScreen_pixmap_as_ptr(self as *const ::splash_screen::SplashScreen) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QSplashScreen::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSplashScreen_qt_metacall(self as *mut ::splash_screen::SplashScreen,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSplashScreen::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QSplashScreen_qt_metacast(self as *mut ::splash_screen::SplashScreen, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSplashScreen::repaint()```</span>
  ///
  ///
  pub fn repaint(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QSplashScreen_repaint(self as *mut ::splash_screen::SplashScreen) }
  }

  /// C++ method: <span style='color: green;'>```void QSplashScreen::setPixmap(const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn set_pixmap(&mut self, pixmap: &::qt_gui::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_widgets_c_QSplashScreen_setPixmap(self as *mut ::splash_screen::SplashScreen,
                                                  pixmap as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```QSplashScreen::showMessage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn show_message(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSplashScreen::showMessage(const QString& message)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSplashScreen::showMessage(const QString& message, int alignment = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, ::libc::c_int, &::qt_gui::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSplashScreen::showMessage(const QString& message, int alignment = ?, const QColor& color = ?)```</span>
  ///
  ///
  pub fn show_message<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SplashScreenShowMessageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QSplashScreen::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSplashScreen_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSplashScreen::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSplashScreen_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::splash_screen::SplashScreen {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSplashScreen_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SplashScreen`.
  pub struct Signals<'a>(&'a ::splash_screen::SplashScreen);
  /// Represents a built-in Qt signal `QSplashScreen::messageChanged`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.signals().message_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct MessageChanged<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for MessageChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2messageChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MessageChanged<'a> {}
  /// Represents a built-in Qt signal `QSplashScreen::windowTitleChanged`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct WindowTitleChanged<'a>(&'a ::splash_screen::SplashScreen);
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
  /// Represents a built-in Qt signal `QSplashScreen::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::splash_screen::SplashScreen);
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
  /// Represents a built-in Qt signal `QSplashScreen::windowIconChanged`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct WindowIconChanged<'a>(&'a ::splash_screen::SplashScreen);
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
  /// Represents a built-in Qt signal `QSplashScreen::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct WindowIconTextChanged<'a>(&'a ::splash_screen::SplashScreen);
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
    /// Returns an object representing a built-in Qt signal `QSplashScreen::messageChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn message_changed(&self) -> MessageChanged {
      MessageChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSplashScreen::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSplashScreen::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSplashScreen::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSplashScreen::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SplashScreen`.
  pub struct Slots<'a>(&'a ::splash_screen::SplashScreen);
  /// Represents a built-in Qt slot `QSplashScreen::updateMicroFocus`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct UpdateMicroFocus<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setWindowModified`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetWindowModified<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::showMessage`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().show_message()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct ShowMessage<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for ShowMessage<'a> {
    type Arguments = (&'static ::qt_core::string::String, ::libc::c_int, &'static ::qt_gui::color::Color);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMessage(const QString&,int,const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setHidden`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetHidden<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::clearMessage`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().clear_message()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct ClearMessage<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for ClearMessage<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearMessage()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setEnabled`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetEnabled<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::show`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct Show<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::update`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct Update<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::showMaximized`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct ShowMaximized<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setVisible`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetVisible<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setStyleSheet`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetStyleSheet<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::showFullScreen`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct ShowFullScreen<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::raise`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct Raise<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::close`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct Close<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::lower`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct Lower<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setWindowTitle`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetWindowTitle<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::showMinimized`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct ShowMinimized<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::hide`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct Hide<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setDisabled`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetDisabled<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::setFocus`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct SetFocus<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QSplashScreen::showNormal`.
  ///
  /// An object of this type can be created from `SplashScreen` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SplashScreen` object.
  pub struct ShowNormal<'a>(&'a ::splash_screen::SplashScreen);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSplashScreen::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::showMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_message(&self) -> ShowMessage {
      ShowMessage(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::clearMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_message(&self) -> ClearMessage {
      ClearMessage(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSplashScreen::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
  }
  impl ::splash_screen::SplashScreen {
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

impl ::cpp_utils::DynamicCast<::splash_screen::SplashScreen> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::splash_screen::SplashScreen> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSplashScreen_G_dynamic_cast_QSplashScreen_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::splash_screen::SplashScreen> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplashScreen_G_dynamic_cast_QSplashScreen_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::splash_screen::SplashScreen {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QObject_ptr(self as *mut ::splash_screen::SplashScreen)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QObject_ptr(self as *const ::splash_screen::SplashScreen as *mut ::splash_screen::SplashScreen) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::splash_screen::SplashScreen {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QPaintDevice_ptr(self as *mut ::splash_screen::SplashScreen)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QPaintDevice_ptr(self as *const ::splash_screen::SplashScreen as *mut ::splash_screen::SplashScreen) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::splash_screen::SplashScreen {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QWidget_ptr(self as *mut ::splash_screen::SplashScreen)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QWidget_ptr(self as *const ::splash_screen::SplashScreen as *mut ::splash_screen::SplashScreen) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splash_screen::SplashScreen> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splash_screen::SplashScreen {
    let ffi_result = ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splash_screen::SplashScreen {
    let ffi_result = ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splash_screen::SplashScreen> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splash_screen::SplashScreen {
    let ffi_result = ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splash_screen::SplashScreen {
    let ffi_result = ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::splash_screen::SplashScreen> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::splash_screen::SplashScreen {
    let ffi_result =
      ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::splash_screen::SplashScreen {
    let ffi_result = ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QSplashScreen_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::splash_screen::SplashScreen {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QWidget_ptr(self as *const ::splash_screen::SplashScreen as *mut ::splash_screen::SplashScreen) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::splash_screen::SplashScreen {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSplashScreen_G_static_cast_QWidget_ptr(self as *mut ::splash_screen::SplashScreen)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SplashScreen::show_message](../struct.SplashScreen.html#method.show_message) method.
  pub trait SplashScreenShowMessageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::splash_screen::SplashScreen) -> ();
  }
  impl<'largs> SplashScreenShowMessageArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::splash_screen::SplashScreen) -> () {
      let message = self;
      unsafe {
        ::ffi::qt_widgets_c_QSplashScreen_showMessage_message(original_self as *mut ::splash_screen::SplashScreen,
                                                              message as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> SplashScreenShowMessageArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::splash_screen::SplashScreen) -> () {
      let message = self.0;
      let alignment = self.1;
      unsafe { ::ffi::qt_widgets_c_QSplashScreen_showMessage_message_alignment(original_self as *mut ::splash_screen::SplashScreen, message as *const ::qt_core::string::String, alignment) }
    }
  }
  impl<'largs> SplashScreenShowMessageArgs<'largs>
    for (&'largs ::qt_core::string::String, ::libc::c_int, &'largs ::qt_gui::color::Color) {
    fn exec(self, original_self: &'largs mut ::splash_screen::SplashScreen) -> () {
      let message = self.0;
      let alignment = self.1;
      let color = self.2;
      unsafe { ::ffi::qt_widgets_c_QSplashScreen_showMessage_message_alignment_color(original_self as *mut ::splash_screen::SplashScreen, message as *const ::qt_core::string::String, alignment, color as *const ::qt_gui::color::Color) }
    }
  }
}

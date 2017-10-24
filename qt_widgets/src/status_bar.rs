/// C++ type: <span style='color: green;'>```QStatusBar```</span>
#[repr(C)]
pub struct StatusBar(u8);

impl StatusBar {
  /// C++ method: <span style='color: green;'>```QStatusBar::addPermanentWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_permanent_widget(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStatusBar::addPermanentWidget(QWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_permanent_widget(&mut self, (*mut ::widget::Widget, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStatusBar::addPermanentWidget(QWidget* widget, int stretch = ?)```</span>
  ///
  ///
  pub unsafe fn add_permanent_widget<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StatusBarAddPermanentWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStatusBar::addWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_widget(&mut self, *mut ::widget::Widget) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStatusBar::addWidget(QWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_widget(&mut self, (*mut ::widget::Widget, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStatusBar::addWidget(QWidget* widget, int stretch = ?)```</span>
  ///
  ///
  pub unsafe fn add_widget<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StatusBarAddWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QStatusBar::clearMessage()```</span>
  ///
  ///
  pub fn clear_message(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QStatusBar_clearMessage(self as *mut ::status_bar::StatusBar) }
  }

  /// C++ method: <span style='color: green;'>```QString QStatusBar::currentMessage() const```</span>
  ///
  ///
  pub fn current_message(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStatusBar_currentMessage_to_output(self as *const ::status_bar::StatusBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStatusBar::insertPermanentWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_permanent_widget(&mut self, (::libc::c_int, *mut ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStatusBar::insertPermanentWidget(int index, QWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_permanent_widget(&mut self, (::libc::c_int, *mut ::widget::Widget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStatusBar::insertPermanentWidget(int index, QWidget* widget, int stretch = ?)```</span>
  ///
  ///
  pub unsafe fn insert_permanent_widget<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::StatusBarInsertPermanentWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStatusBar::insertWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_widget(&mut self, (::libc::c_int, *mut ::widget::Widget)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStatusBar::insertWidget(int index, QWidget* widget)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_widget(&mut self, (::libc::c_int, *mut ::widget::Widget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStatusBar::insertWidget(int index, QWidget* widget, int stretch = ?)```</span>
  ///
  ///
  pub unsafe fn insert_widget<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::StatusBarInsertWidgetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QStatusBar::isSizeGripEnabled() const```</span>
  ///
  ///
  pub fn is_size_grip_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStatusBar_isSizeGripEnabled(self as *const ::status_bar::StatusBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStatusBar::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QStatusBar_metaObject(self as *const ::status_bar::StatusBar) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStatusBar::QStatusBar()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::status_bar::StatusBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStatusBar_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QStatusBar::QStatusBar(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::status_bar::StatusBar> {
    let ffi_result = ::ffi::qt_widgets_c_QStatusBar_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QStatusBar::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QStatusBar_qt_metacall(self as *mut ::status_bar::StatusBar,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QStatusBar::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QStatusBar_qt_metacast(self as *mut ::status_bar::StatusBar, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QStatusBar::removeWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn remove_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QStatusBar_removeWidget(self as *mut ::status_bar::StatusBar, widget)
  }

  /// C++ method: <span style='color: green;'>```void QStatusBar::setSizeGripEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_size_grip_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QStatusBar_setSizeGripEnabled(self as *mut ::status_bar::StatusBar, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QStatusBar::showMessage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn show_message(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QStatusBar::showMessage(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QStatusBar::showMessage(const QString& text, int timeout = ?)```</span>
  ///
  ///
  pub fn show_message<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StatusBarShowMessageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QStatusBar::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStatusBar_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStatusBar::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QStatusBar_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::status_bar::StatusBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStatusBar_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StatusBar`.
  pub struct Signals<'a>(&'a ::status_bar::StatusBar);
  /// Represents a built-in Qt signal `QStatusBar::windowIconChanged`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct WindowIconChanged<'a>(&'a ::status_bar::StatusBar);
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
  /// Represents a built-in Qt signal `QStatusBar::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::status_bar::StatusBar);
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
  /// Represents a built-in Qt signal `QStatusBar::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct WindowIconTextChanged<'a>(&'a ::status_bar::StatusBar);
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
  /// Represents a built-in Qt signal `QStatusBar::messageChanged`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.signals().message_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct MessageChanged<'a>(&'a ::status_bar::StatusBar);
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
  /// Represents a built-in Qt signal `QStatusBar::windowTitleChanged`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct WindowTitleChanged<'a>(&'a ::status_bar::StatusBar);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QStatusBar::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStatusBar::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStatusBar::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStatusBar::messageChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn message_changed(&self) -> MessageChanged {
      MessageChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStatusBar::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StatusBar`.
  pub struct Slots<'a>(&'a ::status_bar::StatusBar);
  /// Represents a built-in Qt slot `QStatusBar::updateMicroFocus`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct UpdateMicroFocus<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setEnabled`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetEnabled<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setDisabled`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetDisabled<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::update`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct Update<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::repaint`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct Repaint<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::raise`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct Raise<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setWindowTitle`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetWindowTitle<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::clearMessage`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().clear_message()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct ClearMessage<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for ClearMessage<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearMessage()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::showMessage`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().show_message()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct ShowMessage<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMessage<'a> {
    type Arguments = (&'static ::qt_core::string::String, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMessage(const QString&,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::lower`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct Lower<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setHidden`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetHidden<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::showNormal`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct ShowNormal<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setWindowModified`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetWindowModified<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setFocus`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetFocus<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::close`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct Close<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::showMinimized`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct ShowMinimized<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::showMaximized`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct ShowMaximized<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setVisible`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetVisible<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::showFullScreen`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct ShowFullScreen<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::show`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct Show<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::hide`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct Hide<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QStatusBar::setStyleSheet`.
  ///
  /// An object of this type can be created from `StatusBar` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StatusBar` object.
  pub struct SetStyleSheet<'a>(&'a ::status_bar::StatusBar);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QStatusBar::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::clearMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_message(&self) -> ClearMessage {
      ClearMessage(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::showMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_message(&self) -> ShowMessage {
      ShowMessage(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStatusBar::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
  }
  impl ::status_bar::StatusBar {
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

impl ::cpp_utils::DynamicCast<::status_bar::StatusBar> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::status_bar::StatusBar> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStatusBar_G_dynamic_cast_QStatusBar_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::status_bar::StatusBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStatusBar_G_dynamic_cast_QStatusBar_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::status_bar::StatusBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QObject_ptr(self as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QObject_ptr(self as *const ::status_bar::StatusBar as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::status_bar::StatusBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QPaintDevice_ptr(self as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QPaintDevice_ptr(self as *const ::status_bar::StatusBar as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::status_bar::StatusBar {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QWidget_ptr(self as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QWidget_ptr(self as *const ::status_bar::StatusBar as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::status_bar::StatusBar> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::status_bar::StatusBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::status_bar::StatusBar {
    let ffi_result = ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::status_bar::StatusBar> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::status_bar::StatusBar {
    let ffi_result = ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::status_bar::StatusBar {
    let ffi_result = ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::status_bar::StatusBar> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::status_bar::StatusBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::status_bar::StatusBar {
    let ffi_result = ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QStatusBar_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::status_bar::StatusBar {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QWidget_ptr(self as *const ::status_bar::StatusBar as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::status_bar::StatusBar {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStatusBar_G_static_cast_QWidget_ptr(self as *mut ::status_bar::StatusBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StatusBar::add_permanent_widget](../struct.StatusBar.html#method.add_permanent_widget) method.
  pub trait StatusBarAddPermanentWidgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ();
  }
  impl<'largs> StatusBarAddPermanentWidgetArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QStatusBar_addPermanentWidget_widget(original_self as *mut ::status_bar::StatusBar, widget)
    }
  }
  impl<'largs> StatusBarAddPermanentWidgetArgs<'largs> for (*mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> () {
      let widget = self.0;
      let stretch = self.1;
      ::ffi::qt_widgets_c_QStatusBar_addPermanentWidget_widget_stretch(original_self as *mut ::status_bar::StatusBar,
                                                                       widget,
                                                                       stretch)
    }
  }
  /// This trait represents a set of arguments accepted by [StatusBar::add_widget](../struct.StatusBar.html#method.add_widget) method.
  pub trait StatusBarAddWidgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ();
  }
  impl<'largs> StatusBarAddWidgetArgs<'largs> for *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> () {
      let widget = self;
      ::ffi::qt_widgets_c_QStatusBar_addWidget_widget(original_self as *mut ::status_bar::StatusBar, widget)
    }
  }
  impl<'largs> StatusBarAddWidgetArgs<'largs> for (*mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> () {
      let widget = self.0;
      let stretch = self.1;
      ::ffi::qt_widgets_c_QStatusBar_addWidget_widget_stretch(original_self as *mut ::status_bar::StatusBar,
                                                              widget,
                                                              stretch)
    }
  }
  /// This trait represents a set of arguments accepted by [StatusBar::insert_permanent_widget](../struct.StatusBar.html#method.insert_permanent_widget) method.
  pub trait StatusBarInsertPermanentWidgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ::libc::c_int;
  }
  impl<'largs> StatusBarInsertPermanentWidgetArgs<'largs> for (::libc::c_int, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      ::ffi::qt_widgets_c_QStatusBar_insertPermanentWidget_index_widget(original_self as *mut ::status_bar::StatusBar,
                                                                        index,
                                                                        widget)
    }
  }
  impl<'largs> StatusBarInsertPermanentWidgetArgs<'largs> for (::libc::c_int, *mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      let stretch = self.2;
      ::ffi::qt_widgets_c_QStatusBar_insertPermanentWidget_index_widget_stretch(original_self as *mut ::status_bar::StatusBar, index, widget, stretch)
    }
  }
  /// This trait represents a set of arguments accepted by [StatusBar::insert_widget](../struct.StatusBar.html#method.insert_widget) method.
  pub trait StatusBarInsertWidgetArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ::libc::c_int;
  }
  impl<'largs> StatusBarInsertWidgetArgs<'largs> for (::libc::c_int, *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      ::ffi::qt_widgets_c_QStatusBar_insertWidget_index_widget(original_self as *mut ::status_bar::StatusBar,
                                                               index,
                                                               widget)
    }
  }
  impl<'largs> StatusBarInsertWidgetArgs<'largs> for (::libc::c_int, *mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ::libc::c_int {
      let index = self.0;
      let widget = self.1;
      let stretch = self.2;
      ::ffi::qt_widgets_c_QStatusBar_insertWidget_index_widget_stretch(original_self as *mut ::status_bar::StatusBar,
                                                                       index,
                                                                       widget,
                                                                       stretch)
    }
  }
  /// This trait represents a set of arguments accepted by [StatusBar::show_message](../struct.StatusBar.html#method.show_message) method.
  pub trait StatusBarShowMessageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> ();
  }
  impl<'largs> StatusBarShowMessageArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> () {
      let text = self;
      unsafe {
        ::ffi::qt_widgets_c_QStatusBar_showMessage_text(original_self as *mut ::status_bar::StatusBar,
                                                        text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> StatusBarShowMessageArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::status_bar::StatusBar) -> () {
      let text = self.0;
      let timeout = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QStatusBar_showMessage_text_timeout(original_self as *mut ::status_bar::StatusBar,
                                                                text as *const ::qt_core::string::String,
                                                                timeout)
      }
    }
  }
}

/// C++ type: <span style='color: green;'>```QProgressBar::Direction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Direction {
  /// C++ enum variant: <span style='color: green;'>```TopToBottom = 0```</span>
  TopToBottom = 0,
  /// C++ enum variant: <span style='color: green;'>```BottomToTop = 1```</span>
  BottomToTop = 1,
}

/// C++ type: <span style='color: green;'>```QProgressBar```</span>
#[repr(C)]
pub struct ProgressBar(u8);

impl ProgressBar {
  /// C++ method: <span style='color: green;'>```QString QProgressBar::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProgressBar_format_to_output(self as *const ::progress_bar::ProgressBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QProgressBar::invertedAppearance() const```</span>
  ///
  ///
  pub fn inverted_appearance(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_invertedAppearance(self as *const ::progress_bar::ProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QProgressBar::isTextVisible() const```</span>
  ///
  ///
  pub fn is_text_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_isTextVisible(self as *const ::progress_bar::ProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```int QProgressBar::maximum() const```</span>
  ///
  ///
  pub fn maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_maximum(self as *const ::progress_bar::ProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QProgressBar::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_metaObject(self as *const ::progress_bar::ProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```int QProgressBar::minimum() const```</span>
  ///
  ///
  pub fn minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_minimum(self as *const ::progress_bar::ProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QProgressBar::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProgressBar_minimumSizeHint_to_output(self as *const ::progress_bar::ProgressBar,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QProgressBar::QProgressBar()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::progress_bar::ProgressBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressBar_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QProgressBar::QProgressBar(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::progress_bar::ProgressBar> {
    let ffi_result = ::ffi::qt_widgets_c_QProgressBar_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QProgressBar::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QProgressBar_qt_metacall(self as *mut ::progress_bar::ProgressBar,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QProgressBar::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QProgressBar_qt_metacast(self as *mut ::progress_bar::ProgressBar, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressBar::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_reset(self as *mut ::progress_bar::ProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```void QProgressBar::resetFormat()```</span>
  ///
  ///
  pub fn reset_format(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_resetFormat(self as *mut ::progress_bar::ProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```void QProgressBar::setFormat(const QString& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QProgressBar_setFormat(self as *mut ::progress_bar::ProgressBar,
                                                 format as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProgressBar::setInvertedAppearance(bool invert)```</span>
  ///
  ///
  pub fn set_inverted_appearance(&mut self, invert: bool) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_setInvertedAppearance(self as *mut ::progress_bar::ProgressBar, invert) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressBar::setMaximum(int maximum)```</span>
  ///
  ///
  pub fn set_maximum(&mut self, maximum: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_setMaximum(self as *mut ::progress_bar::ProgressBar, maximum) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressBar::setMinimum(int minimum)```</span>
  ///
  ///
  pub fn set_minimum(&mut self, minimum: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_setMinimum(self as *mut ::progress_bar::ProgressBar, minimum) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressBar::setOrientation(Qt::Orientation arg1)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, arg1: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QProgressBar_setOrientation(self as *mut ::progress_bar::ProgressBar,
                                                      arg1 as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressBar::setRange(int minimum, int maximum)```</span>
  ///
  ///
  pub fn set_range(&mut self, minimum: ::libc::c_int, maximum: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_setRange(self as *mut ::progress_bar::ProgressBar, minimum, maximum) }
  }

  /// C++ method: <span style='color: green;'>```void QProgressBar::setTextDirection(QProgressBar::Direction textDirection)```</span>
  ///
  ///
  pub fn set_text_direction(&mut self, text_direction: &::progress_bar::Direction) {
    unsafe {
      ::ffi::qt_widgets_c_QProgressBar_setTextDirection(self as *mut ::progress_bar::ProgressBar,
                                                        text_direction as *const ::progress_bar::Direction)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProgressBar::setTextVisible(bool visible)```</span>
  ///
  ///
  pub fn set_text_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_setTextVisible(self as *mut ::progress_bar::ProgressBar, visible) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressBar::setValue(int value)```</span>
  ///
  ///
  pub fn set_value(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_setValue(self as *mut ::progress_bar::ProgressBar, value) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QProgressBar::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProgressBar_sizeHint_to_output(self as *const ::progress_bar::ProgressBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QProgressBar::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProgressBar_text_to_output(self as *const ::progress_bar::ProgressBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QProgressBar::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProgressBar_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QProgressBar::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProgressBar_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QProgressBar::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProgressBar_value(self as *const ::progress_bar::ProgressBar) }
  }
}

impl ::cpp_utils::CppDeletable for ::progress_bar::ProgressBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QProgressBar_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ProgressBar`.
  pub struct Signals<'a>(&'a ::progress_bar::ProgressBar);
  /// Represents a built-in Qt signal `QProgressBar::windowIconChanged`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct WindowIconChanged<'a>(&'a ::progress_bar::ProgressBar);
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
  /// Represents a built-in Qt signal `QProgressBar::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct WindowIconTextChanged<'a>(&'a ::progress_bar::ProgressBar);
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
  /// Represents a built-in Qt signal `QProgressBar::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::progress_bar::ProgressBar);
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
  /// Represents a built-in Qt signal `QProgressBar::windowTitleChanged`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct WindowTitleChanged<'a>(&'a ::progress_bar::ProgressBar);
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
  /// Represents a built-in Qt signal `QProgressBar::valueChanged`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct ValueChanged<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for ValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QProgressBar::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProgressBar::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProgressBar::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProgressBar::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProgressBar::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ProgressBar`.
  pub struct Slots<'a>(&'a ::progress_bar::ProgressBar);
  /// Represents a built-in Qt slot `QProgressBar::setFocus`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetFocus<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::raise`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Raise<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setWindowTitle`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetWindowTitle<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setStyleSheet`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetStyleSheet<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setValue`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetValue<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::repaint`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Repaint<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setMaximum`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_maximum()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetMaximum<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetMaximum<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximum(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::updateMicroFocus`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct UpdateMicroFocus<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setDisabled`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetDisabled<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setHidden`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetHidden<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::showNormal`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct ShowNormal<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::reset`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Reset<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setOrientation`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_orientation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetOrientation<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetOrientation<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOrientation(Qt::Orientation)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::hide`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Hide<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::showMaximized`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct ShowMaximized<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setVisible`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetVisible<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::show`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Show<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::update`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Update<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::close`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Close<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::showMinimized`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct ShowMinimized<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setEnabled`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetEnabled<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::showFullScreen`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct ShowFullScreen<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setRange`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_range()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetRange<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetRange<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRange(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::lower`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct Lower<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setMinimum`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_minimum()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetMinimum<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetMinimum<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimum(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressBar::setWindowModified`.
  ///
  /// An object of this type can be created from `ProgressBar` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressBar` object.
  pub struct SetWindowModified<'a>(&'a ::progress_bar::ProgressBar);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QProgressBar::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setMaximum`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum(&self) -> SetMaximum {
      SetMaximum(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setOrientation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_orientation(&self) -> SetOrientation {
      SetOrientation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setRange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_range(&self) -> SetRange {
      SetRange(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setMinimum`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum(&self) -> SetMinimum {
      SetMinimum(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressBar::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
  }
  impl ::progress_bar::ProgressBar {
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

impl ::cpp_utils::DynamicCast<::progress_bar::ProgressBar> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::progress_bar::ProgressBar> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProgressBar_G_dynamic_cast_QProgressBar_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::progress_bar::ProgressBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressBar_G_dynamic_cast_QProgressBar_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::progress_bar::ProgressBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QObject_ptr(self as *mut ::progress_bar::ProgressBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QObject_ptr(self as *const ::progress_bar::ProgressBar as *mut ::progress_bar::ProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::progress_bar::ProgressBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QPaintDevice_ptr(self as *mut ::progress_bar::ProgressBar)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QPaintDevice_ptr(self as *const ::progress_bar::ProgressBar as *mut ::progress_bar::ProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::progress_bar::ProgressBar {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QWidget_ptr(self as *mut ::progress_bar::ProgressBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QWidget_ptr(self as *const ::progress_bar::ProgressBar as *mut ::progress_bar::ProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::progress_bar::ProgressBar> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::progress_bar::ProgressBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::progress_bar::ProgressBar {
    let ffi_result = ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::progress_bar::ProgressBar> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::progress_bar::ProgressBar {
    let ffi_result = ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::progress_bar::ProgressBar {
    let ffi_result = ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::progress_bar::ProgressBar> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::progress_bar::ProgressBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::progress_bar::ProgressBar {
    let ffi_result = ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QProgressBar_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::progress_bar::ProgressBar {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QWidget_ptr(self as *const ::progress_bar::ProgressBar as *mut ::progress_bar::ProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::progress_bar::ProgressBar {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QProgressBar_G_static_cast_QWidget_ptr(self as *mut ::progress_bar::ProgressBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

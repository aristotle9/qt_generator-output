/// C++ type: <span style='color: green;'>```QDialog```</span>
#[repr(C)]
pub struct Dialog(u8);

impl Dialog {
  /// C++ method: <span style='color: green;'>```virtual [slot] void QDialog::accept()```</span>
  ///
  ///
  pub fn accept(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDialog_accept(self as *mut ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QDialog::done(int arg1)```</span>
  ///
  ///
  pub fn done(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QDialog_done(self as *mut ::dialog::Dialog, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] int QDialog::exec()```</span>
  ///
  ///
  pub fn exec(&mut self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDialog_exec(self as *mut ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QDialog::extension() const```</span>
  ///
  ///
  pub fn extension(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QDialog_extension(self as *const ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QDialog::isSizeGripEnabled() const```</span>
  ///
  ///
  pub fn is_size_grip_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDialog_isSizeGripEnabled(self as *const ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDialog::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDialog_metaObject(self as *const ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QDialog::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDialog_minimumSizeHint_to_output(self as *const ::dialog::Dialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QDialog::open()```</span>
  ///
  ///
  pub fn open(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDialog_open(self as *mut ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDialog::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDialog_qt_metacall(self as *mut ::dialog::Dialog,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDialog::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDialog_qt_metacast(self as *mut ::dialog::Dialog, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] void QDialog::reject()```</span>
  ///
  ///
  pub fn reject(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDialog_reject(self as *mut ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```int QDialog::result() const```</span>
  ///
  ///
  pub fn result(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDialog_result(self as *const ::dialog::Dialog) }
  }

  /// C++ method: <span style='color: green;'>```void QDialog::setExtension(QWidget* extension)```</span>
  ///
  ///
  pub unsafe fn set_extension(&mut self, extension: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QDialog_setExtension(self as *mut ::dialog::Dialog, extension)
  }

  /// C++ method: <span style='color: green;'>```void QDialog::setModal(bool modal)```</span>
  ///
  ///
  pub fn set_modal(&mut self, modal: bool) {
    unsafe { ::ffi::qt_widgets_c_QDialog_setModal(self as *mut ::dialog::Dialog, modal) }
  }

  /// C++ method: <span style='color: green;'>```void QDialog::setOrientation(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, orientation: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QDialog_setOrientation(self as *mut ::dialog::Dialog,
                                                 orientation as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDialog::setResult(int r)```</span>
  ///
  ///
  pub fn set_result(&mut self, r: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QDialog_setResult(self as *mut ::dialog::Dialog, r) }
  }

  /// C++ method: <span style='color: green;'>```void QDialog::setSizeGripEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_size_grip_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QDialog_setSizeGripEnabled(self as *mut ::dialog::Dialog, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QDialog::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QDialog_setVisible(self as *mut ::dialog::Dialog, visible) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDialog::showExtension(bool arg1)```</span>
  ///
  ///
  pub fn show_extension(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QDialog_showExtension(self as *mut ::dialog::Dialog, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QDialog::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDialog_sizeHint_to_output(self as *const ::dialog::Dialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDialog::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDialog_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDialog::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDialog_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::dialog::Dialog {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDialog_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Dialog`.
  pub struct Signals<'a>(&'a ::dialog::Dialog);
  /// Represents a built-in Qt signal `QDialog::windowIconChanged`.
  ///
  /// An object of this type can be created from `Dialog` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct WindowIconChanged<'a>(&'a ::dialog::Dialog);
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
  /// Represents a built-in Qt signal `QDialog::accepted`.
  ///
  /// An object of this type can be created from `Dialog` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Accepted<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Accepted<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2accepted()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Accepted<'a> {}
  /// Represents a built-in Qt signal `QDialog::rejected`.
  ///
  /// An object of this type can be created from `Dialog` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Rejected<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Rejected<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rejected()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Rejected<'a> {}
  /// Represents a built-in Qt signal `QDialog::windowTitleChanged`.
  ///
  /// An object of this type can be created from `Dialog` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct WindowTitleChanged<'a>(&'a ::dialog::Dialog);
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
  /// Represents a built-in Qt signal `QDialog::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `Dialog` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct WindowIconTextChanged<'a>(&'a ::dialog::Dialog);
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
  /// Represents a built-in Qt signal `QDialog::finished`.
  ///
  /// An object of this type can be created from `Dialog` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Finished<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Finished<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Finished<'a> {}
  /// Represents a built-in Qt signal `QDialog::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `Dialog` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::dialog::Dialog);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDialog::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialog::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialog::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialog::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialog::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialog::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialog::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Dialog`.
  pub struct Slots<'a>(&'a ::dialog::Dialog);
  /// Represents a built-in Qt slot `QDialog::done`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().done()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Done<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Done<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1done(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::setEnabled`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct SetEnabled<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::showNormal`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct ShowNormal<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::raise`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Raise<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::setWindowTitle`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct SetWindowTitle<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::showFullScreen`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct ShowFullScreen<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::lower`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Lower<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::close`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Close<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::setStyleSheet`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct SetStyleSheet<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::showMaximized`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct ShowMaximized<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::updateMicroFocus`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct UpdateMicroFocus<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::reject`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Reject<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::update`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Update<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::setFocus`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct SetFocus<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::show`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Show<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::showMinimized`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct ShowMinimized<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::repaint`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Repaint<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::accept`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Accept<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::exec`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Exec<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::open`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Open<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::showExtension`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct ShowExtension<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::setDisabled`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct SetDisabled<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::setHidden`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct SetHidden<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::setWindowModified`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct SetWindowModified<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialog::hide`.
  ///
  /// An object of this type can be created from `Dialog` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dialog` object.
  pub struct Hide<'a>(&'a ::dialog::Dialog);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDialog::done`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn done(&self) -> Done {
      Done(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialog::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
  }
  impl ::dialog::Dialog {
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

/// C++ type: <span style='color: green;'>```QDialog::DialogCode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DialogCode {
  /// C++ enum variant: <span style='color: green;'>```Rejected = 0```</span>
  Rejected = 0,
  /// C++ enum variant: <span style='color: green;'>```Accepted = 1```</span>
  Accepted = 1,
}

impl ::cpp_utils::DynamicCast<::dialog::Dialog> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dialog::Dialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialog_G_dynamic_cast_QDialog_ptr(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::dialog::Dialog> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDialog_G_dynamic_cast_QDialog_ptr(self as *const ::widget::Widget as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::dialog::Dialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialog_G_static_cast_QObject_ptr(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDialog_G_static_cast_QObject_ptr(self as *const ::dialog::Dialog as *mut ::dialog::Dialog)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::dialog::Dialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDialog_G_static_cast_QPaintDevice_ptr(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialog_G_static_cast_QPaintDevice_ptr(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::dialog::Dialog {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialog_G_static_cast_QWidget_ptr(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDialog_G_static_cast_QWidget_ptr(self as *const ::dialog::Dialog as *mut ::dialog::Dialog)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dialog::Dialog> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = ::ffi::qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dialog::Dialog> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result = ::ffi::qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = ::ffi::qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dialog::Dialog> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result = ::ffi::qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = ::ffi::qt_widgets_c_QDialog_G_static_cast_QDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::dialog::Dialog {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDialog_G_static_cast_QWidget_ptr(self as *const ::dialog::Dialog as *mut ::dialog::Dialog)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dialog::Dialog {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialog_G_static_cast_QWidget_ptr(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

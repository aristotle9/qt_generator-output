/// C++ type: <span style='color: green;'>```QProgressDialog```</span>
#[repr(C)]
pub struct ProgressDialog(u8);

impl ProgressDialog {
  /// C++ method: <span style='color: green;'>```bool QProgressDialog::autoClose() const```</span>
  ///
  ///
  pub fn auto_close(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_autoClose(self as *const ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QProgressDialog::autoReset() const```</span>
  ///
  ///
  pub fn auto_reset(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_autoReset(self as *const ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::cancel()```</span>
  ///
  ///
  pub fn cancel(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_cancel(self as *mut ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```QString QProgressDialog::labelText() const```</span>
  ///
  ///
  pub fn label_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_labelText_to_output(self as *const ::progress_dialog::ProgressDialog,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QProgressDialog::maximum() const```</span>
  ///
  ///
  pub fn maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_maximum(self as *const ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QProgressDialog::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_metaObject(self as *const ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```int QProgressDialog::minimum() const```</span>
  ///
  ///
  pub fn minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_minimum(self as *const ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```int QProgressDialog::minimumDuration() const```</span>
  ///
  ///
  pub fn minimum_duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_minimumDuration(self as *const ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```void QProgressDialog::open(QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn open(&mut self, receiver: *mut ::qt_core::object::Object, member: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QProgressDialog_open(self as *mut ::progress_dialog::ProgressDialog,
                                             receiver,
                                             member)
  }

  /// C++ method: <span style='color: green;'>```virtual int QProgressDialog::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QProgressDialog_qt_metacall(self as *mut ::progress_dialog::ProgressDialog,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QProgressDialog::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QProgressDialog_qt_metacast(self as *mut ::progress_dialog::ProgressDialog, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_reset(self as *mut ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```void QProgressDialog::setAutoClose(bool close)```</span>
  ///
  ///
  pub fn set_auto_close(&mut self, close: bool) {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_setAutoClose(self as *mut ::progress_dialog::ProgressDialog, close) }
  }

  /// C++ method: <span style='color: green;'>```void QProgressDialog::setAutoReset(bool reset)```</span>
  ///
  ///
  pub fn set_auto_reset(&mut self, reset: bool) {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_setAutoReset(self as *mut ::progress_dialog::ProgressDialog, reset) }
  }

  /// C++ method: <span style='color: green;'>```void QProgressDialog::setBar(QProgressBar* bar)```</span>
  ///
  ///
  pub unsafe fn set_bar(&mut self, bar: *mut ::progress_bar::ProgressBar) {
    ::ffi::qt_widgets_c_QProgressDialog_setBar(self as *mut ::progress_dialog::ProgressDialog, bar)
  }

  /// C++ method: <span style='color: green;'>```void QProgressDialog::setCancelButton(QPushButton* button)```</span>
  ///
  ///
  pub unsafe fn set_cancel_button(&mut self, button: *mut ::push_button::PushButton) {
    ::ffi::qt_widgets_c_QProgressDialog_setCancelButton(self as *mut ::progress_dialog::ProgressDialog, button)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::setCancelButtonText(const QString& text)```</span>
  ///
  ///
  pub fn set_cancel_button_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QProgressDialog_setCancelButtonText(self as *mut ::progress_dialog::ProgressDialog,
                                                              text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QProgressDialog::setLabel(QLabel* label)```</span>
  ///
  ///
  pub unsafe fn set_label(&mut self, label: *mut ::label::Label) {
    ::ffi::qt_widgets_c_QProgressDialog_setLabel(self as *mut ::progress_dialog::ProgressDialog, label)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::setLabelText(const QString& text)```</span>
  ///
  ///
  pub fn set_label_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QProgressDialog_setLabelText(self as *mut ::progress_dialog::ProgressDialog,
                                                       text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::setMaximum(int maximum)```</span>
  ///
  ///
  pub fn set_maximum(&mut self, maximum: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_setMaximum(self as *mut ::progress_dialog::ProgressDialog, maximum) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::setMinimum(int minimum)```</span>
  ///
  ///
  pub fn set_minimum(&mut self, minimum: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_setMinimum(self as *mut ::progress_dialog::ProgressDialog, minimum) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::setMinimumDuration(int ms)```</span>
  ///
  ///
  pub fn set_minimum_duration(&mut self, ms: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QProgressDialog_setMinimumDuration(self as *mut ::progress_dialog::ProgressDialog, ms)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::setRange(int minimum, int maximum)```</span>
  ///
  ///
  pub fn set_range(&mut self, minimum: ::libc::c_int, maximum: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QProgressDialog_setRange(self as *mut ::progress_dialog::ProgressDialog,
                                                   minimum,
                                                   maximum)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QProgressDialog::setValue(int progress)```</span>
  ///
  ///
  pub fn set_value(&mut self, progress: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_setValue(self as *mut ::progress_dialog::ProgressDialog, progress) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QProgressDialog::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_sizeHint_to_output(self as *const ::progress_dialog::ProgressDialog,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QProgressDialog::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProgressDialog_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QProgressDialog::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QProgressDialog_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QProgressDialog::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_value(self as *const ::progress_dialog::ProgressDialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QProgressDialog::wasCanceled() const```</span>
  ///
  ///
  pub fn was_canceled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QProgressDialog_wasCanceled(self as *const ::progress_dialog::ProgressDialog) }
  }
}

impl ::cpp_utils::CppDeletable for ::progress_dialog::ProgressDialog {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QProgressDialog_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ProgressDialog`.
  pub struct Signals<'a>(&'a ::progress_dialog::ProgressDialog);
  /// Represents a built-in Qt signal `QProgressDialog::canceled`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.signals().canceled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Canceled<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Canceled<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2canceled()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Canceled<'a> {}
  /// Represents a built-in Qt signal `QProgressDialog::rejected`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Rejected<'a>(&'a ::progress_dialog::ProgressDialog);
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
  /// Represents a built-in Qt signal `QProgressDialog::finished`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Finished<'a>(&'a ::progress_dialog::ProgressDialog);
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
  /// Represents a built-in Qt signal `QProgressDialog::accepted`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Accepted<'a>(&'a ::progress_dialog::ProgressDialog);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QProgressDialog::canceled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn canceled(&self) -> Canceled {
      Canceled(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProgressDialog::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProgressDialog::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QProgressDialog::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ProgressDialog`.
  pub struct Slots<'a>(&'a ::progress_dialog::ProgressDialog);
  /// Represents a built-in Qt slot `QProgressDialog::open`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Open<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::accept`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Accept<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::setRange`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().set_range()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct SetRange<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for SetRange<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRange(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::setMinimum`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().set_minimum()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct SetMinimum<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for SetMinimum<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimum(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::done`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().done()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Done<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Done<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1done(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::setValue`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct SetValue<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::setCancelButtonText`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().set_cancel_button_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct SetCancelButtonText<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for SetCancelButtonText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCancelButtonText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::reject`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Reject<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::cancel`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().cancel()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Cancel<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Cancel<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1cancel()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::setMinimumDuration`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().set_minimum_duration()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct SetMinimumDuration<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for SetMinimumDuration<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinimumDuration(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::reset`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Reset<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Reset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reset()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::exec`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct Exec<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::setLabelText`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().set_label_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct SetLabelText<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for SetLabelText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLabelText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::showExtension`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct ShowExtension<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::setMaximum`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().set_maximum()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct SetMaximum<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for SetMaximum<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximum(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QProgressDialog::forceShow`.
  ///
  /// An object of this type can be created from `ProgressDialog` with `object.slots().force_show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ProgressDialog` object.
  pub struct ForceShow<'a>(&'a ::progress_dialog::ProgressDialog);
  impl<'a> ::qt_core::connection::Receiver for ForceShow<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1forceShow()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QProgressDialog::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::setRange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_range(&self) -> SetRange {
      SetRange(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::setMinimum`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum(&self) -> SetMinimum {
      SetMinimum(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::done`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn done(&self) -> Done {
      Done(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::setCancelButtonText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_cancel_button_text(&self) -> SetCancelButtonText {
      SetCancelButtonText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::cancel`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cancel(&self) -> Cancel {
      Cancel(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::setMinimumDuration`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minimum_duration(&self) -> SetMinimumDuration {
      SetMinimumDuration(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::reset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset(&self) -> Reset {
      Reset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::setLabelText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_label_text(&self) -> SetLabelText {
      SetLabelText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::setMaximum`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum(&self) -> SetMaximum {
      SetMaximum(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QProgressDialog::forceShow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn force_show(&self) -> ForceShow {
      ForceShow(self.0)
    }
  }
  impl ::progress_dialog::ProgressDialog {
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

impl ::cpp_utils::DynamicCast<::progress_dialog::ProgressDialog> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::progress_dialog::ProgressDialog> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QDialog(self as *mut ::dialog::Dialog)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::progress_dialog::ProgressDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::progress_dialog::ProgressDialog> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::progress_dialog::ProgressDialog> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::progress_dialog::ProgressDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::progress_dialog::ProgressDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QObject_ptr(self as *mut ::progress_dialog::ProgressDialog)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QObject_ptr(self as *const ::progress_dialog::ProgressDialog as *mut ::progress_dialog::ProgressDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::progress_dialog::ProgressDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QPaintDevice_ptr(self as *mut ::progress_dialog::ProgressDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QPaintDevice_ptr(self as *const ::progress_dialog::ProgressDialog as *mut ::progress_dialog::ProgressDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::progress_dialog::ProgressDialog {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QDialog_ptr(self as *mut ::progress_dialog::ProgressDialog)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QDialog_ptr(self as *const ::progress_dialog::ProgressDialog as *mut ::progress_dialog::ProgressDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::progress_dialog::ProgressDialog {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QWidget_ptr(self as *mut ::progress_dialog::ProgressDialog)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QWidget_ptr(self as *const ::progress_dialog::ProgressDialog as *mut ::progress_dialog::ProgressDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::progress_dialog::ProgressDialog> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::progress_dialog::ProgressDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::progress_dialog::ProgressDialog {
    let ffi_result = ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::progress_dialog::ProgressDialog> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::progress_dialog::ProgressDialog {
    let ffi_result = ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::progress_dialog::ProgressDialog {
    let ffi_result = ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::progress_dialog::ProgressDialog> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::progress_dialog::ProgressDialog {
    let ffi_result = ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::progress_dialog::ProgressDialog {
    let ffi_result = ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::progress_dialog::ProgressDialog> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::progress_dialog::ProgressDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::progress_dialog::ProgressDialog {
    let ffi_result = ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::progress_dialog::ProgressDialog {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QDialog_ptr(self as *const ::progress_dialog::ProgressDialog as *mut ::progress_dialog::ProgressDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::progress_dialog::ProgressDialog {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QProgressDialog_G_static_cast_QDialog_ptr(self as *mut ::progress_dialog::ProgressDialog)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

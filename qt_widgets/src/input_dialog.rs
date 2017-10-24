/// C++ type: <span style='color: green;'>```QInputDialog```</span>
#[repr(C)]
pub struct InputDialog(u8);

impl InputDialog {
  /// C++ method: <span style='color: green;'>```QString QInputDialog::cancelButtonText() const```</span>
  ///
  ///
  pub fn cancel_button_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_cancelButtonText_to_output(self as *const ::input_dialog::InputDialog,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QInputDialog::comboBoxItems() const```</span>
  ///
  ///
  pub fn combo_box_items(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_comboBoxItems_to_output(self as *const ::input_dialog::InputDialog,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QInputDialog::done(int result)```</span>
  ///
  ///
  pub fn done(&mut self, result: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_done(self as *mut ::input_dialog::InputDialog, result) }
  }

  /// C++ method: <span style='color: green;'>```int QInputDialog::doubleDecimals() const```</span>
  ///
  ///
  pub fn double_decimals(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_doubleDecimals(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```double QInputDialog::doubleMaximum() const```</span>
  ///
  ///
  pub fn double_maximum(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_doubleMaximum(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```double QInputDialog::doubleMinimum() const```</span>
  ///
  ///
  pub fn double_minimum(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_doubleMinimum(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```double QInputDialog::doubleValue() const```</span>
  ///
  ///
  pub fn double_value(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_doubleValue(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```QInputDialog::InputMode QInputDialog::inputMode() const```</span>
  ///
  ///
  pub fn input_mode(&self) -> ::input_dialog::InputMode {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_inputMode(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```int QInputDialog::intMaximum() const```</span>
  ///
  ///
  pub fn int_maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_intMaximum(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```int QInputDialog::intMinimum() const```</span>
  ///
  ///
  pub fn int_minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_intMinimum(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```int QInputDialog::intStep() const```</span>
  ///
  ///
  pub fn int_step(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_intStep(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```int QInputDialog::intValue() const```</span>
  ///
  ///
  pub fn int_value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_intValue(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```bool QInputDialog::isComboBoxEditable() const```</span>
  ///
  ///
  pub fn is_combo_box_editable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_isComboBoxEditable(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```QString QInputDialog::labelText() const```</span>
  ///
  ///
  pub fn label_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_labelText_to_output(self as *const ::input_dialog::InputDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QInputDialog::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_metaObject(self as *const ::input_dialog::InputDialog) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QInputDialog::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_minimumSizeHint_to_output(self as *const ::input_dialog::InputDialog,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QInputDialog::okButtonText() const```</span>
  ///
  ///
  pub fn ok_button_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_okButtonText_to_output(self as *const ::input_dialog::InputDialog,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::open(QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn open(&mut self, receiver: *mut ::qt_core::object::Object, member: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QInputDialog_open(self as *mut ::input_dialog::InputDialog, receiver, member)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QInputDialog::InputDialogOption> QInputDialog::options() const```</span>
  ///
  ///
  pub fn options(&self) -> ::qt_core::flags::Flags<::input_dialog::InputDialogOption> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_options(self as *const ::input_dialog::InputDialog) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual int QInputDialog::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QInputDialog_qt_metacall(self as *mut ::input_dialog::InputDialog,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QInputDialog::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QInputDialog_qt_metacast(self as *mut ::input_dialog::InputDialog, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setCancelButtonText(const QString& text)```</span>
  ///
  ///
  pub fn set_cancel_button_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QInputDialog_setCancelButtonText(self as *mut ::input_dialog::InputDialog,
                                                           text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setComboBoxEditable(bool editable)```</span>
  ///
  ///
  pub fn set_combo_box_editable(&mut self, editable: bool) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setComboBoxEditable(self as *mut ::input_dialog::InputDialog, editable) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setComboBoxItems(const QStringList& items)```</span>
  ///
  ///
  pub fn set_combo_box_items(&mut self, items: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QInputDialog_setComboBoxItems(self as *mut ::input_dialog::InputDialog,
                                                        items as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setDoubleDecimals(int decimals)```</span>
  ///
  ///
  pub fn set_double_decimals(&mut self, decimals: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setDoubleDecimals(self as *mut ::input_dialog::InputDialog, decimals) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setDoubleMaximum(double max)```</span>
  ///
  ///
  pub fn set_double_maximum(&mut self, max: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setDoubleMaximum(self as *mut ::input_dialog::InputDialog, max) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setDoubleMinimum(double min)```</span>
  ///
  ///
  pub fn set_double_minimum(&mut self, min: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setDoubleMinimum(self as *mut ::input_dialog::InputDialog, min) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setDoubleRange(double min, double max)```</span>
  ///
  ///
  pub fn set_double_range(&mut self, min: ::libc::c_double, max: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setDoubleRange(self as *mut ::input_dialog::InputDialog, min, max) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setDoubleValue(double value)```</span>
  ///
  ///
  pub fn set_double_value(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setDoubleValue(self as *mut ::input_dialog::InputDialog, value) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setInputMode(QInputDialog::InputMode mode)```</span>
  ///
  ///
  pub fn set_input_mode(&mut self, mode: ::input_dialog::InputMode) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setInputMode(self as *mut ::input_dialog::InputDialog, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setIntMaximum(int max)```</span>
  ///
  ///
  pub fn set_int_maximum(&mut self, max: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setIntMaximum(self as *mut ::input_dialog::InputDialog, max) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setIntMinimum(int min)```</span>
  ///
  ///
  pub fn set_int_minimum(&mut self, min: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setIntMinimum(self as *mut ::input_dialog::InputDialog, min) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setIntRange(int min, int max)```</span>
  ///
  ///
  pub fn set_int_range(&mut self, min: ::libc::c_int, max: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setIntRange(self as *mut ::input_dialog::InputDialog, min, max) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setIntStep(int step)```</span>
  ///
  ///
  pub fn set_int_step(&mut self, step: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setIntStep(self as *mut ::input_dialog::InputDialog, step) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setIntValue(int value)```</span>
  ///
  ///
  pub fn set_int_value(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setIntValue(self as *mut ::input_dialog::InputDialog, value) }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setLabelText(const QString& text)```</span>
  ///
  ///
  pub fn set_label_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QInputDialog_setLabelText(self as *mut ::input_dialog::InputDialog,
                                                    text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setOkButtonText(const QString& text)```</span>
  ///
  ///
  pub fn set_ok_button_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QInputDialog_setOkButtonText(self as *mut ::input_dialog::InputDialog,
                                                       text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QInputDialog::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::input_dialog::InputDialogOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QInputDialog::setOption(QInputDialog::InputDialogOption option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::input_dialog::InputDialogOption, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QInputDialog::setOption(QInputDialog::InputDialogOption option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::InputDialogSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QInputDialog::setOptions(QFlags<QInputDialog::InputDialogOption> options)```</span>
  ///
  ///
  pub fn set_options(&mut self, options: ::qt_core::flags::Flags<::input_dialog::InputDialogOption>) {
    unsafe {
      ::ffi::qt_widgets_c_QInputDialog_setOptions(self as *mut ::input_dialog::InputDialog,
                                                  options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setTextEchoMode(QLineEdit::EchoMode mode)```</span>
  ///
  ///
  pub fn set_text_echo_mode(&mut self, mode: &::line_edit::EchoMode) {
    unsafe {
      ::ffi::qt_widgets_c_QInputDialog_setTextEchoMode(self as *mut ::input_dialog::InputDialog,
                                                       mode as *const ::line_edit::EchoMode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QInputDialog::setTextValue(const QString& text)```</span>
  ///
  ///
  pub fn set_text_value(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QInputDialog_setTextValue(self as *mut ::input_dialog::InputDialog,
                                                    text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QInputDialog::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_setVisible(self as *mut ::input_dialog::InputDialog, visible) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QInputDialog::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_sizeHint_to_output(self as *const ::input_dialog::InputDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QInputDialog::testOption(QInputDialog::InputDialogOption option) const```</span>
  ///
  ///
  pub fn test_option(&self, option: ::input_dialog::InputDialogOption) -> bool {
    unsafe { ::ffi::qt_widgets_c_QInputDialog_testOption(self as *const ::input_dialog::InputDialog, option) }
  }

  /// C++ method: <span style='color: green;'>```QString QInputDialog::textValue() const```</span>
  ///
  ///
  pub fn text_value(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_textValue_to_output(self as *const ::input_dialog::InputDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QInputDialog::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QInputDialog_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QInputDialog::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QInputDialog_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::input_dialog::InputDialog {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QInputDialog_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `InputDialog`.
  pub struct Signals<'a>(&'a ::input_dialog::InputDialog);
  /// Represents a built-in Qt signal `QInputDialog::intValueChanged`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().int_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct IntValueChanged<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for IntValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2intValueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IntValueChanged<'a> {}
  /// Represents a built-in Qt signal `QInputDialog::textValueChanged`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().text_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct TextValueChanged<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for TextValueChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textValueChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextValueChanged<'a> {}
  /// Represents a built-in Qt signal `QInputDialog::intValueSelected`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().int_value_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct IntValueSelected<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for IntValueSelected<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2intValueSelected(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IntValueSelected<'a> {}
  /// Represents a built-in Qt signal `QInputDialog::doubleValueChanged`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().double_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct DoubleValueChanged<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for DoubleValueChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2doubleValueChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DoubleValueChanged<'a> {}
  /// Represents a built-in Qt signal `QInputDialog::doubleValueSelected`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().double_value_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct DoubleValueSelected<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for DoubleValueSelected<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2doubleValueSelected(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DoubleValueSelected<'a> {}
  /// Represents a built-in Qt signal `QInputDialog::accepted`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct Accepted<'a>(&'a ::input_dialog::InputDialog);
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
  /// Represents a built-in Qt signal `QInputDialog::rejected`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct Rejected<'a>(&'a ::input_dialog::InputDialog);
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
  /// Represents a built-in Qt signal `QInputDialog::textValueSelected`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().text_value_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct TextValueSelected<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for TextValueSelected<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textValueSelected(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextValueSelected<'a> {}
  /// Represents a built-in Qt signal `QInputDialog::finished`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct Finished<'a>(&'a ::input_dialog::InputDialog);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QInputDialog::intValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn int_value_changed(&self) -> IntValueChanged {
      IntValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::textValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_value_changed(&self) -> TextValueChanged {
      TextValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::intValueSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn int_value_selected(&self) -> IntValueSelected {
      IntValueSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::doubleValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_value_changed(&self) -> DoubleValueChanged {
      DoubleValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::doubleValueSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_value_selected(&self) -> DoubleValueSelected {
      DoubleValueSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::textValueSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_value_selected(&self) -> TextValueSelected {
      TextValueSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QInputDialog::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `InputDialog`.
  pub struct Slots<'a>(&'a ::input_dialog::InputDialog);
  /// Represents a built-in Qt slot `QInputDialog::accept`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct Accept<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  /// Represents a built-in Qt slot `QInputDialog::reject`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct Reject<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QInputDialog::exec`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct Exec<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QInputDialog::showExtension`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct ShowExtension<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QInputDialog::open`.
  ///
  /// An object of this type can be created from `InputDialog` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputDialog` object.
  pub struct Open<'a>(&'a ::input_dialog::InputDialog);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QInputDialog::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputDialog::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputDialog::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputDialog::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QInputDialog::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
  }
  impl ::input_dialog::InputDialog {
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

/// C++ type: <span style='color: green;'>```QInputDialog::InputDialogOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputDialogOption {
  /// C++ enum variant: <span style='color: green;'>```NoButtons = 1```</span>
  NoButtons = 1,
  /// C++ enum variant: <span style='color: green;'>```UseListViewForComboBoxItems = 2```</span>
  UseListViewForComboBoxItems = 2,
  /// C++ enum variant: <span style='color: green;'>```UsePlainTextEditForTextInput = 4```</span>
  UsePlainTextEditForTextInput = 4,
}

impl ::qt_core::flags::FlaggableEnum for InputDialogOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "InputDialogOption"
  }
}

/// C++ type: <span style='color: green;'>```QInputDialog::InputMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputMode {
  /// C++ enum variant: <span style='color: green;'>```TextInput = 0```</span>
  Text = 0,
  /// C++ enum variant: <span style='color: green;'>```IntInput = 1```</span>
  Int = 1,
  /// C++ enum variant: <span style='color: green;'>```DoubleInput = 2```</span>
  Double = 2,
}

impl ::cpp_utils::DynamicCast<::input_dialog::InputDialog> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::input_dialog::InputDialog> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QDialog(self as *mut ::dialog::Dialog)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::input_dialog::InputDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::input_dialog::InputDialog> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::input_dialog::InputDialog> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::input_dialog::InputDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::input_dialog::InputDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QObject_ptr(self as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QObject_ptr(self as *const ::input_dialog::InputDialog as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::input_dialog::InputDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QPaintDevice_ptr(self as *mut ::input_dialog::InputDialog)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QPaintDevice_ptr(self as *const ::input_dialog::InputDialog as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::input_dialog::InputDialog {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QDialog_ptr(self as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QDialog_ptr(self as *const ::input_dialog::InputDialog as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::input_dialog::InputDialog {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QWidget_ptr(self as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QWidget_ptr(self as *const ::input_dialog::InputDialog as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_dialog::InputDialog> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_dialog::InputDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_dialog::InputDialog {
    let ffi_result = ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_dialog::InputDialog> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_dialog::InputDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_dialog::InputDialog {
    let ffi_result = ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_dialog::InputDialog> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_dialog::InputDialog {
    let ffi_result = ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_dialog::InputDialog {
    let ffi_result = ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_dialog::InputDialog> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_dialog::InputDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_dialog::InputDialog {
    let ffi_result = ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_dialog::InputDialog {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QDialog_ptr(self as *const ::input_dialog::InputDialog as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_dialog::InputDialog {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QInputDialog_G_static_cast_QDialog_ptr(self as *mut ::input_dialog::InputDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [InputDialog::set_option](../struct.InputDialog.html#method.set_option) method.
  pub trait InputDialogSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::input_dialog::InputDialog) -> ();
  }
  impl<'largs> InputDialogSetOptionArgs<'largs> for ::input_dialog::InputDialogOption {
    fn exec(self, original_self: &'largs mut ::input_dialog::InputDialog) -> () {
      let option = self;
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_setOption_option(original_self as *mut ::input_dialog::InputDialog, option)
      }
    }
  }
  impl<'largs> InputDialogSetOptionArgs<'largs> for (::input_dialog::InputDialogOption, bool) {
    fn exec(self, original_self: &'largs mut ::input_dialog::InputDialog) -> () {
      let option = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QInputDialog_setOption_option_on(original_self as *mut ::input_dialog::InputDialog,
                                                             option,
                                                             on)
      }
    }
  }
}

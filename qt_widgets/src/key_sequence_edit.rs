/// C++ type: <span style='color: green;'>```QKeySequenceEdit```</span>
#[repr(C)]
pub struct KeySequenceEdit(u8);

impl KeySequenceEdit {
  /// C++ method: <span style='color: green;'>```[slot] void QKeySequenceEdit::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_clear(self as *mut ::key_sequence_edit::KeySequenceEdit) }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence QKeySequenceEdit::keySequence() const```</span>
  ///
  ///
  pub fn key_sequence(&self) -> ::qt_gui::key_sequence::KeySequence {
    {
      let mut object: ::qt_gui::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QKeySequenceEdit_keySequence_to_output(self as *const ::key_sequence_edit::KeySequenceEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QKeySequenceEdit::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_metaObject(self as *const ::key_sequence_edit::KeySequenceEdit) }
  }

  /// C++ method: <span style='color: green;'>```QKeySequenceEdit::QKeySequenceEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequenceEdit::QKeySequenceEdit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::key_sequence::KeySequence) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequenceEdit::QKeySequenceEdit(const QKeySequence& keySequence)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>
    where Args: overloading::KeySequenceEditNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QKeySequenceEdit::QKeySequenceEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequenceEdit::QKeySequenceEdit(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QKeySequenceEdit::QKeySequenceEdit(const QKeySequence& keySequence, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>
    where Args: overloading::KeySequenceEditNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QKeySequenceEdit::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QKeySequenceEdit_qt_metacall(self as *mut ::key_sequence_edit::KeySequenceEdit,
                                                     arg1 as *const ::qt_core::meta_object::Call,
                                                     arg2,
                                                     arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QKeySequenceEdit::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QKeySequenceEdit_qt_metacast(self as *mut ::key_sequence_edit::KeySequenceEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QKeySequenceEdit::setKeySequence(const QKeySequence& keySequence)```</span>
  ///
  ///
  pub fn set_key_sequence(&mut self, key_sequence: &::qt_gui::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_widgets_c_QKeySequenceEdit_setKeySequence(self as *mut ::key_sequence_edit::KeySequenceEdit,
                                                          key_sequence as *const ::qt_gui::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QKeySequenceEdit::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QKeySequenceEdit_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QKeySequenceEdit::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QKeySequenceEdit_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::key_sequence_edit::KeySequenceEdit {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QKeySequenceEdit_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `KeySequenceEdit`.
  pub struct Signals<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  /// Represents a built-in Qt signal `QKeySequenceEdit::windowIconChanged`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct WindowIconChanged<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
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
  /// Represents a built-in Qt signal `QKeySequenceEdit::editingFinished`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.signals().editing_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct EditingFinished<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for EditingFinished<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2editingFinished()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EditingFinished<'a> {}
  /// Represents a built-in Qt signal `QKeySequenceEdit::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
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
  /// Represents a built-in Qt signal `QKeySequenceEdit::keySequenceChanged`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.signals().key_sequence_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct KeySequenceChanged<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for KeySequenceChanged<'a> {
    type Arguments = (&'static ::qt_gui::key_sequence::KeySequence,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2keySequenceChanged(const QKeySequence&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for KeySequenceChanged<'a> {}
  /// Represents a built-in Qt signal `QKeySequenceEdit::windowTitleChanged`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct WindowTitleChanged<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
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
  /// Represents a built-in Qt signal `QKeySequenceEdit::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct WindowIconTextChanged<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
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
    /// Returns an object representing a built-in Qt signal `QKeySequenceEdit::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QKeySequenceEdit::editingFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editing_finished(&self) -> EditingFinished {
      EditingFinished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QKeySequenceEdit::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QKeySequenceEdit::keySequenceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn key_sequence_changed(&self) -> KeySequenceChanged {
      KeySequenceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QKeySequenceEdit::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QKeySequenceEdit::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `KeySequenceEdit`.
  pub struct Slots<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  /// Represents a built-in Qt slot `QKeySequenceEdit::setHidden`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetHidden<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::update`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Update<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::showMaximized`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct ShowMaximized<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::lower`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Lower<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setWindowModified`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetWindowModified<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::updateMicroFocus`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct UpdateMicroFocus<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::close`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Close<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::hide`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Hide<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setKeySequence`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_key_sequence()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetKeySequence<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetKeySequence<'a> {
    type Arguments = (&'static ::qt_gui::key_sequence::KeySequence,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setKeySequence(const QKeySequence&)\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::showMinimized`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct ShowMinimized<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::clear`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Clear<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setFocus`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetFocus<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setEnabled`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetEnabled<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setStyleSheet`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetStyleSheet<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::showFullScreen`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct ShowFullScreen<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::repaint`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Repaint<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::show`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Show<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::raise`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct Raise<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setWindowTitle`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetWindowTitle<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::showNormal`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct ShowNormal<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setVisible`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetVisible<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QKeySequenceEdit::setDisabled`.
  ///
  /// An object of this type can be created from `KeySequenceEdit` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeySequenceEdit` object.
  pub struct SetDisabled<'a>(&'a ::key_sequence_edit::KeySequenceEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setKeySequence`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_key_sequence(&self) -> SetKeySequence {
      SetKeySequence(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QKeySequenceEdit::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
  }
  impl ::key_sequence_edit::KeySequenceEdit {
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

impl ::cpp_utils::DynamicCast<::key_sequence_edit::KeySequenceEdit> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::key_sequence_edit::KeySequenceEdit> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QKeySequenceEdit_G_dynamic_cast_QKeySequenceEdit_ptr(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::key_sequence_edit::KeySequenceEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_dynamic_cast_QKeySequenceEdit_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::key_sequence_edit::KeySequenceEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QObject_ptr(self as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QObject_ptr(self as *const ::key_sequence_edit::KeySequenceEdit as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::key_sequence_edit::KeySequenceEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QPaintDevice_ptr(self as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QPaintDevice_ptr(self as *const ::key_sequence_edit::KeySequenceEdit as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::key_sequence_edit::KeySequenceEdit {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QWidget_ptr(self as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QWidget_ptr(self as *const ::key_sequence_edit::KeySequenceEdit as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::key_sequence_edit::KeySequenceEdit> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::key_sequence_edit::KeySequenceEdit {
    let ffi_result = ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::key_sequence_edit::KeySequenceEdit {
    let ffi_result = ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::key_sequence_edit::KeySequenceEdit> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::key_sequence_edit::KeySequenceEdit {
    let ffi_result = ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::key_sequence_edit::KeySequenceEdit {
    let ffi_result = ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::key_sequence_edit::KeySequenceEdit> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::key_sequence_edit::KeySequenceEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::key_sequence_edit::KeySequenceEdit {
    let ffi_result = ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QKeySequenceEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::key_sequence_edit::KeySequenceEdit {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QWidget_ptr(self as *const ::key_sequence_edit::KeySequenceEdit as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::key_sequence_edit::KeySequenceEdit {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_G_static_cast_QWidget_ptr(self as *mut ::key_sequence_edit::KeySequenceEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [KeySequenceEdit::new](../struct.KeySequenceEdit.html#method.new) method.
  pub trait KeySequenceEditNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>;
  }
  impl<'a> KeySequenceEditNewArgs for &'a ::qt_gui::key_sequence::KeySequence {
    fn exec(self) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit> {
      let key_sequence = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_new_keySequence(key_sequence as *const ::qt_gui::key_sequence::KeySequence) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl KeySequenceEditNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QKeySequenceEdit_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [KeySequenceEdit::new_unsafe](../struct.KeySequenceEdit.html#method.new_unsafe) method.
  pub trait KeySequenceEditNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit>;
  }
  impl<'a> KeySequenceEditNewUnsafeArgs for (&'a ::qt_gui::key_sequence::KeySequence, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit> {
      let key_sequence = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QKeySequenceEdit_new_keySequence_parent(key_sequence as *const ::qt_gui::key_sequence::KeySequence, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl KeySequenceEditNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::key_sequence_edit::KeySequenceEdit> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QKeySequenceEdit_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

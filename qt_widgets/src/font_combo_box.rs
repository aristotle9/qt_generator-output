/// C++ type: <span style='color: green;'>```QFontComboBox```</span>
#[repr(C)]
pub struct FontComboBox(u8);

impl FontComboBox {
  /// C++ method: <span style='color: green;'>```QFont QFontComboBox::currentFont() const```</span>
  ///
  ///
  pub fn current_font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_currentFont_to_output(self as *const ::font_combo_box::FontComboBox,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QFontComboBox::FontFilter> QFontComboBox::fontFilters() const```</span>
  ///
  ///
  pub fn font_filters(&self) -> ::qt_core::flags::Flags<::font_combo_box::FontFilter> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontComboBox_fontFilters(self as *const ::font_combo_box::FontComboBox) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFontComboBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QFontComboBox_metaObject(self as *const ::font_combo_box::FontComboBox) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFontComboBox::QFontComboBox()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::font_combo_box::FontComboBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFontComboBox::QFontComboBox(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::font_combo_box::FontComboBox> {
    let ffi_result = ::ffi::qt_widgets_c_QFontComboBox_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QFontComboBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QFontComboBox_qt_metacall(self as *mut ::font_combo_box::FontComboBox,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QFontComboBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QFontComboBox_qt_metacast(self as *mut ::font_combo_box::FontComboBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QFontComboBox::setCurrentFont(const QFont& f)```</span>
  ///
  ///
  pub fn set_current_font(&mut self, f: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QFontComboBox_setCurrentFont(self as *mut ::font_combo_box::FontComboBox,
                                                       f as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFontComboBox::setFontFilters(QFlags<QFontComboBox::FontFilter> filters)```</span>
  ///
  ///
  pub fn set_font_filters(&mut self, filters: ::qt_core::flags::Flags<::font_combo_box::FontFilter>) {
    unsafe {
      ::ffi::qt_widgets_c_QFontComboBox_setFontFilters(self as *mut ::font_combo_box::FontComboBox,
                                                       filters.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFontComboBox::setWritingSystem(QFontDatabase::WritingSystem arg1)```</span>
  ///
  ///
  pub fn set_writing_system(&mut self, arg1: &::qt_gui::font_database::WritingSystem) {
    unsafe {
      ::ffi::qt_widgets_c_QFontComboBox_setWritingSystem(self as *mut ::font_combo_box::FontComboBox,
                                                         arg1 as *const ::qt_gui::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QFontComboBox::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_sizeHint_to_output(self as *const ::font_combo_box::FontComboBox,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFontComboBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFontComboBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFontComboBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFontComboBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::font_combo_box::FontComboBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QFontComboBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FontComboBox`.
  pub struct Signals<'a>(&'a ::font_combo_box::FontComboBox);
  /// Represents a built-in Qt signal `QFontComboBox::highlighted`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().highlighted_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct HighlightedCInt<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for HighlightedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedCInt<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::highlighted`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().highlighted_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct HighlightedQtCoreStringRef<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for HighlightedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2highlighted(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HighlightedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::currentIndexChanged`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().current_index_changed_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct CurrentIndexChangedCInt<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for CurrentIndexChangedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentIndexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentIndexChangedCInt<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::currentIndexChanged`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().current_index_changed_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct CurrentIndexChangedQtCoreStringRef<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for CurrentIndexChangedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentIndexChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentIndexChangedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::activated`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().activated_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct ActivatedCInt<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for ActivatedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActivatedCInt<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::activated`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().activated_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct ActivatedQtCoreStringRef<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for ActivatedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActivatedQtCoreStringRef<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::currentTextChanged`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().current_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct CurrentTextChanged<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for CurrentTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentTextChanged<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::currentFontChanged`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().current_font_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct CurrentFontChanged<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for CurrentFontChanged<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentFontChanged(const QFont&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentFontChanged<'a> {}
  /// Represents a built-in Qt signal `QFontComboBox::editTextChanged`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.signals().edit_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct EditTextChanged<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for EditTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2editTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EditTextChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFontComboBox::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_c_int(&self) -> HighlightedCInt {
      HighlightedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::highlighted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn highlighted_qt_core_string_ref(&self) -> HighlightedQtCoreStringRef {
      HighlightedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::currentIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_index_changed_c_int(&self) -> CurrentIndexChangedCInt {
      CurrentIndexChangedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::currentIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_index_changed_qt_core_string_ref(&self) -> CurrentIndexChangedQtCoreStringRef {
      CurrentIndexChangedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated_c_int(&self) -> ActivatedCInt {
      ActivatedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated_qt_core_string_ref(&self) -> ActivatedQtCoreStringRef {
      ActivatedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::currentTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_text_changed(&self) -> CurrentTextChanged {
      CurrentTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::currentFontChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_font_changed(&self) -> CurrentFontChanged {
      CurrentFontChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontComboBox::editTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn edit_text_changed(&self) -> EditTextChanged {
      EditTextChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `FontComboBox`.
  pub struct Slots<'a>(&'a ::font_combo_box::FontComboBox);
  /// Represents a built-in Qt slot `QFontComboBox::setCurrentFont`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.slots().set_current_font()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct SetCurrentFont<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentFont<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentFont(const QFont&)\0"
    }
  }
  /// Represents a built-in Qt slot `QFontComboBox::clearEditText`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.slots().clear_edit_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct ClearEditText<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for ClearEditText<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearEditText()\0"
    }
  }
  /// Represents a built-in Qt slot `QFontComboBox::clear`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct Clear<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QFontComboBox::setEditText`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.slots().set_edit_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct SetEditText<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetEditText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEditText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QFontComboBox::setCurrentText`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.slots().set_current_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct SetCurrentText<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QFontComboBox::setCurrentIndex`.
  ///
  /// An object of this type can be created from `FontComboBox` with `object.slots().set_current_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontComboBox` object.
  pub struct SetCurrentIndex<'a>(&'a ::font_combo_box::FontComboBox);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentIndex(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QFontComboBox::setCurrentFont`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_font(&self) -> SetCurrentFont {
      SetCurrentFont(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontComboBox::clearEditText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_edit_text(&self) -> ClearEditText {
      ClearEditText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontComboBox::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontComboBox::setEditText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_edit_text(&self) -> SetEditText {
      SetEditText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontComboBox::setCurrentText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_text(&self) -> SetCurrentText {
      SetCurrentText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontComboBox::setCurrentIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_index(&self) -> SetCurrentIndex {
      SetCurrentIndex(self.0)
    }
  }
  impl ::font_combo_box::FontComboBox {
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

/// C++ type: <span style='color: green;'>```QFontComboBox::FontFilter```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FontFilter {
  /// C++ enum variant: <span style='color: green;'>```AllFonts = 0```</span>
  All = 0,
  /// C++ enum variant: <span style='color: green;'>```ScalableFonts = 1```</span>
  Scalable = 1,
  /// C++ enum variant: <span style='color: green;'>```NonScalableFonts = 2```</span>
  NonScalable = 2,
  /// C++ enum variant: <span style='color: green;'>```MonospacedFonts = 4```</span>
  Monospaced = 4,
  /// C++ enum variant: <span style='color: green;'>```ProportionalFonts = 8```</span>
  Proportional = 8,
}

impl ::qt_core::flags::FlaggableEnum for FontFilter {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "FontFilter"
  }
}

impl ::cpp_utils::DynamicCast<::font_combo_box::FontComboBox> for ::combo_box::ComboBox {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::font_combo_box::FontComboBox> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QComboBox(self as *mut ::combo_box::ComboBox)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::font_combo_box::FontComboBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QComboBox(self as *const ::combo_box::ComboBox as *mut ::combo_box::ComboBox) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::font_combo_box::FontComboBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::font_combo_box::FontComboBox> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::font_combo_box::FontComboBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_G_dynamic_cast_QFontComboBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::font_combo_box::FontComboBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QObject_ptr(self as *mut ::font_combo_box::FontComboBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QObject_ptr(self as *const ::font_combo_box::FontComboBox as *mut ::font_combo_box::FontComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::font_combo_box::FontComboBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QPaintDevice_ptr(self as *mut ::font_combo_box::FontComboBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QPaintDevice_ptr(self as *const ::font_combo_box::FontComboBox as *mut ::font_combo_box::FontComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::combo_box::ComboBox> for ::font_combo_box::FontComboBox {
  fn static_cast_mut(&mut self) -> &mut ::combo_box::ComboBox {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QComboBox_ptr(self as *mut ::font_combo_box::FontComboBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::combo_box::ComboBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QComboBox_ptr(self as *const ::font_combo_box::FontComboBox as *mut ::font_combo_box::FontComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::font_combo_box::FontComboBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QWidget_ptr(self as *mut ::font_combo_box::FontComboBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QWidget_ptr(self as *const ::font_combo_box::FontComboBox as *mut ::font_combo_box::FontComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_combo_box::FontComboBox> for ::combo_box::ComboBox {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_combo_box::FontComboBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QComboBox(self as *mut ::combo_box::ComboBox);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_combo_box::FontComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QComboBox(self as *const ::combo_box::ComboBox as *mut ::combo_box::ComboBox);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_combo_box::FontComboBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_combo_box::FontComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_combo_box::FontComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_combo_box::FontComboBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_combo_box::FontComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_combo_box::FontComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_combo_box::FontComboBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_combo_box::FontComboBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_combo_box::FontComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QFontComboBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::font_combo_box::FontComboBox {
  type Target = ::combo_box::ComboBox;
  fn deref(&self) -> &::combo_box::ComboBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QComboBox_ptr(self as *const ::font_combo_box::FontComboBox as *mut ::font_combo_box::FontComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::font_combo_box::FontComboBox {
  fn deref_mut(&mut self) -> &mut ::combo_box::ComboBox {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QFontComboBox_G_static_cast_QComboBox_ptr(self as *mut ::font_combo_box::FontComboBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

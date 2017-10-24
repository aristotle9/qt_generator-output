/// C++ type: <span style='color: green;'>```QFontDialog```</span>
#[repr(C)]
pub struct FontDialog(u8);

impl FontDialog {
  /// C++ method: <span style='color: green;'>```QFont QFontDialog::currentFont() const```</span>
  ///
  ///
  pub fn current_font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFontDialog_currentFont_to_output(self as *const ::font_dialog::FontDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDialog::getFont```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_font(*mut bool) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QFontDialog::getFont(bool* ok)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_font((*mut bool, *mut ::widget::Widget)) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QFontDialog::getFont(bool* ok, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_font((*mut bool, &::qt_gui::font::Font)) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QFontDialog::getFont(bool* ok, const QFont& initial)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn get_font((*mut bool, &::qt_gui::font::Font, *mut ::widget::Widget)) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QFontDialog::getFont(bool* ok, const QFont& initial, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn get_font((*mut bool, &::qt_gui::font::Font, *mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QFontDialog::getFont(bool* ok, const QFont& initial, QWidget* parent = ?, const QString& title = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn get_font((*mut bool, &::qt_gui::font::Font, *mut ::widget::Widget, &::qt_core::string::String, ::qt_core::flags::Flags<::font_dialog::FontDialogOption>)) -> ::qt_gui::font::Font```<br>
  /// C++ method: <span style='color: green;'>```static QFont QFontDialog::getFont(bool* ok, const QFont& initial, QWidget* parent = ?, const QString& title = ?, QFlags<QFontDialog::FontDialogOption> options = ?)```</span>
  ///
  ///
  pub unsafe fn get_font<Args>(args: Args) -> ::qt_gui::font::Font
    where Args: overloading::FontDialogGetFontArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFontDialog::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QFontDialog_metaObject(self as *const ::font_dialog::FontDialog) }
  }

  /// C++ method: <span style='color: green;'>```QFontDialog::QFontDialog```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontDialog::QFontDialog()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::font::Font) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontDialog::QFontDialog(const QFont& initial)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>
    where Args: overloading::FontDialogNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFontDialog::QFontDialog```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontDialog::QFontDialog(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::font::Font, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontDialog::QFontDialog(const QFont& initial, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>
    where Args: overloading::FontDialogNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QFontDialog::open(QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn open(&mut self, receiver: *mut ::qt_core::object::Object, member: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QFontDialog_open(self as *mut ::font_dialog::FontDialog, receiver, member)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QFontDialog::FontDialogOption> QFontDialog::options() const```</span>
  ///
  ///
  pub fn options(&self) -> ::qt_core::flags::Flags<::font_dialog::FontDialogOption> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_options(self as *const ::font_dialog::FontDialog) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual int QFontDialog::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QFontDialog_qt_metacall(self as *mut ::font_dialog::FontDialog,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QFontDialog::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QFontDialog_qt_metacast(self as *mut ::font_dialog::FontDialog, arg1)
  }

  /// C++ method: <span style='color: green;'>```QFont QFontDialog::selectedFont() const```</span>
  ///
  ///
  pub fn selected_font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFontDialog_selectedFont_to_output(self as *const ::font_dialog::FontDialog, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QFontDialog::setCurrentFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_current_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QFontDialog_setCurrentFont(self as *mut ::font_dialog::FontDialog,
                                                     font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDialog::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::font_dialog::FontDialogOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFontDialog::setOption(QFontDialog::FontDialogOption option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::font_dialog::FontDialogOption, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFontDialog::setOption(QFontDialog::FontDialogOption option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FontDialogSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QFontDialog::setOptions(QFlags<QFontDialog::FontDialogOption> options)```</span>
  ///
  ///
  pub fn set_options(&mut self, options: ::qt_core::flags::Flags<::font_dialog::FontDialogOption>) {
    unsafe {
      ::ffi::qt_widgets_c_QFontDialog_setOptions(self as *mut ::font_dialog::FontDialog,
                                                 options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QFontDialog::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QFontDialog_setVisible(self as *mut ::font_dialog::FontDialog, visible) }
  }

  /// C++ method: <span style='color: green;'>```bool QFontDialog::testOption(QFontDialog::FontDialogOption option) const```</span>
  ///
  ///
  pub fn test_option(&self, option: ::font_dialog::FontDialogOption) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFontDialog_testOption(self as *const ::font_dialog::FontDialog, option) }
  }

  /// C++ method: <span style='color: green;'>```static QString QFontDialog::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFontDialog_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFontDialog::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFontDialog_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::font_dialog::FontDialog {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QFontDialog_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FontDialog`.
  pub struct Signals<'a>(&'a ::font_dialog::FontDialog);
  /// Represents a built-in Qt signal `QFontDialog::finished`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct Finished<'a>(&'a ::font_dialog::FontDialog);
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
  /// Represents a built-in Qt signal `QFontDialog::fontSelected`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.signals().font_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct FontSelected<'a>(&'a ::font_dialog::FontDialog);
  impl<'a> ::qt_core::connection::Receiver for FontSelected<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fontSelected(const QFont&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FontSelected<'a> {}
  /// Represents a built-in Qt signal `QFontDialog::currentFontChanged`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.signals().current_font_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct CurrentFontChanged<'a>(&'a ::font_dialog::FontDialog);
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
  /// Represents a built-in Qt signal `QFontDialog::accepted`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct Accepted<'a>(&'a ::font_dialog::FontDialog);
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
  /// Represents a built-in Qt signal `QFontDialog::rejected`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct Rejected<'a>(&'a ::font_dialog::FontDialog);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFontDialog::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontDialog::fontSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn font_selected(&self) -> FontSelected {
      FontSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontDialog::currentFontChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_font_changed(&self) -> CurrentFontChanged {
      CurrentFontChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontDialog::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFontDialog::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `FontDialog`.
  pub struct Slots<'a>(&'a ::font_dialog::FontDialog);
  /// Represents a built-in Qt slot `QFontDialog::accept`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct Accept<'a>(&'a ::font_dialog::FontDialog);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  /// Represents a built-in Qt slot `QFontDialog::showExtension`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct ShowExtension<'a>(&'a ::font_dialog::FontDialog);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QFontDialog::reject`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct Reject<'a>(&'a ::font_dialog::FontDialog);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QFontDialog::exec`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct Exec<'a>(&'a ::font_dialog::FontDialog);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QFontDialog::open`.
  ///
  /// An object of this type can be created from `FontDialog` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FontDialog` object.
  pub struct Open<'a>(&'a ::font_dialog::FontDialog);
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
    /// Returns an object representing a built-in Qt slot `QFontDialog::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontDialog::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontDialog::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontDialog::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFontDialog::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
  }
  impl ::font_dialog::FontDialog {
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

/// C++ type: <span style='color: green;'>```QFontDialog::FontDialogOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FontDialogOption {
  /// C++ enum variant: <span style='color: green;'>```NoButtons = 1```</span>
  NoButtons = 1,
  /// C++ enum variant: <span style='color: green;'>```DontUseNativeDialog = 2```</span>
  DontUseNativeDialog = 2,
  /// C++ enum variant: <span style='color: green;'>```ScalableFonts = 4```</span>
  ScalableFonts = 4,
  /// C++ enum variant: <span style='color: green;'>```NonScalableFonts = 8```</span>
  NonScalableFonts = 8,
  /// C++ enum variant: <span style='color: green;'>```MonospacedFonts = 16```</span>
  MonospacedFonts = 16,
  /// C++ enum variant: <span style='color: green;'>```ProportionalFonts = 32```</span>
  ProportionalFonts = 32,
}

impl ::qt_core::flags::FlaggableEnum for FontDialogOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "FontDialogOption"
  }
}

impl ::cpp_utils::DynamicCast<::font_dialog::FontDialog> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::font_dialog::FontDialog> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QDialog(self as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::font_dialog::FontDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::font_dialog::FontDialog> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::font_dialog::FontDialog> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::font_dialog::FontDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_G_dynamic_cast_QFontDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::font_dialog::FontDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QObject_ptr(self as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QObject_ptr(self as *const ::font_dialog::FontDialog as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::font_dialog::FontDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QPaintDevice_ptr(self as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QPaintDevice_ptr(self as *const ::font_dialog::FontDialog as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::font_dialog::FontDialog {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QDialog_ptr(self as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QDialog_ptr(self as *const ::font_dialog::FontDialog as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::font_dialog::FontDialog {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QWidget_ptr(self as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QWidget_ptr(self as *const ::font_dialog::FontDialog as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_dialog::FontDialog> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_dialog::FontDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_dialog::FontDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_dialog::FontDialog> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_dialog::FontDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_dialog::FontDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_dialog::FontDialog> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_dialog::FontDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_dialog::FontDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::font_dialog::FontDialog> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::font_dialog::FontDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::font_dialog::FontDialog {
    let ffi_result = ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QFontDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::font_dialog::FontDialog {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QDialog_ptr(self as *const ::font_dialog::FontDialog as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::font_dialog::FontDialog {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QFontDialog_G_static_cast_QDialog_ptr(self as *mut ::font_dialog::FontDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FontDialog::get_font](../struct.FontDialog.html#method.get_font) method.
  pub trait FontDialogGetFontArgs {
    unsafe fn exec(self) -> ::qt_gui::font::Font;
  }
  impl FontDialogGetFontArgs for *mut bool {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let ok = self;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFontDialog_getFont_to_output_ok(ok, &mut object);
        object
      }
    }
  }
  impl<'a> FontDialogGetFontArgs for (*mut bool, &'a ::qt_gui::font::Font) {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let ok = self.0;
      let initial = self.1;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFontDialog_getFont_to_output_ok_initial(ok,
                                                                     initial as *const ::qt_gui::font::Font,
                                                                     &mut object);
        object
      }
    }
  }
  impl<'a> FontDialogGetFontArgs for (*mut bool, &'a ::qt_gui::font::Font, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let ok = self.0;
      let initial = self.1;
      let parent = self.2;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent(ok,
                                                                            initial as *const ::qt_gui::font::Font,
                                                                            parent,
                                                                            &mut object);
        object
      }
    }
  }
  impl<'a> FontDialogGetFontArgs
    for (*mut bool, &'a ::qt_gui::font::Font, *mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let ok = self.0;
      let initial = self.1;
      let parent = self.2;
      let title = self.3;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent_title(ok, initial as *const ::qt_gui::font::Font, parent, title as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> FontDialogGetFontArgs
    for (*mut bool,
                                          &'a ::qt_gui::font::Font,
                                          *mut ::widget::Widget,
                                          &'a ::qt_core::string::String,
                                          ::qt_core::flags::Flags<::font_dialog::FontDialogOption>) {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let ok = self.0;
      let initial = self.1;
      let parent = self.2;
      let title = self.3;
      let options = self.4;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFontDialog_getFont_to_output_ok_initial_parent_title_options(ok, initial as *const ::qt_gui::font::Font, parent, title as *const ::qt_core::string::String, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  impl FontDialogGetFontArgs for (*mut bool, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::qt_gui::font::Font {
      let ok = self.0;
      let parent = self.1;
      {
        let mut object: ::qt_gui::font::Font = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QFontDialog_getFont_to_output_ok_parent(ok, parent, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDialog::new](../struct.FontDialog.html#method.new) method.
  pub trait FontDialogNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>;
  }
  impl<'a> FontDialogNewArgs for &'a ::qt_gui::font::Font {
    fn exec(self) -> ::cpp_utils::CppBox<::font_dialog::FontDialog> {
      let initial = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_new_initial(initial as *const ::qt_gui::font::Font) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl FontDialogNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::font_dialog::FontDialog> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QFontDialog_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [FontDialog::new_unsafe](../struct.FontDialog.html#method.new_unsafe) method.
  pub trait FontDialogNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::font_dialog::FontDialog>;
  }
  impl<'a> FontDialogNewUnsafeArgs for (&'a ::qt_gui::font::Font, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::font_dialog::FontDialog> {
      let initial = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QFontDialog_new_initial_parent(initial as *const ::qt_gui::font::Font,
                                                                          parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl FontDialogNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::font_dialog::FontDialog> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QFontDialog_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [FontDialog::set_option](../struct.FontDialog.html#method.set_option) method.
  pub trait FontDialogSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::font_dialog::FontDialog) -> ();
  }
  impl<'largs> FontDialogSetOptionArgs<'largs> for ::font_dialog::FontDialogOption {
    fn exec(self, original_self: &'largs mut ::font_dialog::FontDialog) -> () {
      let option = self;
      unsafe {
        ::ffi::qt_widgets_c_QFontDialog_setOption_option(original_self as *mut ::font_dialog::FontDialog, option)
      }
    }
  }
  impl<'largs> FontDialogSetOptionArgs<'largs> for (::font_dialog::FontDialogOption, bool) {
    fn exec(self, original_self: &'largs mut ::font_dialog::FontDialog) -> () {
      let option = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QFontDialog_setOption_option_on(original_self as *mut ::font_dialog::FontDialog, option, on)
      }
    }
  }
}

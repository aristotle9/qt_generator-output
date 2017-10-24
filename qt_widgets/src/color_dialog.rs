/// C++ type: <span style='color: green;'>```QColorDialog```</span>
#[repr(C)]
pub struct ColorDialog(u8);

impl ColorDialog {
  /// C++ method: <span style='color: green;'>```QColor QColorDialog::currentColor() const```</span>
  ///
  ///
  pub fn current_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_currentColor_to_output(self as *const ::color_dialog::ColorDialog,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QColor QColorDialog::customColor(int index)```</span>
  ///
  ///
  pub fn custom_color(index: ::libc::c_int) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_customColor_to_output(index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static int QColorDialog::customCount()```</span>
  ///
  ///
  pub fn custom_count() -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QColorDialog_customCount() }
  }

  /// C++ method: <span style='color: green;'>```QColorDialog::getColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_color(()) -> ::qt_gui::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColorDialog::getColor()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_color(&::qt_gui::color::Color) -> ::qt_gui::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColorDialog::getColor(const QColor& initial = ?)```</span>
  ///
  ///
  pub fn get_color<Args>(args: Args) -> ::qt_gui::color::Color
    where Args: overloading::ColorDialogGetColorArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColorDialog::getColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_color_unsafe((&::qt_gui::color::Color, *mut ::widget::Widget)) -> ::qt_gui::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColorDialog::getColor(const QColor& initial = ?, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_color_unsafe((&::qt_gui::color::Color, *mut ::widget::Widget, &::qt_core::string::String)) -> ::qt_gui::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColorDialog::getColor(const QColor& initial = ?, QWidget* parent = ?, const QString& title = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn get_color_unsafe((&::qt_gui::color::Color, *mut ::widget::Widget, &::qt_core::string::String, ::qt_core::flags::Flags<::color_dialog::ColorDialogOption>)) -> ::qt_gui::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColorDialog::getColor(const QColor& initial = ?, QWidget* parent = ?, const QString& title = ?, QFlags<QColorDialog::ColorDialogOption> options = ?)```</span>
  ///
  ///
  pub unsafe fn get_color_unsafe<Args>(args: Args) -> ::qt_gui::color::Color
    where Args: overloading::ColorDialogGetColorUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColorDialog::getRgba```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_rgba(()) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```static unsigned int QColorDialog::getRgba()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_rgba(::libc::c_uint) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```static unsigned int QColorDialog::getRgba(unsigned int rgba = ?)```</span>
  ///
  ///
  pub fn get_rgba<Args>(args: Args) -> ::libc::c_uint
    where Args: overloading::ColorDialogGetRgbaArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColorDialog::getRgba```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_rgba_unsafe((::libc::c_uint, *mut bool)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```static unsigned int QColorDialog::getRgba(unsigned int rgba = ?, bool* ok = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_rgba_unsafe((::libc::c_uint, *mut bool, *mut ::widget::Widget)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```static unsigned int QColorDialog::getRgba(unsigned int rgba = ?, bool* ok = ?, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn get_rgba_unsafe<Args>(args: Args) -> ::libc::c_uint
    where Args: overloading::ColorDialogGetRgbaUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QColorDialog::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QColorDialog_metaObject(self as *const ::color_dialog::ColorDialog) }
  }

  /// C++ method: <span style='color: green;'>```QColorDialog::QColorDialog```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColorDialog::QColorDialog()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::color::Color) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColorDialog::QColorDialog(const QColor& initial)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>
    where Args: overloading::ColorDialogNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColorDialog::QColorDialog```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColorDialog::QColorDialog(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::color::Color, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColorDialog::QColorDialog(const QColor& initial, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>
    where Args: overloading::ColorDialogNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QColorDialog::open(QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn open(&mut self, receiver: *mut ::qt_core::object::Object, member: *const ::libc::c_char) {
    ::ffi::qt_widgets_c_QColorDialog_open(self as *mut ::color_dialog::ColorDialog, receiver, member)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QColorDialog::ColorDialogOption> QColorDialog::options() const```</span>
  ///
  ///
  pub fn options(&self) -> ::qt_core::flags::Flags<::color_dialog::ColorDialogOption> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_options(self as *const ::color_dialog::ColorDialog) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual int QColorDialog::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QColorDialog_qt_metacall(self as *mut ::color_dialog::ColorDialog,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QColorDialog::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QColorDialog_qt_metacast(self as *mut ::color_dialog::ColorDialog, arg1)
  }

  /// C++ method: <span style='color: green;'>```QColor QColorDialog::selectedColor() const```</span>
  ///
  ///
  pub fn selected_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_selectedColor_to_output(self as *const ::color_dialog::ColorDialog,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QColorDialog::setCurrentColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_current_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QColorDialog_setCurrentColor(self as *mut ::color_dialog::ColorDialog,
                                                       color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QColorDialog::setCustomColor(int index, QColor color)```</span>
  ///
  ///
  pub fn set_custom_color(index: ::libc::c_int, color: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_widgets_c_QColorDialog_setCustomColor(index, color as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QColorDialog::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::color_dialog::ColorDialogOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColorDialog::setOption(QColorDialog::ColorDialogOption option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::color_dialog::ColorDialogOption, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColorDialog::setOption(QColorDialog::ColorDialogOption option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorDialogSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QColorDialog::setOptions(QFlags<QColorDialog::ColorDialogOption> options)```</span>
  ///
  ///
  pub fn set_options(&mut self, options: ::qt_core::flags::Flags<::color_dialog::ColorDialogOption>) {
    unsafe {
      ::ffi::qt_widgets_c_QColorDialog_setOptions(self as *mut ::color_dialog::ColorDialog,
                                                  options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QColorDialog::setStandardColor(int index, QColor color)```</span>
  ///
  ///
  pub fn set_standard_color(index: ::libc::c_int, color: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_widgets_c_QColorDialog_setStandardColor(index, color as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QColorDialog::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QColorDialog_setVisible(self as *mut ::color_dialog::ColorDialog, visible) }
  }

  /// C++ method: <span style='color: green;'>```static QColor QColorDialog::standardColor(int index)```</span>
  ///
  ///
  pub fn standard_color(index: ::libc::c_int) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_standardColor_to_output(index, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QColorDialog::testOption(QColorDialog::ColorDialogOption option) const```</span>
  ///
  ///
  pub fn test_option(&self, option: ::color_dialog::ColorDialogOption) -> bool {
    unsafe { ::ffi::qt_widgets_c_QColorDialog_testOption(self as *const ::color_dialog::ColorDialog, option) }
  }

  /// C++ method: <span style='color: green;'>```static QString QColorDialog::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QColorDialog_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QColorDialog::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QColorDialog_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::color_dialog::ColorDialog {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QColorDialog_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ColorDialog`.
  pub struct Signals<'a>(&'a ::color_dialog::ColorDialog);
  /// Represents a built-in Qt signal `QColorDialog::accepted`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct Accepted<'a>(&'a ::color_dialog::ColorDialog);
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
  /// Represents a built-in Qt signal `QColorDialog::finished`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct Finished<'a>(&'a ::color_dialog::ColorDialog);
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
  /// Represents a built-in Qt signal `QColorDialog::colorSelected`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.signals().color_selected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct ColorSelected<'a>(&'a ::color_dialog::ColorDialog);
  impl<'a> ::qt_core::connection::Receiver for ColorSelected<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2colorSelected(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColorSelected<'a> {}
  /// Represents a built-in Qt signal `QColorDialog::currentColorChanged`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.signals().current_color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct CurrentColorChanged<'a>(&'a ::color_dialog::ColorDialog);
  impl<'a> ::qt_core::connection::Receiver for CurrentColorChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentColorChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentColorChanged<'a> {}
  /// Represents a built-in Qt signal `QColorDialog::rejected`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct Rejected<'a>(&'a ::color_dialog::ColorDialog);
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
    /// Returns an object representing a built-in Qt signal `QColorDialog::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColorDialog::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColorDialog::colorSelected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_selected(&self) -> ColorSelected {
      ColorSelected(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColorDialog::currentColorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_color_changed(&self) -> CurrentColorChanged {
      CurrentColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QColorDialog::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ColorDialog`.
  pub struct Slots<'a>(&'a ::color_dialog::ColorDialog);
  /// Represents a built-in Qt slot `QColorDialog::open`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct Open<'a>(&'a ::color_dialog::ColorDialog);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  /// Represents a built-in Qt slot `QColorDialog::reject`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct Reject<'a>(&'a ::color_dialog::ColorDialog);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QColorDialog::accept`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct Accept<'a>(&'a ::color_dialog::ColorDialog);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  /// Represents a built-in Qt slot `QColorDialog::showExtension`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct ShowExtension<'a>(&'a ::color_dialog::ColorDialog);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QColorDialog::exec`.
  ///
  /// An object of this type can be created from `ColorDialog` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ColorDialog` object.
  pub struct Exec<'a>(&'a ::color_dialog::ColorDialog);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QColorDialog::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColorDialog::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColorDialog::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColorDialog::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QColorDialog::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
  }
  impl ::color_dialog::ColorDialog {
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

/// C++ type: <span style='color: green;'>```QColorDialog::ColorDialogOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ColorDialogOption {
  /// C++ enum variant: <span style='color: green;'>```ShowAlphaChannel = 1```</span>
  ShowAlphaChannel = 1,
  /// C++ enum variant: <span style='color: green;'>```NoButtons = 2```</span>
  NoButtons = 2,
  /// C++ enum variant: <span style='color: green;'>```DontUseNativeDialog = 4```</span>
  DontUseNativeDialog = 4,
}

impl ::qt_core::flags::FlaggableEnum for ColorDialogOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ColorDialogOption"
  }
}

impl ::cpp_utils::DynamicCast<::color_dialog::ColorDialog> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::color_dialog::ColorDialog> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QDialog(self as *mut ::dialog::Dialog)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::color_dialog::ColorDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::color_dialog::ColorDialog> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::color_dialog::ColorDialog> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::color_dialog::ColorDialog> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_G_dynamic_cast_QColorDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::color_dialog::ColorDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QObject_ptr(self as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QObject_ptr(self as *const ::color_dialog::ColorDialog as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::color_dialog::ColorDialog {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QPaintDevice_ptr(self as *mut ::color_dialog::ColorDialog)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QPaintDevice_ptr(self as *const ::color_dialog::ColorDialog as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::color_dialog::ColorDialog {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QDialog_ptr(self as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QDialog_ptr(self as *const ::color_dialog::ColorDialog as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::color_dialog::ColorDialog {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QWidget_ptr(self as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QWidget_ptr(self as *const ::color_dialog::ColorDialog as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::color_dialog::ColorDialog> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::color_dialog::ColorDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::color_dialog::ColorDialog {
    let ffi_result = ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::color_dialog::ColorDialog> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::color_dialog::ColorDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::color_dialog::ColorDialog {
    let ffi_result = ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::color_dialog::ColorDialog> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::color_dialog::ColorDialog {
    let ffi_result = ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::color_dialog::ColorDialog {
    let ffi_result = ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::color_dialog::ColorDialog> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::color_dialog::ColorDialog {
    let ffi_result =
      ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::color_dialog::ColorDialog {
    let ffi_result = ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QColorDialog_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::color_dialog::ColorDialog {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QDialog_ptr(self as *const ::color_dialog::ColorDialog as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::color_dialog::ColorDialog {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QColorDialog_G_static_cast_QDialog_ptr(self as *mut ::color_dialog::ColorDialog) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ColorDialog::get_color](../struct.ColorDialog.html#method.get_color) method.
  pub trait ColorDialogGetColorArgs {
    fn exec(self) -> ::qt_gui::color::Color;
  }
  impl<'a> ColorDialogGetColorArgs for &'a ::qt_gui::color::Color {
    fn exec(self) -> ::qt_gui::color::Color {
      let initial = self;
      {
        let mut object: ::qt_gui::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QColorDialog_getColor_to_output_initial(initial as *const ::qt_gui::color::Color,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl ColorDialogGetColorArgs for () {
    fn exec(self) -> ::qt_gui::color::Color {

      {
        let mut object: ::qt_gui::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QColorDialog_getColor_to_output_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ColorDialog::get_color_unsafe](../struct.ColorDialog.html#method.get_color_unsafe) method.
  pub trait ColorDialogGetColorUnsafeArgs {
    unsafe fn exec(self) -> ::qt_gui::color::Color;
  }
  impl<'a> ColorDialogGetColorUnsafeArgs for (&'a ::qt_gui::color::Color, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::qt_gui::color::Color {
      let initial = self.0;
      let parent = self.1;
      {
        let mut object: ::qt_gui::color::Color =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QColorDialog_getColor_to_output_initial_parent(initial as *const ::qt_gui::color::Color,
                                                                           parent,
                                                                           &mut object);
        object
      }
    }
  }
  impl<'a> ColorDialogGetColorUnsafeArgs
    for (&'a ::qt_gui::color::Color, *mut ::widget::Widget, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::qt_gui::color::Color {
      let initial = self.0;
      let parent = self.1;
      let title = self.2;
      {
        let mut object: ::qt_gui::color::Color =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QColorDialog_getColor_to_output_initial_parent_title(initial as *const ::qt_gui::color::Color, parent, title as *const ::qt_core::string::String, &mut object);
        object
      }
    }
  }
  impl<'a> ColorDialogGetColorUnsafeArgs
    for (&'a ::qt_gui::color::Color,
                                                  *mut ::widget::Widget,
                                                  &'a ::qt_core::string::String,
                                                  ::qt_core::flags::Flags<::color_dialog::ColorDialogOption>) {
    unsafe fn exec(self) -> ::qt_gui::color::Color {
      let initial = self.0;
      let parent = self.1;
      let title = self.2;
      let options = self.3;
      {
        let mut object: ::qt_gui::color::Color =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_widgets_c_QColorDialog_getColor_to_output_initial_parent_title_options(initial as *const ::qt_gui::color::Color, parent, title as *const ::qt_core::string::String, options.to_int() as ::libc::c_uint, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ColorDialog::get_rgba](../struct.ColorDialog.html#method.get_rgba) method.
  pub trait ColorDialogGetRgbaArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl ColorDialogGetRgbaArgs for () {
    fn exec(self) -> ::libc::c_uint {

      unsafe { ::ffi::qt_widgets_c_QColorDialog_getRgba_no_args() }
    }
  }
  impl ColorDialogGetRgbaArgs for ::libc::c_uint {
    fn exec(self) -> ::libc::c_uint {
      let rgba = self;
      unsafe { ::ffi::qt_widgets_c_QColorDialog_getRgba_rgba(rgba) }
    }
  }
  /// This trait represents a set of arguments accepted by [ColorDialog::get_rgba_unsafe](../struct.ColorDialog.html#method.get_rgba_unsafe) method.
  pub trait ColorDialogGetRgbaUnsafeArgs {
    unsafe fn exec(self) -> ::libc::c_uint;
  }
  impl ColorDialogGetRgbaUnsafeArgs for (::libc::c_uint, *mut bool) {
    unsafe fn exec(self) -> ::libc::c_uint {
      let rgba = self.0;
      let ok = self.1;
      ::ffi::qt_widgets_c_QColorDialog_getRgba_rgba_ok(rgba, ok)
    }
  }
  impl ColorDialogGetRgbaUnsafeArgs for (::libc::c_uint, *mut bool, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::libc::c_uint {
      let rgba = self.0;
      let ok = self.1;
      let parent = self.2;
      ::ffi::qt_widgets_c_QColorDialog_getRgba_rgba_ok_parent(rgba, ok, parent)
    }
  }
  /// This trait represents a set of arguments accepted by [ColorDialog::new](../struct.ColorDialog.html#method.new) method.
  pub trait ColorDialogNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>;
  }
  impl<'a> ColorDialogNewArgs for &'a ::qt_gui::color::Color {
    fn exec(self) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog> {
      let initial = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QColorDialog_new_initial(initial as *const ::qt_gui::color::Color) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ColorDialogNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QColorDialog_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [ColorDialog::new_unsafe](../struct.ColorDialog.html#method.new_unsafe) method.
  pub trait ColorDialogNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog>;
  }
  impl<'a> ColorDialogNewUnsafeArgs for (&'a ::qt_gui::color::Color, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog> {
      let initial = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QColorDialog_new_initial_parent(initial as *const ::qt_gui::color::Color,
                                                                           parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ColorDialogNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::color_dialog::ColorDialog> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QColorDialog_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [ColorDialog::set_option](../struct.ColorDialog.html#method.set_option) method.
  pub trait ColorDialogSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color_dialog::ColorDialog) -> ();
  }
  impl<'largs> ColorDialogSetOptionArgs<'largs> for ::color_dialog::ColorDialogOption {
    fn exec(self, original_self: &'largs mut ::color_dialog::ColorDialog) -> () {
      let option = self;
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_setOption_option(original_self as *mut ::color_dialog::ColorDialog, option)
      }
    }
  }
  impl<'largs> ColorDialogSetOptionArgs<'largs> for (::color_dialog::ColorDialogOption, bool) {
    fn exec(self, original_self: &'largs mut ::color_dialog::ColorDialog) -> () {
      let option = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QColorDialog_setOption_option_on(original_self as *mut ::color_dialog::ColorDialog,
                                                             option,
                                                             on)
      }
    }
  }
}

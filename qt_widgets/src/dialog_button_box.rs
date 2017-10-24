/// C++ type: <span style='color: green;'>```QDialogButtonBox::ButtonLayout```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ButtonLayout {
  /// C++ enum variant: <span style='color: green;'>```WinLayout = 0```</span>
  Win = 0,
  /// C++ enum variant: <span style='color: green;'>```MacLayout = 1```</span>
  Mac = 1,
  /// C++ enum variant: <span style='color: green;'>```KdeLayout = 2```</span>
  Kde = 2,
  /// C++ enum variant: <span style='color: green;'>```GnomeLayout = 3```</span>
  Gnome = 3,
}

/// C++ type: <span style='color: green;'>```QDialogButtonBox::ButtonRole```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ButtonRole {
  /// C++ enum variant: <span style='color: green;'>```InvalidRole = -1```</span>
  InvalidRole = -1,
  /// C++ enum variant: <span style='color: green;'>```AcceptRole = 0```</span>
  AcceptRole = 0,
  /// C++ enum variant: <span style='color: green;'>```RejectRole = 1```</span>
  RejectRole = 1,
  /// C++ enum variant: <span style='color: green;'>```DestructiveRole = 2```</span>
  DestructiveRole = 2,
  /// C++ enum variant: <span style='color: green;'>```ActionRole = 3```</span>
  ActionRole = 3,
  /// C++ enum variant: <span style='color: green;'>```HelpRole = 4```</span>
  HelpRole = 4,
  /// C++ enum variant: <span style='color: green;'>```YesRole = 5```</span>
  YesRole = 5,
  /// C++ enum variant: <span style='color: green;'>```NoRole = 6```</span>
  NoRole = 6,
  /// C++ enum variant: <span style='color: green;'>```ResetRole = 7```</span>
  ResetRole = 7,
  /// C++ enum variant: <span style='color: green;'>```ApplyRole = 8```</span>
  ApplyRole = 8,
  /// C++ enum variant: <span style='color: green;'>```NRoles = 9```</span>
  NRoles = 9,
}

/// C++ type: <span style='color: green;'>```QDialogButtonBox```</span>
#[repr(C)]
pub struct DialogButtonBox(u8);

impl DialogButtonBox {
  /// C++ method: <span style='color: green;'>```QDialogButtonBox::addButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_button(&mut self, ::dialog_button_box::StandardButton) -> *mut ::push_button::PushButton```<br>
  /// C++ method: <span style='color: green;'>```QPushButton* QDialogButtonBox::addButton(QDialogButtonBox::StandardButton button)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_button(&mut self, (&::qt_core::string::String, ::dialog_button_box::ButtonRole)) -> *mut ::push_button::PushButton```<br>
  /// C++ method: <span style='color: green;'>```QPushButton* QDialogButtonBox::addButton(const QString& text, QDialogButtonBox::ButtonRole role)```</span>
  ///
  ///
  pub fn add_button<'largs, Args>(&'largs mut self, args: Args) -> *mut ::push_button::PushButton
    where Args: overloading::DialogButtonBoxAddButtonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QDialogButtonBox::addButton(QAbstractButton* button, QDialogButtonBox::ButtonRole role)```</span>
  ///
  ///
  pub unsafe fn add_button_unsafe(&mut self,
                                  button: *mut ::abstract_button::AbstractButton,
                                  role: ::dialog_button_box::ButtonRole) {
    ::ffi::qt_widgets_c_QDialogButtonBox_addButton_button_role(self as *mut ::dialog_button_box::DialogButtonBox,
                                                               button,
                                                               role)
  }

  /// C++ method: <span style='color: green;'>```QPushButton* QDialogButtonBox::button(QDialogButtonBox::StandardButton which) const```</span>
  ///
  ///
  pub fn button(&self, which: ::dialog_button_box::StandardButton) -> *mut ::push_button::PushButton {
    unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_button(self as *const ::dialog_button_box::DialogButtonBox, which) }
  }

  /// C++ method: <span style='color: green;'>```QDialogButtonBox::ButtonRole QDialogButtonBox::buttonRole(QAbstractButton* button) const```</span>
  ///
  ///
  pub unsafe fn button_role(&self, button: *mut ::abstract_button::AbstractButton) -> ::dialog_button_box::ButtonRole {
    ::ffi::qt_widgets_c_QDialogButtonBox_buttonRole(self as *const ::dialog_button_box::DialogButtonBox, button)
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*> QDialogButtonBox::buttons() const```</span>
  ///
  ///
  pub fn buttons(&self) -> ::list::ListAbstractButtonMutPtr {
    {
      let mut object: ::list::ListAbstractButtonMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDialogButtonBox_buttons_to_output(self as *const ::dialog_button_box::DialogButtonBox,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDialogButtonBox::centerButtons() const```</span>
  ///
  ///
  pub fn center_buttons(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_centerButtons(self as *const ::dialog_button_box::DialogButtonBox) }
  }

  /// C++ method: <span style='color: green;'>```void QDialogButtonBox::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_clear(self as *mut ::dialog_button_box::DialogButtonBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDialogButtonBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_metaObject(self as *const ::dialog_button_box::DialogButtonBox) }
  }

  /// C++ method: <span style='color: green;'>```QDialogButtonBox::QDialogButtonBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::qt_core::flags::Flags<::dialog_button_box::StandardButton>) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox(QFlags<QDialogButtonBox::StandardButton> buttons)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::qt_core::flags::Flags<::dialog_button_box::StandardButton>, &::qt_core::qt::Orientation)) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox(QFlags<QDialogButtonBox::StandardButton> buttons, Qt::Orientation orientation)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::Orientation) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>
    where Args: overloading::DialogButtonBoxNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDialogButtonBox::QDialogButtonBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((::qt_core::flags::Flags<::dialog_button_box::StandardButton>, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox(QFlags<QDialogButtonBox::StandardButton> buttons, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::qt_core::flags::Flags<::dialog_button_box::StandardButton>, &::qt_core::qt::Orientation, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox(QFlags<QDialogButtonBox::StandardButton> buttons, Qt::Orientation orientation, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::qt::Orientation, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDialogButtonBox::QDialogButtonBox(Qt::Orientation orientation, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>
    where Args: overloading::DialogButtonBoxNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QDialogButtonBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDialogButtonBox_qt_metacall(self as *mut ::dialog_button_box::DialogButtonBox,
                                                     arg1 as *const ::qt_core::meta_object::Call,
                                                     arg2,
                                                     arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDialogButtonBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDialogButtonBox_qt_metacast(self as *mut ::dialog_button_box::DialogButtonBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDialogButtonBox::removeButton(QAbstractButton* button)```</span>
  ///
  ///
  pub unsafe fn remove_button(&mut self, button: *mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QDialogButtonBox_removeButton(self as *mut ::dialog_button_box::DialogButtonBox, button)
  }

  /// C++ method: <span style='color: green;'>```void QDialogButtonBox::setCenterButtons(bool center)```</span>
  ///
  ///
  pub fn set_center_buttons(&mut self, center: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QDialogButtonBox_setCenterButtons(self as *mut ::dialog_button_box::DialogButtonBox, center)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDialogButtonBox::setOrientation(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, orientation: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QDialogButtonBox_setOrientation(self as *mut ::dialog_button_box::DialogButtonBox,
                                                          orientation as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDialogButtonBox::setStandardButtons(QFlags<QDialogButtonBox::StandardButton> buttons)```</span>
  ///
  ///
  pub fn set_standard_buttons(&mut self, buttons: ::qt_core::flags::Flags<::dialog_button_box::StandardButton>) {
    unsafe {
      ::ffi::qt_widgets_c_QDialogButtonBox_setStandardButtons(self as *mut ::dialog_button_box::DialogButtonBox,
                                                              buttons.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```QDialogButtonBox::StandardButton QDialogButtonBox::standardButton(QAbstractButton* button) const```</span>
  ///
  ///
  pub unsafe fn standard_button(&self,
                                button: *mut ::abstract_button::AbstractButton)
                                -> ::dialog_button_box::StandardButton {
    ::ffi::qt_widgets_c_QDialogButtonBox_standardButton(self as *const ::dialog_button_box::DialogButtonBox, button)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QDialogButtonBox::StandardButton> QDialogButtonBox::standardButtons() const```</span>
  ///
  ///
  pub fn standard_buttons(&self) -> ::qt_core::flags::Flags<::dialog_button_box::StandardButton> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDialogButtonBox_standardButtons(self as *const ::dialog_button_box::DialogButtonBox)
      };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```static QString QDialogButtonBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDialogButtonBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDialogButtonBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDialogButtonBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::dialog_button_box::DialogButtonBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDialogButtonBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DialogButtonBox`.
  pub struct Signals<'a>(&'a ::dialog_button_box::DialogButtonBox);
  /// Represents a built-in Qt signal `QDialogButtonBox::windowIconTextChanged`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().window_icon_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct WindowIconTextChanged<'a>(&'a ::dialog_button_box::DialogButtonBox);
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
  /// Represents a built-in Qt signal `QDialogButtonBox::clicked`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Clicked<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Clicked<'a> {
    type Arguments = (*mut ::abstract_button::AbstractButton,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clicked(QAbstractButton*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Clicked<'a> {}
  /// Represents a built-in Qt signal `QDialogButtonBox::windowIconChanged`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().window_icon_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct WindowIconChanged<'a>(&'a ::dialog_button_box::DialogButtonBox);
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
  /// Represents a built-in Qt signal `QDialogButtonBox::helpRequested`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().help_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct HelpRequested<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for HelpRequested<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2helpRequested()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HelpRequested<'a> {}
  /// Represents a built-in Qt signal `QDialogButtonBox::accepted`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Accepted<'a>(&'a ::dialog_button_box::DialogButtonBox);
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
  /// Represents a built-in Qt signal `QDialogButtonBox::customContextMenuRequested`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().custom_context_menu_requested()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct CustomContextMenuRequested<'a>(&'a ::dialog_button_box::DialogButtonBox);
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
  /// Represents a built-in Qt signal `QDialogButtonBox::windowTitleChanged`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().window_title_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct WindowTitleChanged<'a>(&'a ::dialog_button_box::DialogButtonBox);
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
  /// Represents a built-in Qt signal `QDialogButtonBox::rejected`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Rejected<'a>(&'a ::dialog_button_box::DialogButtonBox);
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
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::windowIconTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_text_changed(&self) -> WindowIconTextChanged {
      WindowIconTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::windowIconChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_icon_changed(&self) -> WindowIconChanged {
      WindowIconChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::helpRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn help_requested(&self) -> HelpRequested {
      HelpRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::customContextMenuRequested`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn custom_context_menu_requested(&self) -> CustomContextMenuRequested {
      CustomContextMenuRequested(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::windowTitleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn window_title_changed(&self) -> WindowTitleChanged {
      WindowTitleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDialogButtonBox::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DialogButtonBox`.
  pub struct Slots<'a>(&'a ::dialog_button_box::DialogButtonBox);
  /// Represents a built-in Qt slot `QDialogButtonBox::raise`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().raise()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Raise<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Raise<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1raise()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::hide`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().hide()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Hide<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Hide<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hide()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setEnabled`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetEnabled<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setHidden`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_hidden()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetHidden<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetHidden<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHidden(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::showNormal`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().show_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct ShowNormal<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for ShowNormal<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showNormal()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setDisabled`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_disabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetDisabled<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetDisabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDisabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setWindowModified`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_window_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetWindowModified<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetWindowModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::update`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Update<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::show`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().show()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Show<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Show<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1show()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::showFullScreen`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().show_full_screen()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct ShowFullScreen<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for ShowFullScreen<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showFullScreen()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::showMaximized`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().show_maximized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct ShowMaximized<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for ShowMaximized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMaximized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::repaint`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().repaint()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Repaint<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Repaint<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1repaint()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::updateMicroFocus`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().update_micro_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct UpdateMicroFocus<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for UpdateMicroFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1updateMicroFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setWindowTitle`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_window_title()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetWindowTitle<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetWindowTitle<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWindowTitle(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setStyleSheet`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_style_sheet()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetStyleSheet<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetStyleSheet<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStyleSheet(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setFocus`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetFocus<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::showMinimized`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().show_minimized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct ShowMinimized<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for ShowMinimized<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMinimized()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::close`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Close<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Close<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1close()\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::setVisible`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().set_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct SetVisible<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for SetVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDialogButtonBox::lower`.
  ///
  /// An object of this type can be created from `DialogButtonBox` with `object.slots().lower()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DialogButtonBox` object.
  pub struct Lower<'a>(&'a ::dialog_button_box::DialogButtonBox);
  impl<'a> ::qt_core::connection::Receiver for Lower<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1lower()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::raise`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn raise(&self) -> Raise {
      Raise(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::hide`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide(&self) -> Hide {
      Hide(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setHidden`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_hidden(&self) -> SetHidden {
      SetHidden(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::showNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_normal(&self) -> ShowNormal {
      ShowNormal(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setDisabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_disabled(&self) -> SetDisabled {
      SetDisabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setWindowModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_modified(&self) -> SetWindowModified {
      SetWindowModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::show`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show(&self) -> Show {
      Show(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::showFullScreen`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_full_screen(&self) -> ShowFullScreen {
      ShowFullScreen(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::showMaximized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_maximized(&self) -> ShowMaximized {
      ShowMaximized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::repaint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn repaint(&self) -> Repaint {
      Repaint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::updateMicroFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_micro_focus(&self) -> UpdateMicroFocus {
      UpdateMicroFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setWindowTitle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_window_title(&self) -> SetWindowTitle {
      SetWindowTitle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setStyleSheet`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_style_sheet(&self) -> SetStyleSheet {
      SetStyleSheet(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::showMinimized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_minimized(&self) -> ShowMinimized {
      ShowMinimized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::close`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn close(&self) -> Close {
      Close(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::setVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_visible(&self) -> SetVisible {
      SetVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDialogButtonBox::lower`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn lower(&self) -> Lower {
      Lower(self.0)
    }
  }
  impl ::dialog_button_box::DialogButtonBox {
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

/// C++ type: <span style='color: green;'>```QDialogButtonBox::StandardButton```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StandardButton {
  /// C++ enum variant: <span style='color: green;'>```NoButton = 0```</span>
  NoButton = 0,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Ok = 1024```</span>
  /// - <span style='color: green;'>```FirstButton = 1024```</span>
  ///
  Ok = 1024,
  /// C++ enum variant: <span style='color: green;'>```Save = 2048```</span>
  Save = 2048,
  /// C++ enum variant: <span style='color: green;'>```SaveAll = 4096```</span>
  SaveAll = 4096,
  /// C++ enum variant: <span style='color: green;'>```Open = 8192```</span>
  Open = 8192,
  /// C++ enum variant: <span style='color: green;'>```Yes = 16384```</span>
  Yes = 16384,
  /// C++ enum variant: <span style='color: green;'>```YesToAll = 32768```</span>
  YesToAll = 32768,
  /// C++ enum variant: <span style='color: green;'>```No = 65536```</span>
  No = 65536,
  /// C++ enum variant: <span style='color: green;'>```NoToAll = 131072```</span>
  NoToAll = 131072,
  /// C++ enum variant: <span style='color: green;'>```Abort = 262144```</span>
  Abort = 262144,
  /// C++ enum variant: <span style='color: green;'>```Retry = 524288```</span>
  Retry = 524288,
  /// C++ enum variant: <span style='color: green;'>```Ignore = 1048576```</span>
  Ignore = 1048576,
  /// C++ enum variant: <span style='color: green;'>```Close = 2097152```</span>
  Close = 2097152,
  /// C++ enum variant: <span style='color: green;'>```Cancel = 4194304```</span>
  Cancel = 4194304,
  /// C++ enum variant: <span style='color: green;'>```Discard = 8388608```</span>
  Discard = 8388608,
  /// C++ enum variant: <span style='color: green;'>```Help = 16777216```</span>
  Help = 16777216,
  /// C++ enum variant: <span style='color: green;'>```Apply = 33554432```</span>
  Apply = 33554432,
  /// C++ enum variant: <span style='color: green;'>```Reset = 67108864```</span>
  Reset = 67108864,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```RestoreDefaults = 134217728```</span>
  /// - <span style='color: green;'>```LastButton = 134217728```</span>
  ///
  RestoreDefaults = 134217728,
}

impl ::qt_core::flags::FlaggableEnum for StandardButton {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "StandardButton"
  }
}

impl ::cpp_utils::DynamicCast<::dialog_button_box::DialogButtonBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dialog_button_box::DialogButtonBox> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDialogButtonBox_G_dynamic_cast_QDialogButtonBox_ptr(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::dialog_button_box::DialogButtonBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_dynamic_cast_QDialogButtonBox_ptr(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::dialog_button_box::DialogButtonBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QObject_ptr(self as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QObject_ptr(self as *const ::dialog_button_box::DialogButtonBox as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::dialog_button_box::DialogButtonBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QPaintDevice_ptr(self as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QPaintDevice_ptr(self as *const ::dialog_button_box::DialogButtonBox as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::dialog_button_box::DialogButtonBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QWidget_ptr(self as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QWidget_ptr(self as *const ::dialog_button_box::DialogButtonBox as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dialog_button_box::DialogButtonBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dialog_button_box::DialogButtonBox {
    let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dialog_button_box::DialogButtonBox {
    let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dialog_button_box::DialogButtonBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dialog_button_box::DialogButtonBox {
    let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dialog_button_box::DialogButtonBox {
    let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dialog_button_box::DialogButtonBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dialog_button_box::DialogButtonBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dialog_button_box::DialogButtonBox {
    let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QDialogButtonBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::dialog_button_box::DialogButtonBox {
  type Target = ::widget::Widget;
  fn deref(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QWidget_ptr(self as *const ::dialog_button_box::DialogButtonBox as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dialog_button_box::DialogButtonBox {
  fn deref_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_G_static_cast_QWidget_ptr(self as *mut ::dialog_button_box::DialogButtonBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DialogButtonBox::add_button](../struct.DialogButtonBox.html#method.add_button) method.
  pub trait DialogButtonBoxAddButtonArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::dialog_button_box::DialogButtonBox) -> *mut ::push_button::PushButton;
  }
  impl<'largs> DialogButtonBoxAddButtonArgs<'largs> for ::dialog_button_box::StandardButton {
    fn exec(self, original_self: &'largs mut ::dialog_button_box::DialogButtonBox) -> *mut ::push_button::PushButton {
      let button = self;
      unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_addButton_button(original_self as *mut ::dialog_button_box::DialogButtonBox, button) }
    }
  }
  impl<'largs> DialogButtonBoxAddButtonArgs<'largs>
    for (&'largs ::qt_core::string::String, ::dialog_button_box::ButtonRole) {
    fn exec(self, original_self: &'largs mut ::dialog_button_box::DialogButtonBox) -> *mut ::push_button::PushButton {
      let text = self.0;
      let role = self.1;
      unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_addButton_text_role(original_self as *mut ::dialog_button_box::DialogButtonBox, text as *const ::qt_core::string::String, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [DialogButtonBox::new](../struct.DialogButtonBox.html#method.new) method.
  pub trait DialogButtonBoxNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>;
  }
  impl DialogButtonBoxNewArgs for ::qt_core::flags::Flags<::dialog_button_box::StandardButton> {
    fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {
      let buttons = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_new_buttons(buttons.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> DialogButtonBoxNewArgs
    for (::qt_core::flags::Flags<::dialog_button_box::StandardButton>, &'a ::qt_core::qt::Orientation) {
    fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {
      let buttons = self.0;
      let orientation = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QDialogButtonBox_new_buttons_orientation(buttons.to_int() as ::libc::c_uint, orientation as *const ::qt_core::qt::Orientation)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl DialogButtonBoxNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDialogButtonBox_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> DialogButtonBoxNewArgs for &'a ::qt_core::qt::Orientation {
    fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {
      let orientation = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QDialogButtonBox_new_orientation(orientation as *const ::qt_core::qt::Orientation)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [DialogButtonBox::new_unsafe](../struct.DialogButtonBox.html#method.new_unsafe) method.
  pub trait DialogButtonBoxNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox>;
  }
  impl<'a> DialogButtonBoxNewUnsafeArgs
    for (::qt_core::flags::Flags<::dialog_button_box::StandardButton>,
                                                 &'a ::qt_core::qt::Orientation,
                                                 *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {
      let buttons = self.0;
      let orientation = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_new_buttons_orientation_parent(buttons.to_int() as ::libc::c_uint, orientation as *const ::qt_core::qt::Orientation, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl DialogButtonBoxNewUnsafeArgs
    for (::qt_core::flags::Flags<::dialog_button_box::StandardButton>, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {
      let buttons = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_new_buttons_parent(buttons.to_int() as ::libc::c_uint,
                                                                               parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> DialogButtonBoxNewUnsafeArgs for (&'a ::qt_core::qt::Orientation, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {
      let orientation = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QDialogButtonBox_new_orientation_parent(orientation as *const ::qt_core::qt::Orientation,
                                                                    parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl DialogButtonBoxNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::dialog_button_box::DialogButtonBox> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QDialogButtonBox_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

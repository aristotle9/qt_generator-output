/// C++ type: <span style='color: green;'>```QToolButton```</span>
#[repr(C)]
pub struct ToolButton(u8);

impl ToolButton {
  /// C++ method: <span style='color: green;'>```bool QToolButton::autoRaise() const```</span>
  ///
  ///
  pub fn auto_raise(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QToolButton_autoRaise(self as *const ::tool_button::ToolButton) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QToolButton::defaultAction() const```</span>
  ///
  ///
  pub fn default_action(&self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QToolButton_defaultAction(self as *const ::tool_button::ToolButton) }
  }

  /// C++ method: <span style='color: green;'>```QMenu* QToolButton::menu() const```</span>
  ///
  ///
  pub fn menu(&self) -> *mut ::menu::Menu {
    unsafe { ::ffi::qt_widgets_c_QToolButton_menu(self as *const ::tool_button::ToolButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QToolButton::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QToolButton_metaObject(self as *const ::tool_button::ToolButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QToolButton::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolButton_minimumSizeHint_to_output(self as *const ::tool_button::ToolButton,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QToolButton::QToolButton()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::tool_button::ToolButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QToolButton::QToolButton(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::tool_button::ToolButton> {
    let ffi_result = ::ffi::qt_widgets_c_QToolButton_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QToolButton::ToolButtonPopupMode QToolButton::popupMode() const```</span>
  ///
  ///
  pub fn popup_mode(&self) -> ::tool_button::ToolButtonPopupMode {
    unsafe { ::ffi::qt_widgets_c_QToolButton_popupMode(self as *const ::tool_button::ToolButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QToolButton::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QToolButton_qt_metacall(self as *mut ::tool_button::ToolButton,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QToolButton::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QToolButton_qt_metacast(self as *mut ::tool_button::ToolButton, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QToolButton::setArrowType(Qt::ArrowType type)```</span>
  ///
  ///
  pub fn set_arrow_type(&mut self, type_: &::qt_core::qt::ArrowType) {
    unsafe {
      ::ffi::qt_widgets_c_QToolButton_setArrowType(self as *mut ::tool_button::ToolButton,
                                                   type_ as *const ::qt_core::qt::ArrowType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QToolButton::setAutoRaise(bool enable)```</span>
  ///
  ///
  pub fn set_auto_raise(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QToolButton_setAutoRaise(self as *mut ::tool_button::ToolButton, enable) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QToolButton::setDefaultAction(QAction* arg1)```</span>
  ///
  ///
  pub unsafe fn set_default_action(&mut self, arg1: *mut ::action::Action) {
    ::ffi::qt_widgets_c_QToolButton_setDefaultAction(self as *mut ::tool_button::ToolButton, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QToolButton::setMenu(QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn set_menu(&mut self, menu: *mut ::menu::Menu) {
    ::ffi::qt_widgets_c_QToolButton_setMenu(self as *mut ::tool_button::ToolButton, menu)
  }

  /// C++ method: <span style='color: green;'>```void QToolButton::setPopupMode(QToolButton::ToolButtonPopupMode mode)```</span>
  ///
  ///
  pub fn set_popup_mode(&mut self, mode: ::tool_button::ToolButtonPopupMode) {
    unsafe { ::ffi::qt_widgets_c_QToolButton_setPopupMode(self as *mut ::tool_button::ToolButton, mode) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QToolButton::setToolButtonStyle(Qt::ToolButtonStyle style)```</span>
  ///
  ///
  pub fn set_tool_button_style(&mut self, style: &::qt_core::qt::ToolButtonStyle) {
    unsafe {
      ::ffi::qt_widgets_c_QToolButton_setToolButtonStyle(self as *mut ::tool_button::ToolButton,
                                                         style as *const ::qt_core::qt::ToolButtonStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QToolButton::showMenu()```</span>
  ///
  ///
  pub fn show_menu(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QToolButton_showMenu(self as *mut ::tool_button::ToolButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QToolButton::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QToolButton_sizeHint_to_output(self as *const ::tool_button::ToolButton, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QToolButton::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QToolButton_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QToolButton::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QToolButton_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::tool_button::ToolButton {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QToolButton_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ToolButton`.
  pub struct Signals<'a>(&'a ::tool_button::ToolButton);
  /// Represents a built-in Qt signal `QToolButton::triggered`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct Triggered<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for Triggered<'a> {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2triggered(QAction*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Triggered<'a> {}
  /// Represents a built-in Qt signal `QToolButton::released`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.signals().released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct Released<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for Released<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2released()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Released<'a> {}
  /// Represents a built-in Qt signal `QToolButton::toggled`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.signals().toggled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct Toggled<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for Toggled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2toggled(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Toggled<'a> {}
  /// Represents a built-in Qt signal `QToolButton::pressed`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct Pressed<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for Pressed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Pressed<'a> {}
  /// Represents a built-in Qt signal `QToolButton::clicked`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct Clicked<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for Clicked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clicked(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Clicked<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QToolButton::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolButton::released`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn released(&self) -> Released {
      Released(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolButton::toggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggled(&self) -> Toggled {
      Toggled(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolButton::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QToolButton::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ToolButton`.
  pub struct Slots<'a>(&'a ::tool_button::ToolButton);
  /// Represents a built-in Qt slot `QToolButton::setDefaultAction`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().set_default_action()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct SetDefaultAction<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultAction<'a> {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultAction(QAction*)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolButton::setChecked`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().set_checked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct SetChecked<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for SetChecked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setChecked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolButton::toggle`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().toggle()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct Toggle<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for Toggle<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toggle()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolButton::click`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct Click<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for Click<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1click()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolButton::showMenu`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().show_menu()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct ShowMenu<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for ShowMenu<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMenu()\0"
    }
  }
  /// Represents a built-in Qt slot `QToolButton::animateClick`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().animate_click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct AnimateClick<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for AnimateClick<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1animateClick(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolButton::setIconSize`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().set_icon_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct SetIconSize<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for SetIconSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIconSize(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `QToolButton::setToolButtonStyle`.
  ///
  /// An object of this type can be created from `ToolButton` with `object.slots().set_tool_button_style()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ToolButton` object.
  pub struct SetToolButtonStyle<'a>(&'a ::tool_button::ToolButton);
  impl<'a> ::qt_core::connection::Receiver for SetToolButtonStyle<'a> {
    type Arguments = (&'static ::qt_core::qt::ToolButtonStyle,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setToolButtonStyle(Qt::ToolButtonStyle)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QToolButton::setDefaultAction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_action(&self) -> SetDefaultAction {
      SetDefaultAction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolButton::setChecked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_checked(&self) -> SetChecked {
      SetChecked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolButton::toggle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggle(&self) -> Toggle {
      Toggle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolButton::click`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn click(&self) -> Click {
      Click(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolButton::showMenu`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_menu(&self) -> ShowMenu {
      ShowMenu(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolButton::animateClick`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn animate_click(&self) -> AnimateClick {
      AnimateClick(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolButton::setIconSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_icon_size(&self) -> SetIconSize {
      SetIconSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QToolButton::setToolButtonStyle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_tool_button_style(&self) -> SetToolButtonStyle {
      SetToolButtonStyle(self.0)
    }
  }
  impl ::tool_button::ToolButton {
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

/// C++ type: <span style='color: green;'>```QToolButton::ToolButtonPopupMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ToolButtonPopupMode {
  /// C++ enum variant: <span style='color: green;'>```DelayedPopup = 0```</span>
  Delayed = 0,
  /// C++ enum variant: <span style='color: green;'>```MenuButtonPopup = 1```</span>
  MenuButton = 1,
  /// C++ enum variant: <span style='color: green;'>```InstantPopup = 2```</span>
  Instant = 2,
}

impl ::cpp_utils::DynamicCast<::tool_button::ToolButton> for ::abstract_button::AbstractButton {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tool_button::ToolButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tool_button::ToolButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tool_button::ToolButton> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tool_button::ToolButton> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tool_button::ToolButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_dynamic_cast_QToolButton_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tool_button::ToolButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QObject_ptr(self as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QObject_ptr(self as *const ::tool_button::ToolButton as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::tool_button::ToolButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QPaintDevice_ptr(self as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QPaintDevice_ptr(self as *const ::tool_button::ToolButton as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_button::AbstractButton> for ::tool_button::ToolButton {
  fn static_cast_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QToolButton_G_static_cast_QAbstractButton_ptr(self as *mut ::tool_button::ToolButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QAbstractButton_ptr(self as *const ::tool_button::ToolButton as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::tool_button::ToolButton {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QWidget_ptr(self as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QWidget_ptr(self as *const ::tool_button::ToolButton as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_button::ToolButton> for ::abstract_button::AbstractButton {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_button::ToolButton {
    let ffi_result = ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_button::ToolButton {
    let ffi_result = ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_button::ToolButton> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_button::ToolButton {
    let ffi_result =
      ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_button::ToolButton {
    let ffi_result = ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_button::ToolButton> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_button::ToolButton {
    let ffi_result = ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_button::ToolButton {
    let ffi_result = ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_button::ToolButton> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_button::ToolButton {
    let ffi_result =
      ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_button::ToolButton {
    let ffi_result = ::ffi::qt_widgets_c_QToolButton_G_static_cast_QToolButton_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tool_button::ToolButton {
  type Target = ::abstract_button::AbstractButton;
  fn deref(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QToolButton_G_static_cast_QAbstractButton_ptr(self as *const ::tool_button::ToolButton as *mut ::tool_button::ToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tool_button::ToolButton {
  fn deref_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QToolButton_G_static_cast_QAbstractButton_ptr(self as *mut ::tool_button::ToolButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

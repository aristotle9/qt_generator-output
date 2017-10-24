/// C++ type: <span style='color: green;'>```QPushButton```</span>
#[repr(C)]
pub struct PushButton(u8);

impl PushButton {
  /// C++ method: <span style='color: green;'>```bool QPushButton::autoDefault() const```</span>
  ///
  ///
  pub fn auto_default(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPushButton_autoDefault(self as *const ::push_button::PushButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QPushButton::isDefault() const```</span>
  ///
  ///
  pub fn is_default(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPushButton_isDefault(self as *const ::push_button::PushButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QPushButton::isFlat() const```</span>
  ///
  ///
  pub fn is_flat(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPushButton_isFlat(self as *const ::push_button::PushButton) }
  }

  /// C++ method: <span style='color: green;'>```QMenu* QPushButton::menu() const```</span>
  ///
  ///
  pub fn menu(&self) -> *mut ::menu::Menu {
    unsafe { ::ffi::qt_widgets_c_QPushButton_menu(self as *const ::push_button::PushButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPushButton::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QPushButton_metaObject(self as *const ::push_button::PushButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QPushButton::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPushButton_minimumSizeHint_to_output(self as *const ::push_button::PushButton,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPushButton::QPushButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::push_button::PushButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPushButton::QPushButton()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_gui::icon::Icon, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::push_button::PushButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPushButton::QPushButton(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::push_button::PushButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPushButton::QPushButton(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::push_button::PushButton>
    where Args: overloading::PushButtonNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPushButton::QPushButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::push_button::PushButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPushButton::QPushButton(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::icon::Icon, &::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::push_button::PushButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPushButton::QPushButton(const QIcon& icon, const QString& text, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::push_button::PushButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPushButton::QPushButton(const QString& text, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::push_button::PushButton>
    where Args: overloading::PushButtonNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QPushButton::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QPushButton_qt_metacall(self as *mut ::push_button::PushButton,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPushButton::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QPushButton_qt_metacast(self as *mut ::push_button::PushButton, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QPushButton::setAutoDefault(bool arg1)```</span>
  ///
  ///
  pub fn set_auto_default(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QPushButton_setAutoDefault(self as *mut ::push_button::PushButton, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QPushButton::setDefault(bool arg1)```</span>
  ///
  ///
  pub fn set_default(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QPushButton_setDefault(self as *mut ::push_button::PushButton, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QPushButton::setFlat(bool arg1)```</span>
  ///
  ///
  pub fn set_flat(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_widgets_c_QPushButton_setFlat(self as *mut ::push_button::PushButton, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QPushButton::setMenu(QMenu* menu)```</span>
  ///
  ///
  pub unsafe fn set_menu(&mut self, menu: *mut ::menu::Menu) {
    ::ffi::qt_widgets_c_QPushButton_setMenu(self as *mut ::push_button::PushButton, menu)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPushButton::showMenu()```</span>
  ///
  ///
  pub fn show_menu(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPushButton_showMenu(self as *mut ::push_button::PushButton) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QPushButton::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPushButton_sizeHint_to_output(self as *const ::push_button::PushButton, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPushButton::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPushButton_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPushButton::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPushButton_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::push_button::PushButton {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QPushButton_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PushButton`.
  pub struct Signals<'a>(&'a ::push_button::PushButton);
  /// Represents a built-in Qt signal `QPushButton::clicked`.
  ///
  /// An object of this type can be created from `PushButton` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct Clicked<'a>(&'a ::push_button::PushButton);
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
  /// Represents a built-in Qt signal `QPushButton::pressed`.
  ///
  /// An object of this type can be created from `PushButton` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct Pressed<'a>(&'a ::push_button::PushButton);
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
  /// Represents a built-in Qt signal `QPushButton::toggled`.
  ///
  /// An object of this type can be created from `PushButton` with `object.signals().toggled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct Toggled<'a>(&'a ::push_button::PushButton);
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
  /// Represents a built-in Qt signal `QPushButton::released`.
  ///
  /// An object of this type can be created from `PushButton` with `object.signals().released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct Released<'a>(&'a ::push_button::PushButton);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPushButton::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPushButton::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPushButton::toggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggled(&self) -> Toggled {
      Toggled(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPushButton::released`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn released(&self) -> Released {
      Released(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PushButton`.
  pub struct Slots<'a>(&'a ::push_button::PushButton);
  /// Represents a built-in Qt slot `QPushButton::showMenu`.
  ///
  /// An object of this type can be created from `PushButton` with `object.slots().show_menu()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct ShowMenu<'a>(&'a ::push_button::PushButton);
  impl<'a> ::qt_core::connection::Receiver for ShowMenu<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMenu()\0"
    }
  }
  /// Represents a built-in Qt slot `QPushButton::toggle`.
  ///
  /// An object of this type can be created from `PushButton` with `object.slots().toggle()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct Toggle<'a>(&'a ::push_button::PushButton);
  impl<'a> ::qt_core::connection::Receiver for Toggle<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toggle()\0"
    }
  }
  /// Represents a built-in Qt slot `QPushButton::animateClick`.
  ///
  /// An object of this type can be created from `PushButton` with `object.slots().animate_click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct AnimateClick<'a>(&'a ::push_button::PushButton);
  impl<'a> ::qt_core::connection::Receiver for AnimateClick<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1animateClick(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPushButton::click`.
  ///
  /// An object of this type can be created from `PushButton` with `object.slots().click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct Click<'a>(&'a ::push_button::PushButton);
  impl<'a> ::qt_core::connection::Receiver for Click<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1click()\0"
    }
  }
  /// Represents a built-in Qt slot `QPushButton::setIconSize`.
  ///
  /// An object of this type can be created from `PushButton` with `object.slots().set_icon_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct SetIconSize<'a>(&'a ::push_button::PushButton);
  impl<'a> ::qt_core::connection::Receiver for SetIconSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIconSize(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `QPushButton::setChecked`.
  ///
  /// An object of this type can be created from `PushButton` with `object.slots().set_checked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PushButton` object.
  pub struct SetChecked<'a>(&'a ::push_button::PushButton);
  impl<'a> ::qt_core::connection::Receiver for SetChecked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setChecked(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QPushButton::showMenu`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_menu(&self) -> ShowMenu {
      ShowMenu(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPushButton::toggle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggle(&self) -> Toggle {
      Toggle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPushButton::animateClick`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn animate_click(&self) -> AnimateClick {
      AnimateClick(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPushButton::click`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn click(&self) -> Click {
      Click(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPushButton::setIconSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_icon_size(&self) -> SetIconSize {
      SetIconSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPushButton::setChecked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_checked(&self) -> SetChecked {
      SetChecked(self.0)
    }
  }
  impl ::push_button::PushButton {
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

impl ::cpp_utils::DynamicCast<::push_button::PushButton> for ::abstract_button::AbstractButton {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::push_button::PushButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::push_button::PushButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::push_button::PushButton> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::push_button::PushButton> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::push_button::PushButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_dynamic_cast_QPushButton_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::push_button::PushButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QObject_ptr(self as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QObject_ptr(self as *const ::push_button::PushButton as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::push_button::PushButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPaintDevice_ptr(self as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPaintDevice_ptr(self as *const ::push_button::PushButton as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_button::AbstractButton> for ::push_button::PushButton {
  fn static_cast_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPushButton_G_static_cast_QAbstractButton_ptr(self as *mut ::push_button::PushButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QAbstractButton_ptr(self as *const ::push_button::PushButton as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::push_button::PushButton {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QWidget_ptr(self as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QWidget_ptr(self as *const ::push_button::PushButton as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::push_button::PushButton> for ::abstract_button::AbstractButton {
  unsafe fn static_cast_mut(&mut self) -> &mut ::push_button::PushButton {
    let ffi_result = ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::push_button::PushButton {
    let ffi_result = ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::push_button::PushButton> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::push_button::PushButton {
    let ffi_result =
      ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::push_button::PushButton {
    let ffi_result = ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::push_button::PushButton> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::push_button::PushButton {
    let ffi_result = ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::push_button::PushButton {
    let ffi_result = ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::push_button::PushButton> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::push_button::PushButton {
    let ffi_result =
      ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::push_button::PushButton {
    let ffi_result = ::ffi::qt_widgets_c_QPushButton_G_static_cast_QPushButton_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::push_button::PushButton {
  type Target = ::abstract_button::AbstractButton;
  fn deref(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_G_static_cast_QAbstractButton_ptr(self as *const ::push_button::PushButton as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::push_button::PushButton {
  fn deref_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPushButton_G_static_cast_QAbstractButton_ptr(self as *mut ::push_button::PushButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PushButton::new](../struct.PushButton.html#method.new) method.
  pub trait PushButtonNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton>;
  }
  impl<'a> PushButtonNewArgs for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton> {
      let icon = self.0;
      let text = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QPushButton_new_icon_text(icon as *const ::qt_gui::icon::Icon,
                                                      text as *const ::qt_core::string::String)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PushButtonNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PushButtonNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QPushButton_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [PushButton::new_unsafe](../struct.PushButton.html#method.new_unsafe) method.
  pub trait PushButtonNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton>;
  }
  impl<'a> PushButtonNewUnsafeArgs for (&'a ::qt_gui::icon::Icon, &'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton> {
      let icon = self.0;
      let text = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QPushButton_new_icon_text_parent(icon as *const ::qt_gui::icon::Icon,
                                                                            text as *const ::qt_core::string::String,
                                                                            parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl PushButtonNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QPushButton_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> PushButtonNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::push_button::PushButton> {
      let text = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QPushButton_new_text_parent(text as *const ::qt_core::string::String,
                                                                       parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

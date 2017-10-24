/// C++ type: <span style='color: green;'>```QCheckBox```</span>
#[repr(C)]
pub struct CheckBox(u8);

impl CheckBox {
  /// C++ method: <span style='color: green;'>```bool QCheckBox::isTristate() const```</span>
  ///
  ///
  pub fn is_tristate(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QCheckBox_isTristate(self as *const ::check_box::CheckBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QCheckBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QCheckBox_metaObject(self as *const ::check_box::CheckBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QCheckBox::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCheckBox_minimumSizeHint_to_output(self as *const ::check_box::CheckBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCheckBox::QCheckBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::check_box::CheckBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCheckBox::QCheckBox()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::check_box::CheckBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCheckBox::QCheckBox(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::check_box::CheckBox>
    where Args: overloading::CheckBoxNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QCheckBox::QCheckBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::check_box::CheckBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCheckBox::QCheckBox(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::check_box::CheckBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCheckBox::QCheckBox(const QString& text, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::check_box::CheckBox>
    where Args: overloading::CheckBoxNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QCheckBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QCheckBox_qt_metacall(self as *mut ::check_box::CheckBox,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QCheckBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QCheckBox_qt_metacast(self as *mut ::check_box::CheckBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QCheckBox::setCheckState(Qt::CheckState state)```</span>
  ///
  ///
  pub fn set_check_state(&mut self, state: &::qt_core::qt::CheckState) {
    unsafe {
      ::ffi::qt_widgets_c_QCheckBox_setCheckState(self as *mut ::check_box::CheckBox,
                                                  state as *const ::qt_core::qt::CheckState)
    }
  }

  /// C++ method: <span style='color: green;'>```QCheckBox::setTristate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_tristate(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCheckBox::setTristate()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_tristate(&mut self, bool) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCheckBox::setTristate(bool y = ?)```</span>
  ///
  ///
  pub fn set_tristate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::CheckBoxSetTristateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QSize QCheckBox::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCheckBox_sizeHint_to_output(self as *const ::check_box::CheckBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCheckBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCheckBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCheckBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCheckBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::check_box::CheckBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QCheckBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CheckBox`.
  pub struct Signals<'a>(&'a ::check_box::CheckBox);
  /// Represents a built-in Qt signal `QCheckBox::pressed`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct Pressed<'a>(&'a ::check_box::CheckBox);
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
  /// Represents a built-in Qt signal `QCheckBox::clicked`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct Clicked<'a>(&'a ::check_box::CheckBox);
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
  /// Represents a built-in Qt signal `QCheckBox::released`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.signals().released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct Released<'a>(&'a ::check_box::CheckBox);
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
  /// Represents a built-in Qt signal `QCheckBox::stateChanged`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct StateChanged<'a>(&'a ::check_box::CheckBox);
  impl<'a> ::qt_core::connection::Receiver for StateChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stateChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StateChanged<'a> {}
  /// Represents a built-in Qt signal `QCheckBox::toggled`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.signals().toggled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct Toggled<'a>(&'a ::check_box::CheckBox);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QCheckBox::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCheckBox::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCheckBox::released`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn released(&self) -> Released {
      Released(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCheckBox::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QCheckBox::toggled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggled(&self) -> Toggled {
      Toggled(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CheckBox`.
  pub struct Slots<'a>(&'a ::check_box::CheckBox);
  /// Represents a built-in Qt slot `QCheckBox::click`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.slots().click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct Click<'a>(&'a ::check_box::CheckBox);
  impl<'a> ::qt_core::connection::Receiver for Click<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1click()\0"
    }
  }
  /// Represents a built-in Qt slot `QCheckBox::animateClick`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.slots().animate_click()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct AnimateClick<'a>(&'a ::check_box::CheckBox);
  impl<'a> ::qt_core::connection::Receiver for AnimateClick<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1animateClick(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QCheckBox::setIconSize`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.slots().set_icon_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct SetIconSize<'a>(&'a ::check_box::CheckBox);
  impl<'a> ::qt_core::connection::Receiver for SetIconSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIconSize(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `QCheckBox::setChecked`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.slots().set_checked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct SetChecked<'a>(&'a ::check_box::CheckBox);
  impl<'a> ::qt_core::connection::Receiver for SetChecked<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setChecked(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QCheckBox::toggle`.
  ///
  /// An object of this type can be created from `CheckBox` with `object.slots().toggle()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CheckBox` object.
  pub struct Toggle<'a>(&'a ::check_box::CheckBox);
  impl<'a> ::qt_core::connection::Receiver for Toggle<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toggle()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QCheckBox::click`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn click(&self) -> Click {
      Click(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCheckBox::animateClick`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn animate_click(&self) -> AnimateClick {
      AnimateClick(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCheckBox::setIconSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_icon_size(&self) -> SetIconSize {
      SetIconSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCheckBox::setChecked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_checked(&self) -> SetChecked {
      SetChecked(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QCheckBox::toggle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggle(&self) -> Toggle {
      Toggle(self.0)
    }
  }
  impl ::check_box::CheckBox {
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

impl ::cpp_utils::DynamicCast<::check_box::CheckBox> for ::abstract_button::AbstractButton {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::check_box::CheckBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::check_box::CheckBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::check_box::CheckBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::check_box::CheckBox> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::check_box::CheckBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_dynamic_cast_QCheckBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::check_box::CheckBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QObject_ptr(self as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QObject_ptr(self as *const ::check_box::CheckBox as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::check_box::CheckBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QPaintDevice_ptr(self as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QPaintDevice_ptr(self as *const ::check_box::CheckBox as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_button::AbstractButton> for ::check_box::CheckBox {
  fn static_cast_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QAbstractButton_ptr(self as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QAbstractButton_ptr(self as *const ::check_box::CheckBox as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::check_box::CheckBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QWidget_ptr(self as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QWidget_ptr(self as *const ::check_box::CheckBox as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::check_box::CheckBox> for ::abstract_button::AbstractButton {
  unsafe fn static_cast_mut(&mut self) -> &mut ::check_box::CheckBox {
    let ffi_result = ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::check_box::CheckBox {
    let ffi_result = ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::check_box::CheckBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::check_box::CheckBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::check_box::CheckBox {
    let ffi_result = ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::check_box::CheckBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::check_box::CheckBox {
    let ffi_result = ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::check_box::CheckBox {
    let ffi_result = ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::check_box::CheckBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::check_box::CheckBox {
    let ffi_result = ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::check_box::CheckBox {
    let ffi_result = ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QCheckBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::check_box::CheckBox {
  type Target = ::abstract_button::AbstractButton;
  fn deref(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QAbstractButton_ptr(self as *const ::check_box::CheckBox as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::check_box::CheckBox {
  fn deref_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QCheckBox_G_static_cast_QAbstractButton_ptr(self as *mut ::check_box::CheckBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CheckBox::new](../struct.CheckBox.html#method.new) method.
  pub trait CheckBoxNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::check_box::CheckBox>;
  }
  impl CheckBoxNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::check_box::CheckBox> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CheckBoxNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::check_box::CheckBox> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QCheckBox_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [CheckBox::new_unsafe](../struct.CheckBox.html#method.new_unsafe) method.
  pub trait CheckBoxNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::check_box::CheckBox>;
  }
  impl CheckBoxNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::check_box::CheckBox> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QCheckBox_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> CheckBoxNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::check_box::CheckBox> {
      let text = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QCheckBox_new_text_parent(text as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [CheckBox::set_tristate](../struct.CheckBox.html#method.set_tristate) method.
  pub trait CheckBoxSetTristateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::check_box::CheckBox) -> ();
  }
  impl<'largs> CheckBoxSetTristateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::check_box::CheckBox) -> () {

      unsafe { ::ffi::qt_widgets_c_QCheckBox_setTristate_no_args(original_self as *mut ::check_box::CheckBox) }
    }
  }
  impl<'largs> CheckBoxSetTristateArgs<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::check_box::CheckBox) -> () {
      let y = self;
      unsafe { ::ffi::qt_widgets_c_QCheckBox_setTristate_y(original_self as *mut ::check_box::CheckBox, y) }
    }
  }
}

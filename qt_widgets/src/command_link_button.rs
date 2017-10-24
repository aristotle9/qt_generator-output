/// C++ type: <span style='color: green;'>```QCommandLinkButton```</span>
#[repr(C)]
pub struct CommandLinkButton(u8);

impl CommandLinkButton {
  /// C++ method: <span style='color: green;'>```QString QCommandLinkButton::description() const```</span>
  ///
  ///
  pub fn description(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QCommandLinkButton_description_to_output(self as *const ::command_link_button::CommandLinkButton, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QCommandLinkButton::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_widgets_c_QCommandLinkButton_metaObject(self as *const ::command_link_button::CommandLinkButton)
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLinkButton::QCommandLinkButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLinkButton::QCommandLinkButton()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLinkButton::QCommandLinkButton(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLinkButton::QCommandLinkButton(const QString& text, const QString& description)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>
    where Args: overloading::CommandLinkButtonNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QCommandLinkButton::QCommandLinkButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLinkButton::QCommandLinkButton(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLinkButton::QCommandLinkButton(const QString& text, QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, &::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QCommandLinkButton::QCommandLinkButton(const QString& text, const QString& description, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>
    where Args: overloading::CommandLinkButtonNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QCommandLinkButton::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QCommandLinkButton_qt_metacall(self as *mut ::command_link_button::CommandLinkButton,
                                                       arg1 as *const ::qt_core::meta_object::Call,
                                                       arg2,
                                                       arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QCommandLinkButton::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QCommandLinkButton_qt_metacast(self as *mut ::command_link_button::CommandLinkButton, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QCommandLinkButton::setDescription(const QString& description)```</span>
  ///
  ///
  pub fn set_description(&mut self, description: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QCommandLinkButton_setDescription(self as *mut ::command_link_button::CommandLinkButton,
                                                            description as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCommandLinkButton::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCommandLinkButton_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QCommandLinkButton::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QCommandLinkButton_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::command_link_button::CommandLinkButton {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QCommandLinkButton_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt slots of `CommandLinkButton`.
  pub struct Slots<'a>(&'a ::command_link_button::CommandLinkButton);
  /// Represents a built-in Qt slot `QCommandLinkButton::showMenu`.
  ///
  /// An object of this type can be created from `CommandLinkButton` with `object.slots().show_menu()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CommandLinkButton` object.
  pub struct ShowMenu<'a>(&'a ::command_link_button::CommandLinkButton);
  impl<'a> ::qt_core::connection::Receiver for ShowMenu<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMenu()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QCommandLinkButton::showMenu`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_menu(&self) -> ShowMenu {
      ShowMenu(self.0)
    }
  }
  impl ::command_link_button::CommandLinkButton {
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::command_link_button::CommandLinkButton> for ::abstract_button::AbstractButton {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::command_link_button::CommandLinkButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::command_link_button::CommandLinkButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::command_link_button::CommandLinkButton> for ::push_button::PushButton {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::command_link_button::CommandLinkButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QPushButton(self as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::command_link_button::CommandLinkButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QPushButton(self as *const ::push_button::PushButton as *mut ::push_button::PushButton) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::command_link_button::CommandLinkButton> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::command_link_button::CommandLinkButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::command_link_button::CommandLinkButton> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_dynamic_cast_QCommandLinkButton_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::command_link_button::CommandLinkButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QObject_ptr(self as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QObject_ptr(self as *const ::command_link_button::CommandLinkButton as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::command_link_button::CommandLinkButton {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QPaintDevice_ptr(self as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QPaintDevice_ptr(self as *const ::command_link_button::CommandLinkButton as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_button::AbstractButton> for ::command_link_button::CommandLinkButton {
  fn static_cast_mut(&mut self) -> &mut ::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QAbstractButton_ptr(self as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QAbstractButton_ptr(self as *const ::command_link_button::CommandLinkButton as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::push_button::PushButton> for ::command_link_button::CommandLinkButton {
  fn static_cast_mut(&mut self) -> &mut ::push_button::PushButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QPushButton_ptr(self as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::push_button::PushButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QPushButton_ptr(self as *const ::command_link_button::CommandLinkButton as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::command_link_button::CommandLinkButton {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QWidget_ptr(self as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QWidget_ptr(self as *const ::command_link_button::CommandLinkButton as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::command_link_button::CommandLinkButton> for ::abstract_button::AbstractButton {
  unsafe fn static_cast_mut(&mut self) -> &mut ::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QAbstractButton(self as *mut ::abstract_button::AbstractButton);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QAbstractButton(self as *const ::abstract_button::AbstractButton as *mut ::abstract_button::AbstractButton);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::command_link_button::CommandLinkButton> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::command_link_button::CommandLinkButton> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::command_link_button::CommandLinkButton> for ::push_button::PushButton {
  unsafe fn static_cast_mut(&mut self) -> &mut ::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QPushButton(self as *mut ::push_button::PushButton);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QPushButton(self as *const ::push_button::PushButton as *mut ::push_button::PushButton);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::command_link_button::CommandLinkButton> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::command_link_button::CommandLinkButton {
    let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QCommandLinkButton_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::command_link_button::CommandLinkButton {
  type Target = ::push_button::PushButton;
  fn deref(&self) -> &::push_button::PushButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QPushButton_ptr(self as *const ::command_link_button::CommandLinkButton as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::command_link_button::CommandLinkButton {
  fn deref_mut(&mut self) -> &mut ::push_button::PushButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_G_static_cast_QPushButton_ptr(self as *mut ::command_link_button::CommandLinkButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CommandLinkButton::new](../struct.CommandLinkButton.html#method.new) method.
  pub trait CommandLinkButtonNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>;
  }
  impl CommandLinkButtonNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CommandLinkButtonNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton> {
      let text = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QCommandLinkButton_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> CommandLinkButtonNewArgs for (&'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton> {
      let text = self.0;
      let description = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QCommandLinkButton_new_text_description(text as *const ::qt_core::string::String,
                                                                      description as *const ::qt_core::string::String)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [CommandLinkButton::new_unsafe](../struct.CommandLinkButton.html#method.new_unsafe) method.
  pub trait CommandLinkButtonNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton>;
  }
  impl CommandLinkButtonNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> CommandLinkButtonNewUnsafeArgs
    for (&'a ::qt_core::string::String, &'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton> {
      let text = self.0;
      let description = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QCommandLinkButton_new_text_description_parent(text as *const ::qt_core::string::String, description as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> CommandLinkButtonNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::command_link_button::CommandLinkButton> {
      let text = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QCommandLinkButton_new_text_parent(text as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

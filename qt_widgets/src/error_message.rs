/// C++ type: <span style='color: green;'>```QErrorMessage```</span>
#[repr(C)]
pub struct ErrorMessage(u8);

impl ErrorMessage {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QErrorMessage::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QErrorMessage_metaObject(self as *const ::error_message::ErrorMessage) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QErrorMessage::QErrorMessage()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::error_message::ErrorMessage> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QErrorMessage::QErrorMessage(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::error_message::ErrorMessage> {
    let ffi_result = ::ffi::qt_widgets_c_QErrorMessage_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```static QErrorMessage* QErrorMessage::qtHandler()```</span>
  ///
  ///
  pub fn qt_handler() -> *mut ::error_message::ErrorMessage {
    unsafe { ::ffi::qt_widgets_c_QErrorMessage_qtHandler() }
  }

  /// C++ method: <span style='color: green;'>```virtual int QErrorMessage::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QErrorMessage_qt_metacall(self as *mut ::error_message::ErrorMessage,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QErrorMessage::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QErrorMessage_qt_metacast(self as *mut ::error_message::ErrorMessage, arg1)
  }

  /// C++ method: <span style='color: green;'>```QErrorMessage::showMessage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn show_message(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QErrorMessage::showMessage(const QString& message)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn show_message(&mut self, (&::qt_core::string::String, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QErrorMessage::showMessage(const QString& message, const QString& type)```</span>
  ///
  ///
  pub fn show_message<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ErrorMessageShowMessageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QErrorMessage::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QErrorMessage_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QErrorMessage::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QErrorMessage_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::error_message::ErrorMessage {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QErrorMessage_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ErrorMessage`.
  pub struct Signals<'a>(&'a ::error_message::ErrorMessage);
  /// Represents a built-in Qt signal `QErrorMessage::finished`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct Finished<'a>(&'a ::error_message::ErrorMessage);
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
  /// Represents a built-in Qt signal `QErrorMessage::accepted`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.signals().accepted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct Accepted<'a>(&'a ::error_message::ErrorMessage);
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
  /// Represents a built-in Qt signal `QErrorMessage::rejected`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.signals().rejected()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct Rejected<'a>(&'a ::error_message::ErrorMessage);
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
    /// Returns an object representing a built-in Qt signal `QErrorMessage::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QErrorMessage::accepted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accepted(&self) -> Accepted {
      Accepted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QErrorMessage::rejected`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rejected(&self) -> Rejected {
      Rejected(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ErrorMessage`.
  pub struct Slots<'a>(&'a ::error_message::ErrorMessage);
  /// Represents a built-in Qt slot `QErrorMessage::exec`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.slots().exec()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct Exec<'a>(&'a ::error_message::ErrorMessage);
  impl<'a> ::qt_core::connection::Receiver for Exec<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1exec()\0"
    }
  }
  /// Represents a built-in Qt slot `QErrorMessage::reject`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.slots().reject()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct Reject<'a>(&'a ::error_message::ErrorMessage);
  impl<'a> ::qt_core::connection::Receiver for Reject<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reject()\0"
    }
  }
  /// Represents a built-in Qt slot `QErrorMessage::open`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.slots().open()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct Open<'a>(&'a ::error_message::ErrorMessage);
  impl<'a> ::qt_core::connection::Receiver for Open<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1open()\0"
    }
  }
  /// Represents a built-in Qt slot `QErrorMessage::showMessage`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.slots().show_message_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct ShowMessageQtCoreStringRef<'a>(&'a ::error_message::ErrorMessage);
  impl<'a> ::qt_core::connection::Receiver for ShowMessageQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMessage(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QErrorMessage::showMessage`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.slots().show_message_qt_core_string_ref_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct ShowMessageQtCoreStringRefQtCoreStringRef<'a>(&'a ::error_message::ErrorMessage);
  impl<'a> ::qt_core::connection::Receiver for ShowMessageQtCoreStringRefQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String, &'static ::qt_core::string::String);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showMessage(const QString&,const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QErrorMessage::accept`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.slots().accept()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct Accept<'a>(&'a ::error_message::ErrorMessage);
  impl<'a> ::qt_core::connection::Receiver for Accept<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1accept()\0"
    }
  }
  /// Represents a built-in Qt slot `QErrorMessage::showExtension`.
  ///
  /// An object of this type can be created from `ErrorMessage` with `object.slots().show_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ErrorMessage` object.
  pub struct ShowExtension<'a>(&'a ::error_message::ErrorMessage);
  impl<'a> ::qt_core::connection::Receiver for ShowExtension<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showExtension(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QErrorMessage::exec`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exec(&self) -> Exec {
      Exec(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QErrorMessage::reject`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reject(&self) -> Reject {
      Reject(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QErrorMessage::open`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn open(&self) -> Open {
      Open(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QErrorMessage::showMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_message_qt_core_string_ref(&self) -> ShowMessageQtCoreStringRef {
      ShowMessageQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QErrorMessage::showMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_message_qt_core_string_ref_qt_core_string_ref(&self) -> ShowMessageQtCoreStringRefQtCoreStringRef {
      ShowMessageQtCoreStringRefQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QErrorMessage::accept`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn accept(&self) -> Accept {
      Accept(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QErrorMessage::showExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_extension(&self) -> ShowExtension {
      ShowExtension(self.0)
    }
  }
  impl ::error_message::ErrorMessage {
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

impl ::cpp_utils::DynamicCast<::error_message::ErrorMessage> for ::dialog::Dialog {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::error_message::ErrorMessage> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QDialog(self as *mut ::dialog::Dialog)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::error_message::ErrorMessage> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::error_message::ErrorMessage> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::error_message::ErrorMessage> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::error_message::ErrorMessage> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_G_dynamic_cast_QErrorMessage_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::error_message::ErrorMessage {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QObject_ptr(self as *mut ::error_message::ErrorMessage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QObject_ptr(self as *const ::error_message::ErrorMessage as *mut ::error_message::ErrorMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::error_message::ErrorMessage {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QPaintDevice_ptr(self as *mut ::error_message::ErrorMessage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QPaintDevice_ptr(self as *const ::error_message::ErrorMessage as *mut ::error_message::ErrorMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::dialog::Dialog> for ::error_message::ErrorMessage {
  fn static_cast_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QDialog_ptr(self as *mut ::error_message::ErrorMessage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QDialog_ptr(self as *const ::error_message::ErrorMessage as *mut ::error_message::ErrorMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::error_message::ErrorMessage {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QWidget_ptr(self as *mut ::error_message::ErrorMessage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QWidget_ptr(self as *const ::error_message::ErrorMessage as *mut ::error_message::ErrorMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::error_message::ErrorMessage> for ::dialog::Dialog {
  unsafe fn static_cast_mut(&mut self) -> &mut ::error_message::ErrorMessage {
    let ffi_result =
      ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QDialog(self as *mut ::dialog::Dialog);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::error_message::ErrorMessage {
    let ffi_result = ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QDialog(self as *const ::dialog::Dialog as *mut ::dialog::Dialog);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::error_message::ErrorMessage> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::error_message::ErrorMessage {
    let ffi_result = ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::error_message::ErrorMessage {
    let ffi_result = ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::error_message::ErrorMessage> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::error_message::ErrorMessage {
    let ffi_result = ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::error_message::ErrorMessage {
    let ffi_result = ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::error_message::ErrorMessage> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::error_message::ErrorMessage {
    let ffi_result =
      ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::error_message::ErrorMessage {
    let ffi_result = ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QErrorMessage_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::error_message::ErrorMessage {
  type Target = ::dialog::Dialog;
  fn deref(&self) -> &::dialog::Dialog {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QDialog_ptr(self as *const ::error_message::ErrorMessage as *mut ::error_message::ErrorMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::error_message::ErrorMessage {
  fn deref_mut(&mut self) -> &mut ::dialog::Dialog {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_G_static_cast_QDialog_ptr(self as *mut ::error_message::ErrorMessage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ErrorMessage::show_message](../struct.ErrorMessage.html#method.show_message) method.
  pub trait ErrorMessageShowMessageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::error_message::ErrorMessage) -> ();
  }
  impl<'largs> ErrorMessageShowMessageArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::error_message::ErrorMessage) -> () {
      let message = self;
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_showMessage_message(original_self as *mut ::error_message::ErrorMessage,
                                                              message as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> ErrorMessageShowMessageArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::error_message::ErrorMessage) -> () {
      let message = self.0;
      let type_ = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QErrorMessage_showMessage_message_type(original_self as *mut ::error_message::ErrorMessage, message as *const ::qt_core::string::String, type_ as *const ::qt_core::string::String)
      }
    }
  }
}

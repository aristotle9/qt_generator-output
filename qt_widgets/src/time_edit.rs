/// C++ type: <span style='color: green;'>```QTimeEdit```</span>
#[repr(C)]
pub struct TimeEdit(u8);

impl TimeEdit {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTimeEdit::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTimeEdit_metaObject(self as *const ::time_edit::TimeEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTimeEdit::QTimeEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeEdit::QTimeEdit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::time::Time) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeEdit::QTimeEdit(const QTime& time)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>
    where Args: overloading::TimeEditNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTimeEdit::QTimeEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeEdit::QTimeEdit(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::time::Time, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeEdit::QTimeEdit(const QTime& time, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>
    where Args: overloading::TimeEditNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QTimeEdit::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTimeEdit_qt_metacall(self as *mut ::time_edit::TimeEdit,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTimeEdit::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTimeEdit_qt_metacast(self as *mut ::time_edit::TimeEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QTimeEdit::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTimeEdit_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTimeEdit::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTimeEdit_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::time_edit::TimeEdit {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTimeEdit_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TimeEdit`.
  pub struct Signals<'a>(&'a ::time_edit::TimeEdit);
  /// Represents a built-in Qt signal `QTimeEdit::dateTimeChanged`.
  ///
  /// An object of this type can be created from `TimeEdit` with `object.signals().date_time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeEdit` object.
  pub struct DateTimeChanged<'a>(&'a ::time_edit::TimeEdit);
  impl<'a> ::qt_core::connection::Receiver for DateTimeChanged<'a> {
    type Arguments = (&'static ::qt_core::date_time::DateTime,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dateTimeChanged(const QDateTime&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DateTimeChanged<'a> {}
  /// Represents a built-in Qt signal `QTimeEdit::userTimeChanged`.
  ///
  /// An object of this type can be created from `TimeEdit` with `object.signals().user_time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeEdit` object.
  pub struct UserTimeChanged<'a>(&'a ::time_edit::TimeEdit);
  impl<'a> ::qt_core::connection::Receiver for UserTimeChanged<'a> {
    type Arguments = (&'static ::qt_core::time::Time,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2userTimeChanged(const QTime&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UserTimeChanged<'a> {}
  /// Represents a built-in Qt signal `QTimeEdit::dateChanged`.
  ///
  /// An object of this type can be created from `TimeEdit` with `object.signals().date_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeEdit` object.
  pub struct DateChanged<'a>(&'a ::time_edit::TimeEdit);
  impl<'a> ::qt_core::connection::Receiver for DateChanged<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dateChanged(const QDate&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DateChanged<'a> {}
  /// Represents a built-in Qt signal `QTimeEdit::timeChanged`.
  ///
  /// An object of this type can be created from `TimeEdit` with `object.signals().time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeEdit` object.
  pub struct TimeChanged<'a>(&'a ::time_edit::TimeEdit);
  impl<'a> ::qt_core::connection::Receiver for TimeChanged<'a> {
    type Arguments = (&'static ::qt_core::time::Time,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2timeChanged(const QTime&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TimeChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTimeEdit::dateTimeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn date_time_changed(&self) -> DateTimeChanged {
      DateTimeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimeEdit::userTimeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn user_time_changed(&self) -> UserTimeChanged {
      UserTimeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimeEdit::dateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn date_changed(&self) -> DateChanged {
      DateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimeEdit::timeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn time_changed(&self) -> TimeChanged {
      TimeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TimeEdit`.
  pub struct Slots<'a>(&'a ::time_edit::TimeEdit);
  /// Represents a built-in Qt slot `QTimeEdit::setTime`.
  ///
  /// An object of this type can be created from `TimeEdit` with `object.slots().set_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeEdit` object.
  pub struct SetTime<'a>(&'a ::time_edit::TimeEdit);
  impl<'a> ::qt_core::connection::Receiver for SetTime<'a> {
    type Arguments = (&'static ::qt_core::time::Time,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTime(const QTime&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTimeEdit::setDate`.
  ///
  /// An object of this type can be created from `TimeEdit` with `object.slots().set_date()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeEdit` object.
  pub struct SetDate<'a>(&'a ::time_edit::TimeEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDate<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDate(const QDate&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTimeEdit::setDateTime`.
  ///
  /// An object of this type can be created from `TimeEdit` with `object.slots().set_date_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeEdit` object.
  pub struct SetDateTime<'a>(&'a ::time_edit::TimeEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDateTime<'a> {
    type Arguments = (&'static ::qt_core::date_time::DateTime,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDateTime(const QDateTime&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTimeEdit::setTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_time(&self) -> SetTime {
      SetTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimeEdit::setDate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_date(&self) -> SetDate {
      SetDate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimeEdit::setDateTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_date_time(&self) -> SetDateTime {
      SetDateTime(self.0)
    }
  }
  impl ::time_edit::TimeEdit {
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

impl ::cpp_utils::DynamicCast<::time_edit::TimeEdit> for ::abstract_spin_box::AbstractSpinBox {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::time_edit::TimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::time_edit::TimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::time_edit::TimeEdit> for ::date_time_edit::DateTimeEdit {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::time_edit::TimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QDateTimeEdit(self as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::time_edit::TimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QDateTimeEdit(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::time_edit::TimeEdit> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::time_edit::TimeEdit> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::time_edit::TimeEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_dynamic_cast_QTimeEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::time_edit::TimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QObject_ptr(self as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QObject_ptr(self as *const ::time_edit::TimeEdit as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::time_edit::TimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QPaintDevice_ptr(self as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QPaintDevice_ptr(self as *const ::time_edit::TimeEdit as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_spin_box::AbstractSpinBox> for ::time_edit::TimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QAbstractSpinBox_ptr(self as *const ::time_edit::TimeEdit as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::date_time_edit::DateTimeEdit> for ::time_edit::TimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QDateTimeEdit_ptr(self as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QDateTimeEdit_ptr(self as *const ::time_edit::TimeEdit as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::time_edit::TimeEdit {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QWidget_ptr(self as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QWidget_ptr(self as *const ::time_edit::TimeEdit as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::time_edit::TimeEdit> for ::abstract_spin_box::AbstractSpinBox {
  unsafe fn static_cast_mut(&mut self) -> &mut ::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::time_edit::TimeEdit> for ::date_time_edit::DateTimeEdit {
  unsafe fn static_cast_mut(&mut self) -> &mut ::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QDateTimeEdit(self as *mut ::date_time_edit::DateTimeEdit);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QDateTimeEdit(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::time_edit::TimeEdit> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::time_edit::TimeEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::time_edit::TimeEdit> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::time_edit::TimeEdit> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::time_edit::TimeEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QTimeEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::time_edit::TimeEdit {
  type Target = ::date_time_edit::DateTimeEdit;
  fn deref(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QDateTimeEdit_ptr(self as *const ::time_edit::TimeEdit as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::time_edit::TimeEdit {
  fn deref_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTimeEdit_G_static_cast_QDateTimeEdit_ptr(self as *mut ::time_edit::TimeEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TimeEdit::new](../struct.TimeEdit.html#method.new) method.
  pub trait TimeEditNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>;
  }
  impl TimeEditNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::time_edit::TimeEdit> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TimeEditNewArgs for &'a ::qt_core::time::Time {
    fn exec(self) -> ::cpp_utils::CppBox<::time_edit::TimeEdit> {
      let time = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTimeEdit_new_time(time as *const ::qt_core::time::Time) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TimeEdit::new_unsafe](../struct.TimeEdit.html#method.new_unsafe) method.
  pub trait TimeEditNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::time_edit::TimeEdit>;
  }
  impl TimeEditNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::time_edit::TimeEdit> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TimeEditNewUnsafeArgs for (&'a ::qt_core::time::Time, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::time_edit::TimeEdit> {
      let time = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QTimeEdit_new_time_parent(time as *const ::qt_core::time::Time, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

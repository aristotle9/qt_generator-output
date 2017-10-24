/// C++ type: <span style='color: green;'>```QDateEdit```</span>
#[repr(C)]
pub struct DateEdit(u8);

impl DateEdit {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDateEdit::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDateEdit_metaObject(self as *const ::date_edit::DateEdit) }
  }

  /// C++ method: <span style='color: green;'>```QDateEdit::QDateEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::date_edit::DateEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateEdit::QDateEdit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::date::Date) -> ::cpp_utils::CppBox<::date_edit::DateEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateEdit::QDateEdit(const QDate& date)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::date_edit::DateEdit>
    where Args: overloading::DateEditNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDateEdit::QDateEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::date_edit::DateEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateEdit::QDateEdit(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::date::Date, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::date_edit::DateEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDateEdit::QDateEdit(const QDate& date, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::date_edit::DateEdit>
    where Args: overloading::DateEditNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QDateEdit::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDateEdit_qt_metacall(self as *mut ::date_edit::DateEdit,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDateEdit::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDateEdit_qt_metacast(self as *mut ::date_edit::DateEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QDateEdit::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDateEdit_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDateEdit::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDateEdit_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::date_edit::DateEdit {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDateEdit_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DateEdit`.
  pub struct Signals<'a>(&'a ::date_edit::DateEdit);
  /// Represents a built-in Qt signal `QDateEdit::userDateChanged`.
  ///
  /// An object of this type can be created from `DateEdit` with `object.signals().user_date_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateEdit` object.
  pub struct UserDateChanged<'a>(&'a ::date_edit::DateEdit);
  impl<'a> ::qt_core::connection::Receiver for UserDateChanged<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2userDateChanged(const QDate&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UserDateChanged<'a> {}
  /// Represents a built-in Qt signal `QDateEdit::timeChanged`.
  ///
  /// An object of this type can be created from `DateEdit` with `object.signals().time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateEdit` object.
  pub struct TimeChanged<'a>(&'a ::date_edit::DateEdit);
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
  /// Represents a built-in Qt signal `QDateEdit::dateChanged`.
  ///
  /// An object of this type can be created from `DateEdit` with `object.signals().date_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateEdit` object.
  pub struct DateChanged<'a>(&'a ::date_edit::DateEdit);
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
  /// Represents a built-in Qt signal `QDateEdit::dateTimeChanged`.
  ///
  /// An object of this type can be created from `DateEdit` with `object.signals().date_time_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateEdit` object.
  pub struct DateTimeChanged<'a>(&'a ::date_edit::DateEdit);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDateEdit::userDateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn user_date_changed(&self) -> UserDateChanged {
      UserDateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDateEdit::timeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn time_changed(&self) -> TimeChanged {
      TimeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDateEdit::dateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn date_changed(&self) -> DateChanged {
      DateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDateEdit::dateTimeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn date_time_changed(&self) -> DateTimeChanged {
      DateTimeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DateEdit`.
  pub struct Slots<'a>(&'a ::date_edit::DateEdit);
  /// Represents a built-in Qt slot `QDateEdit::setTime`.
  ///
  /// An object of this type can be created from `DateEdit` with `object.slots().set_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateEdit` object.
  pub struct SetTime<'a>(&'a ::date_edit::DateEdit);
  impl<'a> ::qt_core::connection::Receiver for SetTime<'a> {
    type Arguments = (&'static ::qt_core::time::Time,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTime(const QTime&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDateEdit::setDateTime`.
  ///
  /// An object of this type can be created from `DateEdit` with `object.slots().set_date_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateEdit` object.
  pub struct SetDateTime<'a>(&'a ::date_edit::DateEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDateTime<'a> {
    type Arguments = (&'static ::qt_core::date_time::DateTime,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDateTime(const QDateTime&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDateEdit::setDate`.
  ///
  /// An object of this type can be created from `DateEdit` with `object.slots().set_date()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DateEdit` object.
  pub struct SetDate<'a>(&'a ::date_edit::DateEdit);
  impl<'a> ::qt_core::connection::Receiver for SetDate<'a> {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDate(const QDate&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDateEdit::setTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_time(&self) -> SetTime {
      SetTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDateEdit::setDateTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_date_time(&self) -> SetDateTime {
      SetDateTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDateEdit::setDate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_date(&self) -> SetDate {
      SetDate(self.0)
    }
  }
  impl ::date_edit::DateEdit {
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

impl ::cpp_utils::DynamicCast<::date_edit::DateEdit> for ::abstract_spin_box::AbstractSpinBox {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::date_edit::DateEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::date_edit::DateEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::date_edit::DateEdit> for ::date_time_edit::DateTimeEdit {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::date_edit::DateEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QDateTimeEdit(self as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::date_edit::DateEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QDateTimeEdit(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::date_edit::DateEdit> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::date_edit::DateEdit> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::date_edit::DateEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_dynamic_cast_QDateEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::date_edit::DateEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QObject_ptr(self as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QObject_ptr(self as *const ::date_edit::DateEdit as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::date_edit::DateEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QPaintDevice_ptr(self as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QPaintDevice_ptr(self as *const ::date_edit::DateEdit as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_spin_box::AbstractSpinBox> for ::date_edit::DateEdit {
  fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QAbstractSpinBox_ptr(self as *const ::date_edit::DateEdit as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::date_time_edit::DateTimeEdit> for ::date_edit::DateEdit {
  fn static_cast_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateTimeEdit_ptr(self as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateTimeEdit_ptr(self as *const ::date_edit::DateEdit as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::date_edit::DateEdit {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QWidget_ptr(self as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QWidget_ptr(self as *const ::date_edit::DateEdit as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_edit::DateEdit> for ::abstract_spin_box::AbstractSpinBox {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_edit::DateEdit> for ::date_time_edit::DateTimeEdit {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QDateTimeEdit(self as *mut ::date_time_edit::DateTimeEdit);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QDateTimeEdit(self as *const ::date_time_edit::DateTimeEdit as *mut ::date_time_edit::DateTimeEdit);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_edit::DateEdit> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_edit::DateEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_edit::DateEdit> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::date_edit::DateEdit> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::date_edit::DateEdit {
    let ffi_result = ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::date_edit::DateEdit {
  type Target = ::date_time_edit::DateTimeEdit;
  fn deref(&self) -> &::date_time_edit::DateTimeEdit {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateTimeEdit_ptr(self as *const ::date_edit::DateEdit as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::date_edit::DateEdit {
  fn deref_mut(&mut self) -> &mut ::date_time_edit::DateTimeEdit {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDateEdit_G_static_cast_QDateTimeEdit_ptr(self as *mut ::date_edit::DateEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DateEdit::new](../struct.DateEdit.html#method.new) method.
  pub trait DateEditNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::date_edit::DateEdit>;
  }
  impl<'a> DateEditNewArgs for &'a ::qt_core::date::Date {
    fn exec(self) -> ::cpp_utils::CppBox<::date_edit::DateEdit> {
      let date = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_new_date(date as *const ::qt_core::date::Date) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl DateEditNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::date_edit::DateEdit> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QDateEdit_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [DateEdit::new_unsafe](../struct.DateEdit.html#method.new_unsafe) method.
  pub trait DateEditNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_edit::DateEdit>;
  }
  impl<'a> DateEditNewUnsafeArgs for (&'a ::qt_core::date::Date, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_edit::DateEdit> {
      let date = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QDateEdit_new_date_parent(date as *const ::qt_core::date::Date, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl DateEditNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::date_edit::DateEdit> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QDateEdit_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

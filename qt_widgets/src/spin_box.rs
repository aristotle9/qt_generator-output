/// C++ type: <span style='color: green;'>```QSpinBox```</span>
#[repr(C)]
pub struct SpinBox(u8);

impl SpinBox {
  /// C++ method: <span style='color: green;'>```QString QSpinBox::cleanText() const```</span>
  ///
  ///
  pub fn clean_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpinBox_cleanText_to_output(self as *const ::spin_box::SpinBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSpinBox::displayIntegerBase() const```</span>
  ///
  ///
  pub fn display_integer_base(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_displayIntegerBase(self as *const ::spin_box::SpinBox) }
  }

  /// C++ method: <span style='color: green;'>```int QSpinBox::maximum() const```</span>
  ///
  ///
  pub fn maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_maximum(self as *const ::spin_box::SpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSpinBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_metaObject(self as *const ::spin_box::SpinBox) }
  }

  /// C++ method: <span style='color: green;'>```int QSpinBox::minimum() const```</span>
  ///
  ///
  pub fn minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_minimum(self as *const ::spin_box::SpinBox) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSpinBox::QSpinBox()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::spin_box::SpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSpinBox::QSpinBox(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::spin_box::SpinBox> {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QString QSpinBox::prefix() const```</span>
  ///
  ///
  pub fn prefix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpinBox_prefix_to_output(self as *const ::spin_box::SpinBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QSpinBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSpinBox_qt_metacall(self as *mut ::spin_box::SpinBox,
                                             arg1 as *const ::qt_core::meta_object::Call,
                                             arg2,
                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSpinBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QSpinBox_qt_metacast(self as *mut ::spin_box::SpinBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSpinBox::setDisplayIntegerBase(int base)```</span>
  ///
  ///
  pub fn set_display_integer_base(&mut self, base: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_setDisplayIntegerBase(self as *mut ::spin_box::SpinBox, base) }
  }

  /// C++ method: <span style='color: green;'>```void QSpinBox::setMaximum(int max)```</span>
  ///
  ///
  pub fn set_maximum(&mut self, max: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_setMaximum(self as *mut ::spin_box::SpinBox, max) }
  }

  /// C++ method: <span style='color: green;'>```void QSpinBox::setMinimum(int min)```</span>
  ///
  ///
  pub fn set_minimum(&mut self, min: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_setMinimum(self as *mut ::spin_box::SpinBox, min) }
  }

  /// C++ method: <span style='color: green;'>```void QSpinBox::setPrefix(const QString& prefix)```</span>
  ///
  ///
  pub fn set_prefix(&mut self, prefix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QSpinBox_setPrefix(self as *mut ::spin_box::SpinBox,
                                             prefix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSpinBox::setRange(int min, int max)```</span>
  ///
  ///
  pub fn set_range(&mut self, min: ::libc::c_int, max: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_setRange(self as *mut ::spin_box::SpinBox, min, max) }
  }

  /// C++ method: <span style='color: green;'>```void QSpinBox::setSingleStep(int val)```</span>
  ///
  ///
  pub fn set_single_step(&mut self, val: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_setSingleStep(self as *mut ::spin_box::SpinBox, val) }
  }

  /// C++ method: <span style='color: green;'>```void QSpinBox::setSuffix(const QString& suffix)```</span>
  ///
  ///
  pub fn set_suffix(&mut self, suffix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QSpinBox_setSuffix(self as *mut ::spin_box::SpinBox,
                                             suffix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSpinBox::setValue(int val)```</span>
  ///
  ///
  pub fn set_value(&mut self, val: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_setValue(self as *mut ::spin_box::SpinBox, val) }
  }

  /// C++ method: <span style='color: green;'>```int QSpinBox::singleStep() const```</span>
  ///
  ///
  pub fn single_step(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_singleStep(self as *const ::spin_box::SpinBox) }
  }

  /// C++ method: <span style='color: green;'>```QString QSpinBox::suffix() const```</span>
  ///
  ///
  pub fn suffix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSpinBox_suffix_to_output(self as *const ::spin_box::SpinBox, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSpinBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSpinBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSpinBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSpinBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSpinBox::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSpinBox_value(self as *const ::spin_box::SpinBox) }
  }
}

impl ::cpp_utils::CppDeletable for ::spin_box::SpinBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSpinBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SpinBox`.
  pub struct Signals<'a>(&'a ::spin_box::SpinBox);
  /// Represents a built-in Qt signal `QSpinBox::editingFinished`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.signals().editing_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct EditingFinished<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for EditingFinished<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2editingFinished()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EditingFinished<'a> {}
  /// Represents a built-in Qt signal `QSpinBox::valueChanged`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.signals().value_changed_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct ValueChangedCInt<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for ValueChangedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChangedCInt<'a> {}
  /// Represents a built-in Qt signal `QSpinBox::valueChanged`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.signals().value_changed_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct ValueChangedQtCoreStringRef<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for ValueChangedQtCoreStringRef<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChangedQtCoreStringRef<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSpinBox::editingFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editing_finished(&self) -> EditingFinished {
      EditingFinished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSpinBox::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed_c_int(&self) -> ValueChangedCInt {
      ValueChangedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSpinBox::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed_qt_core_string_ref(&self) -> ValueChangedQtCoreStringRef {
      ValueChangedQtCoreStringRef(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SpinBox`.
  pub struct Slots<'a>(&'a ::spin_box::SpinBox);
  /// Represents a built-in Qt slot `QSpinBox::stepDown`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.slots().step_down()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct StepDown<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for StepDown<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepDown()\0"
    }
  }
  /// Represents a built-in Qt slot `QSpinBox::clear`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct Clear<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QSpinBox::selectAll`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct SelectAll<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QSpinBox::setValue`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct SetValue<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QSpinBox::stepUp`.
  ///
  /// An object of this type can be created from `SpinBox` with `object.slots().step_up()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpinBox` object.
  pub struct StepUp<'a>(&'a ::spin_box::SpinBox);
  impl<'a> ::qt_core::connection::Receiver for StepUp<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepUp()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSpinBox::stepDown`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_down(&self) -> StepDown {
      StepDown(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSpinBox::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSpinBox::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSpinBox::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSpinBox::stepUp`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_up(&self) -> StepUp {
      StepUp(self.0)
    }
  }
  impl ::spin_box::SpinBox {
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

impl ::cpp_utils::DynamicCast<::spin_box::SpinBox> for ::abstract_spin_box::AbstractSpinBox {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::spin_box::SpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::spin_box::SpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::spin_box::SpinBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::spin_box::SpinBox> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::spin_box::SpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_dynamic_cast_QSpinBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::spin_box::SpinBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QObject_ptr(self as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QObject_ptr(self as *const ::spin_box::SpinBox as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::spin_box::SpinBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QPaintDevice_ptr(self as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QPaintDevice_ptr(self as *const ::spin_box::SpinBox as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_spin_box::AbstractSpinBox> for ::spin_box::SpinBox {
  fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *const ::spin_box::SpinBox as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::spin_box::SpinBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QWidget_ptr(self as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QWidget_ptr(self as *const ::spin_box::SpinBox as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spin_box::SpinBox> for ::abstract_spin_box::AbstractSpinBox {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spin_box::SpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spin_box::SpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spin_box::SpinBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spin_box::SpinBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spin_box::SpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spin_box::SpinBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spin_box::SpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spin_box::SpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spin_box::SpinBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spin_box::SpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spin_box::SpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QSpinBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::spin_box::SpinBox {
  type Target = ::abstract_spin_box::AbstractSpinBox;
  fn deref(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *const ::spin_box::SpinBox as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::spin_box::SpinBox {
  fn deref_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::spin_box::SpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

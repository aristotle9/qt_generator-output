/// C++ type: <span style='color: green;'>```QDoubleSpinBox```</span>
#[repr(C)]
pub struct DoubleSpinBox(u8);

impl DoubleSpinBox {
  /// C++ method: <span style='color: green;'>```QString QDoubleSpinBox::cleanText() const```</span>
  ///
  ///
  pub fn clean_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_cleanText_to_output(self as *const ::double_spin_box::DoubleSpinBox,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QDoubleSpinBox::decimals() const```</span>
  ///
  ///
  pub fn decimals(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_decimals(self as *const ::double_spin_box::DoubleSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QDoubleSpinBox::fixup(QString& str) const```</span>
  ///
  ///
  pub fn fixup(&self, str: &mut ::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QDoubleSpinBox_fixup(self as *const ::double_spin_box::DoubleSpinBox,
                                               str as *mut ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```double QDoubleSpinBox::maximum() const```</span>
  ///
  ///
  pub fn maximum(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_maximum(self as *const ::double_spin_box::DoubleSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDoubleSpinBox::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_metaObject(self as *const ::double_spin_box::DoubleSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```double QDoubleSpinBox::minimum() const```</span>
  ///
  ///
  pub fn minimum(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_minimum(self as *const ::double_spin_box::DoubleSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDoubleSpinBox::QDoubleSpinBox()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::double_spin_box::DoubleSpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDoubleSpinBox::QDoubleSpinBox(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::double_spin_box::DoubleSpinBox> {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QString QDoubleSpinBox::prefix() const```</span>
  ///
  ///
  pub fn prefix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_prefix_to_output(self as *const ::double_spin_box::DoubleSpinBox,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDoubleSpinBox::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDoubleSpinBox_qt_metacall(self as *mut ::double_spin_box::DoubleSpinBox,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDoubleSpinBox::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDoubleSpinBox_qt_metacast(self as *mut ::double_spin_box::DoubleSpinBox, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDoubleSpinBox::setDecimals(int prec)```</span>
  ///
  ///
  pub fn set_decimals(&mut self, prec: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_setDecimals(self as *mut ::double_spin_box::DoubleSpinBox, prec) }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleSpinBox::setMaximum(double max)```</span>
  ///
  ///
  pub fn set_maximum(&mut self, max: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_setMaximum(self as *mut ::double_spin_box::DoubleSpinBox, max) }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleSpinBox::setMinimum(double min)```</span>
  ///
  ///
  pub fn set_minimum(&mut self, min: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_setMinimum(self as *mut ::double_spin_box::DoubleSpinBox, min) }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleSpinBox::setPrefix(const QString& prefix)```</span>
  ///
  ///
  pub fn set_prefix(&mut self, prefix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QDoubleSpinBox_setPrefix(self as *mut ::double_spin_box::DoubleSpinBox,
                                                   prefix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleSpinBox::setRange(double min, double max)```</span>
  ///
  ///
  pub fn set_range(&mut self, min: ::libc::c_double, max: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_setRange(self as *mut ::double_spin_box::DoubleSpinBox, min, max) }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleSpinBox::setSingleStep(double val)```</span>
  ///
  ///
  pub fn set_single_step(&mut self, val: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_setSingleStep(self as *mut ::double_spin_box::DoubleSpinBox, val) }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleSpinBox::setSuffix(const QString& suffix)```</span>
  ///
  ///
  pub fn set_suffix(&mut self, suffix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QDoubleSpinBox_setSuffix(self as *mut ::double_spin_box::DoubleSpinBox,
                                                   suffix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDoubleSpinBox::setValue(double val)```</span>
  ///
  ///
  pub fn set_value(&mut self, val: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_setValue(self as *mut ::double_spin_box::DoubleSpinBox, val) }
  }

  /// C++ method: <span style='color: green;'>```double QDoubleSpinBox::singleStep() const```</span>
  ///
  ///
  pub fn single_step(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_singleStep(self as *const ::double_spin_box::DoubleSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```QString QDoubleSpinBox::suffix() const```</span>
  ///
  ///
  pub fn suffix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_suffix_to_output(self as *const ::double_spin_box::DoubleSpinBox,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QDoubleSpinBox::textFromValue(double val) const```</span>
  ///
  ///
  pub fn text_from_value(&self, val: ::libc::c_double) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_textFromValue_to_output(self as *const ::double_spin_box::DoubleSpinBox,
                                                                   val,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDoubleSpinBox::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDoubleSpinBox_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDoubleSpinBox::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDoubleSpinBox_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QDoubleSpinBox::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_value(self as *const ::double_spin_box::DoubleSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```virtual double QDoubleSpinBox::valueFromText(const QString& text) const```</span>
  ///
  ///
  pub fn value_from_text(&self, text: &::qt_core::string::String) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QDoubleSpinBox_valueFromText(self as *const ::double_spin_box::DoubleSpinBox,
                                                       text as *const ::qt_core::string::String)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::double_spin_box::DoubleSpinBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDoubleSpinBox_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DoubleSpinBox`.
  pub struct Signals<'a>(&'a ::double_spin_box::DoubleSpinBox);
  /// Represents a built-in Qt signal `QDoubleSpinBox::valueChanged`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.signals().value_changed_c_double()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct ValueChangedCDouble<'a>(&'a ::double_spin_box::DoubleSpinBox);
  impl<'a> ::qt_core::connection::Receiver for ValueChangedCDouble<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChangedCDouble<'a> {}
  /// Represents a built-in Qt signal `QDoubleSpinBox::valueChanged`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.signals().value_changed_qt_core_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct ValueChangedQtCoreStringRef<'a>(&'a ::double_spin_box::DoubleSpinBox);
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
  /// Represents a built-in Qt signal `QDoubleSpinBox::editingFinished`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.signals().editing_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct EditingFinished<'a>(&'a ::double_spin_box::DoubleSpinBox);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDoubleSpinBox::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed_c_double(&self) -> ValueChangedCDouble {
      ValueChangedCDouble(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDoubleSpinBox::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed_qt_core_string_ref(&self) -> ValueChangedQtCoreStringRef {
      ValueChangedQtCoreStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDoubleSpinBox::editingFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn editing_finished(&self) -> EditingFinished {
      EditingFinished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DoubleSpinBox`.
  pub struct Slots<'a>(&'a ::double_spin_box::DoubleSpinBox);
  /// Represents a built-in Qt slot `QDoubleSpinBox::selectAll`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct SelectAll<'a>(&'a ::double_spin_box::DoubleSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QDoubleSpinBox::clear`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct Clear<'a>(&'a ::double_spin_box::DoubleSpinBox);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QDoubleSpinBox::stepDown`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.slots().step_down()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct StepDown<'a>(&'a ::double_spin_box::DoubleSpinBox);
  impl<'a> ::qt_core::connection::Receiver for StepDown<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepDown()\0"
    }
  }
  /// Represents a built-in Qt slot `QDoubleSpinBox::stepUp`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.slots().step_up()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct StepUp<'a>(&'a ::double_spin_box::DoubleSpinBox);
  impl<'a> ::qt_core::connection::Receiver for StepUp<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stepUp()\0"
    }
  }
  /// Represents a built-in Qt slot `QDoubleSpinBox::setValue`.
  ///
  /// An object of this type can be created from `DoubleSpinBox` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleSpinBox` object.
  pub struct SetValue<'a>(&'a ::double_spin_box::DoubleSpinBox);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(double)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDoubleSpinBox::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDoubleSpinBox::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDoubleSpinBox::stepDown`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_down(&self) -> StepDown {
      StepDown(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDoubleSpinBox::stepUp`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn step_up(&self) -> StepUp {
      StepUp(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDoubleSpinBox::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
  }
  impl ::double_spin_box::DoubleSpinBox {
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

impl ::cpp_utils::DynamicCast<::double_spin_box::DoubleSpinBox> for ::abstract_spin_box::AbstractSpinBox {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::double_spin_box::DoubleSpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::double_spin_box::DoubleSpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::double_spin_box::DoubleSpinBox> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::double_spin_box::DoubleSpinBox> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::double_spin_box::DoubleSpinBox> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_dynamic_cast_QDoubleSpinBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::double_spin_box::DoubleSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QObject_ptr(self as *mut ::double_spin_box::DoubleSpinBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QObject_ptr(self as *const ::double_spin_box::DoubleSpinBox as *mut ::double_spin_box::DoubleSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::double_spin_box::DoubleSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QPaintDevice_ptr(self as *mut ::double_spin_box::DoubleSpinBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QPaintDevice_ptr(self as *const ::double_spin_box::DoubleSpinBox as *mut ::double_spin_box::DoubleSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_spin_box::AbstractSpinBox> for ::double_spin_box::DoubleSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::double_spin_box::DoubleSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *const ::double_spin_box::DoubleSpinBox as *mut ::double_spin_box::DoubleSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::double_spin_box::DoubleSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QWidget_ptr(self as *mut ::double_spin_box::DoubleSpinBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QWidget_ptr(self as *const ::double_spin_box::DoubleSpinBox as *mut ::double_spin_box::DoubleSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::double_spin_box::DoubleSpinBox> for ::abstract_spin_box::AbstractSpinBox {
  unsafe fn static_cast_mut(&mut self) -> &mut ::double_spin_box::DoubleSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(self as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::double_spin_box::DoubleSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QAbstractSpinBox(self as *const ::abstract_spin_box::AbstractSpinBox as *mut ::abstract_spin_box::AbstractSpinBox);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::double_spin_box::DoubleSpinBox> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::double_spin_box::DoubleSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::double_spin_box::DoubleSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::double_spin_box::DoubleSpinBox> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::double_spin_box::DoubleSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::double_spin_box::DoubleSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::double_spin_box::DoubleSpinBox> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::double_spin_box::DoubleSpinBox {
    let ffi_result =
      ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::double_spin_box::DoubleSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QDoubleSpinBox_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::double_spin_box::DoubleSpinBox {
  type Target = ::abstract_spin_box::AbstractSpinBox;
  fn deref(&self) -> &::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *const ::double_spin_box::DoubleSpinBox as *mut ::double_spin_box::DoubleSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::double_spin_box::DoubleSpinBox {
  fn deref_mut(&mut self) -> &mut ::abstract_spin_box::AbstractSpinBox {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDoubleSpinBox_G_static_cast_QAbstractSpinBox_ptr(self as *mut ::double_spin_box::DoubleSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

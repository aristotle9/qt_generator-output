/// C++ type: <span style='color: green;'>```QDial```</span>
#[repr(C)]
pub struct Dial(u8);

impl Dial {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDial_metaObject(self as *const ::dial::Dial) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QDial::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDial_minimumSizeHint_to_output(self as *const ::dial::Dial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDial::QDial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::dial::Dial> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDial::QDial(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::dial::Dial> {
    let ffi_result = ::ffi::qt_widgets_c_QDial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```int QDial::notchSize() const```</span>
  ///
  ///
  pub fn notch_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QDial_notchSize(self as *const ::dial::Dial) }
  }

  /// C++ method: <span style='color: green;'>```double QDial::notchTarget() const```</span>
  ///
  ///
  pub fn notch_target(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QDial_notchTarget(self as *const ::dial::Dial) }
  }

  /// C++ method: <span style='color: green;'>```bool QDial::notchesVisible() const```</span>
  ///
  ///
  pub fn notches_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDial_notchesVisible(self as *const ::dial::Dial) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDial_qt_metacall(self as *mut ::dial::Dial,
                                          arg1 as *const ::qt_core::meta_object::Call,
                                          arg2,
                                          arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDial_qt_metacast(self as *mut ::dial::Dial, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDial::setNotchTarget(double target)```</span>
  ///
  ///
  pub fn set_notch_target(&mut self, target: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QDial_setNotchTarget(self as *mut ::dial::Dial, target) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDial::setNotchesVisible(bool visible)```</span>
  ///
  ///
  pub fn set_notches_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_widgets_c_QDial_setNotchesVisible(self as *mut ::dial::Dial, visible) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QDial::setWrapping(bool on)```</span>
  ///
  ///
  pub fn set_wrapping(&mut self, on: bool) {
    unsafe { ::ffi::qt_widgets_c_QDial_setWrapping(self as *mut ::dial::Dial, on) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QDial::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDial_sizeHint_to_output(self as *const ::dial::Dial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDial::wrapping() const```</span>
  ///
  ///
  pub fn wrapping(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDial_wrapping(self as *const ::dial::Dial) }
  }
}

impl ::cpp_utils::CppDeletable for ::dial::Dial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Dial`.
  pub struct Signals<'a>(&'a ::dial::Dial);
  /// Represents a built-in Qt signal `QDial::sliderPressed`.
  ///
  /// An object of this type can be created from `Dial` with `object.signals().slider_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SliderPressed<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SliderPressed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sliderPressed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SliderPressed<'a> {}
  /// Represents a built-in Qt signal `QDial::valueChanged`.
  ///
  /// An object of this type can be created from `Dial` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct ValueChanged<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for ValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ValueChanged<'a> {}
  /// Represents a built-in Qt signal `QDial::rangeChanged`.
  ///
  /// An object of this type can be created from `Dial` with `object.signals().range_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct RangeChanged<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for RangeChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rangeChanged(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RangeChanged<'a> {}
  /// Represents a built-in Qt signal `QDial::actionTriggered`.
  ///
  /// An object of this type can be created from `Dial` with `object.signals().action_triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct ActionTriggered<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for ActionTriggered<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2actionTriggered(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActionTriggered<'a> {}
  /// Represents a built-in Qt signal `QDial::sliderReleased`.
  ///
  /// An object of this type can be created from `Dial` with `object.signals().slider_released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SliderReleased<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SliderReleased<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sliderReleased()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SliderReleased<'a> {}
  /// Represents a built-in Qt signal `QDial::sliderMoved`.
  ///
  /// An object of this type can be created from `Dial` with `object.signals().slider_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SliderMoved<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SliderMoved<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sliderMoved(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SliderMoved<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDial::sliderPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_pressed(&self) -> SliderPressed {
      SliderPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDial::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDial::rangeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn range_changed(&self) -> RangeChanged {
      RangeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDial::actionTriggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn action_triggered(&self) -> ActionTriggered {
      ActionTriggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDial::sliderReleased`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_released(&self) -> SliderReleased {
      SliderReleased(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDial::sliderMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_moved(&self) -> SliderMoved {
      SliderMoved(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Dial`.
  pub struct Slots<'a>(&'a ::dial::Dial);
  /// Represents a built-in Qt slot `QDial::setValue`.
  ///
  /// An object of this type can be created from `Dial` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SetValue<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QDial::setRange`.
  ///
  /// An object of this type can be created from `Dial` with `object.slots().set_range()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SetRange<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SetRange<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRange(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QDial::setNotchesVisible`.
  ///
  /// An object of this type can be created from `Dial` with `object.slots().set_notches_visible()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SetNotchesVisible<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SetNotchesVisible<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNotchesVisible(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QDial::setOrientation`.
  ///
  /// An object of this type can be created from `Dial` with `object.slots().set_orientation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SetOrientation<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SetOrientation<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOrientation(Qt::Orientation)\0"
    }
  }
  /// Represents a built-in Qt slot `QDial::setWrapping`.
  ///
  /// An object of this type can be created from `Dial` with `object.slots().set_wrapping()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Dial` object.
  pub struct SetWrapping<'a>(&'a ::dial::Dial);
  impl<'a> ::qt_core::connection::Receiver for SetWrapping<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWrapping(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QDial::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDial::setRange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_range(&self) -> SetRange {
      SetRange(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDial::setNotchesVisible`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_notches_visible(&self) -> SetNotchesVisible {
      SetNotchesVisible(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDial::setOrientation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_orientation(&self) -> SetOrientation {
      SetOrientation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDial::setWrapping`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_wrapping(&self) -> SetWrapping {
      SetWrapping(self.0)
    }
  }
  impl ::dial::Dial {
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

impl ::cpp_utils::DynamicCast<::dial::Dial> for ::abstract_slider::AbstractSlider {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dial::Dial> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QAbstractSlider(self as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::dial::Dial> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QAbstractSlider(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::dial::Dial> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dial::Dial> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::dial::Dial> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_dynamic_cast_QDial_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::dial::Dial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_static_cast_QObject_ptr(self as *mut ::dial::Dial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDial_G_static_cast_QObject_ptr(self as *const ::dial::Dial as *mut ::dial::Dial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::dial::Dial {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_static_cast_QPaintDevice_ptr(self as *mut ::dial::Dial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDial_G_static_cast_QPaintDevice_ptr(self as *const ::dial::Dial as *mut ::dial::Dial)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_slider::AbstractSlider> for ::dial::Dial {
  fn static_cast_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_static_cast_QAbstractSlider_ptr(self as *mut ::dial::Dial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDial_G_static_cast_QAbstractSlider_ptr(self as *const ::dial::Dial as *mut ::dial::Dial)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::dial::Dial {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_static_cast_QWidget_ptr(self as *mut ::dial::Dial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDial_G_static_cast_QWidget_ptr(self as *const ::dial::Dial as *mut ::dial::Dial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dial::Dial> for ::abstract_slider::AbstractSlider {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dial::Dial {
    let ffi_result = ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QAbstractSlider(self as *mut ::abstract_slider::AbstractSlider);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dial::Dial {
    let ffi_result = ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QAbstractSlider(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dial::Dial> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dial::Dial {
    let ffi_result =
      ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dial::Dial {
    let ffi_result = ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dial::Dial> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dial::Dial {
    let ffi_result = ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dial::Dial {
    let ffi_result = ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dial::Dial> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dial::Dial {
    let ffi_result = ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dial::Dial {
    let ffi_result = ::ffi::qt_widgets_c_QDial_G_static_cast_QDial_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::dial::Dial {
  type Target = ::abstract_slider::AbstractSlider;
  fn deref(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QDial_G_static_cast_QAbstractSlider_ptr(self as *const ::dial::Dial as *mut ::dial::Dial)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dial::Dial {
  fn deref_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDial_G_static_cast_QAbstractSlider_ptr(self as *mut ::dial::Dial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// C++ type: <span style='color: green;'>```QSlider```</span>
#[repr(C)]
pub struct Slider(u8);

impl Slider {
  /// C++ method: <span style='color: green;'>```virtual bool QSlider::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_widgets_c_QSlider_event(self as *mut ::slider::Slider, event)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSlider::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QSlider_metaObject(self as *const ::slider::Slider) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSlider::minimumSizeHint() const```</span>
  ///
  ///
  pub fn minimum_size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSlider_minimumSizeHint_to_output(self as *const ::slider::Slider, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSlider::QSlider```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::slider::Slider>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSlider::QSlider()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::Orientation) -> ::cpp_utils::CppBox<::slider::Slider>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSlider::QSlider(Qt::Orientation orientation)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::slider::Slider>
    where Args: overloading::SliderNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSlider::QSlider```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::slider::Slider>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSlider::QSlider(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::qt::Orientation, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::slider::Slider>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSlider::QSlider(Qt::Orientation orientation, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::slider::Slider>
    where Args: overloading::SliderNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QSlider::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QSlider_qt_metacall(self as *mut ::slider::Slider,
                                            arg1 as *const ::qt_core::meta_object::Call,
                                            arg2,
                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QSlider::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QSlider_qt_metacast(self as *mut ::slider::Slider, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QSlider::setTickInterval(int ti)```</span>
  ///
  ///
  pub fn set_tick_interval(&mut self, ti: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QSlider_setTickInterval(self as *mut ::slider::Slider, ti) }
  }

  /// C++ method: <span style='color: green;'>```void QSlider::setTickPosition(QSlider::TickPosition position)```</span>
  ///
  ///
  pub fn set_tick_position(&mut self, position: ::slider::TickPosition) {
    unsafe { ::ffi::qt_widgets_c_QSlider_setTickPosition(self as *mut ::slider::Slider, position) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSlider::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QSlider_sizeHint_to_output(self as *const ::slider::Slider, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSlider::tickInterval() const```</span>
  ///
  ///
  pub fn tick_interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QSlider_tickInterval(self as *const ::slider::Slider) }
  }

  /// C++ method: <span style='color: green;'>```QSlider::TickPosition QSlider::tickPosition() const```</span>
  ///
  ///
  pub fn tick_position(&self) -> ::slider::TickPosition {
    unsafe { ::ffi::qt_widgets_c_QSlider_tickPosition(self as *const ::slider::Slider) }
  }

  /// C++ method: <span style='color: green;'>```static QString QSlider::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSlider_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSlider::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QSlider_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::slider::Slider {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QSlider_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Slider`.
  pub struct Signals<'a>(&'a ::slider::Slider);
  /// Represents a built-in Qt signal `QSlider::valueChanged`.
  ///
  /// An object of this type can be created from `Slider` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct ValueChanged<'a>(&'a ::slider::Slider);
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
  /// Represents a built-in Qt signal `QSlider::rangeChanged`.
  ///
  /// An object of this type can be created from `Slider` with `object.signals().range_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct RangeChanged<'a>(&'a ::slider::Slider);
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
  /// Represents a built-in Qt signal `QSlider::sliderPressed`.
  ///
  /// An object of this type can be created from `Slider` with `object.signals().slider_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct SliderPressed<'a>(&'a ::slider::Slider);
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
  /// Represents a built-in Qt signal `QSlider::sliderReleased`.
  ///
  /// An object of this type can be created from `Slider` with `object.signals().slider_released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct SliderReleased<'a>(&'a ::slider::Slider);
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
  /// Represents a built-in Qt signal `QSlider::actionTriggered`.
  ///
  /// An object of this type can be created from `Slider` with `object.signals().action_triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct ActionTriggered<'a>(&'a ::slider::Slider);
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
  /// Represents a built-in Qt signal `QSlider::sliderMoved`.
  ///
  /// An object of this type can be created from `Slider` with `object.signals().slider_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct SliderMoved<'a>(&'a ::slider::Slider);
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
    /// Returns an object representing a built-in Qt signal `QSlider::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSlider::rangeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn range_changed(&self) -> RangeChanged {
      RangeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSlider::sliderPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_pressed(&self) -> SliderPressed {
      SliderPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSlider::sliderReleased`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_released(&self) -> SliderReleased {
      SliderReleased(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSlider::actionTriggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn action_triggered(&self) -> ActionTriggered {
      ActionTriggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSlider::sliderMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_moved(&self) -> SliderMoved {
      SliderMoved(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Slider`.
  pub struct Slots<'a>(&'a ::slider::Slider);
  /// Represents a built-in Qt slot `QSlider::setRange`.
  ///
  /// An object of this type can be created from `Slider` with `object.slots().set_range()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct SetRange<'a>(&'a ::slider::Slider);
  impl<'a> ::qt_core::connection::Receiver for SetRange<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRange(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QSlider::setValue`.
  ///
  /// An object of this type can be created from `Slider` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct SetValue<'a>(&'a ::slider::Slider);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QSlider::setOrientation`.
  ///
  /// An object of this type can be created from `Slider` with `object.slots().set_orientation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Slider` object.
  pub struct SetOrientation<'a>(&'a ::slider::Slider);
  impl<'a> ::qt_core::connection::Receiver for SetOrientation<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOrientation(Qt::Orientation)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSlider::setRange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_range(&self) -> SetRange {
      SetRange(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSlider::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSlider::setOrientation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_orientation(&self) -> SetOrientation {
      SetOrientation(self.0)
    }
  }
  impl ::slider::Slider {
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

/// C++ type: <span style='color: green;'>```QSlider::TickPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TickPosition {
  /// C++ enum variant: <span style='color: green;'>```NoTicks = 0```</span>
  NoTicks = 0,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```TicksAbove = 1```</span>
  /// - <span style='color: green;'>```TicksLeft = 1```</span>
  ///
  TicksAbove = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```TicksBelow = 2```</span>
  /// - <span style='color: green;'>```TicksRight = 2```</span>
  ///
  TicksBelow = 2,
  /// C++ enum variant: <span style='color: green;'>```TicksBothSides = 3```</span>
  TicksBothSides = 3,
}

impl ::cpp_utils::DynamicCast<::slider::Slider> for ::abstract_slider::AbstractSlider {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::slider::Slider> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QAbstractSlider(self as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::slider::Slider> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QAbstractSlider(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::slider::Slider> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::slider::Slider> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::slider::Slider> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_dynamic_cast_QSlider_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slider::Slider {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QObject_ptr(self as *mut ::slider::Slider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSlider_G_static_cast_QObject_ptr(self as *const ::slider::Slider as *mut ::slider::Slider)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::slider::Slider {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QPaintDevice_ptr(self as *mut ::slider::Slider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QPaintDevice_ptr(self as *const ::slider::Slider as *mut ::slider::Slider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_slider::AbstractSlider> for ::slider::Slider {
  fn static_cast_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QAbstractSlider_ptr(self as *mut ::slider::Slider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QAbstractSlider_ptr(self as *const ::slider::Slider as *mut ::slider::Slider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::slider::Slider {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QWidget_ptr(self as *mut ::slider::Slider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QSlider_G_static_cast_QWidget_ptr(self as *const ::slider::Slider as *mut ::slider::Slider)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::slider::Slider> for ::abstract_slider::AbstractSlider {
  unsafe fn static_cast_mut(&mut self) -> &mut ::slider::Slider {
    let ffi_result = ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QAbstractSlider(self as *mut ::abstract_slider::AbstractSlider);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::slider::Slider {
    let ffi_result = ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QAbstractSlider(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::slider::Slider> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::slider::Slider {
    let ffi_result =
      ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::slider::Slider {
    let ffi_result = ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::slider::Slider> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::slider::Slider {
    let ffi_result = ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::slider::Slider {
    let ffi_result = ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::slider::Slider> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::slider::Slider {
    let ffi_result = ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::slider::Slider {
    let ffi_result = ::ffi::qt_widgets_c_QSlider_G_static_cast_QSlider_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::slider::Slider {
  type Target = ::abstract_slider::AbstractSlider;
  fn deref(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QAbstractSlider_ptr(self as *const ::slider::Slider as *mut ::slider::Slider) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::slider::Slider {
  fn deref_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QSlider_G_static_cast_QAbstractSlider_ptr(self as *mut ::slider::Slider) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Slider::new](../struct.Slider.html#method.new) method.
  pub trait SliderNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::slider::Slider>;
  }
  impl SliderNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::slider::Slider> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QSlider_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SliderNewArgs for &'a ::qt_core::qt::Orientation {
    fn exec(self) -> ::cpp_utils::CppBox<::slider::Slider> {
      let orientation = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QSlider_new_orientation(orientation as *const ::qt_core::qt::Orientation) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Slider::new_unsafe](../struct.Slider.html#method.new_unsafe) method.
  pub trait SliderNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::slider::Slider>;
  }
  impl<'a> SliderNewUnsafeArgs for (&'a ::qt_core::qt::Orientation, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::slider::Slider> {
      let orientation = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QSlider_new_orientation_parent(orientation as *const ::qt_core::qt::Orientation, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SliderNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::slider::Slider> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QSlider_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

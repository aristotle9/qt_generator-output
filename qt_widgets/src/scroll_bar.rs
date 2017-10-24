/// C++ type: <span style='color: green;'>```QScrollBar```</span>
#[repr(C)]
pub struct ScrollBar(u8);

impl ScrollBar {
  /// C++ method: <span style='color: green;'>```virtual bool QScrollBar::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::qt_core::event::Event) -> bool {
    ::ffi::qt_widgets_c_QScrollBar_event(self as *mut ::scroll_bar::ScrollBar, event)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QScrollBar::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QScrollBar_metaObject(self as *const ::scroll_bar::ScrollBar) }
  }

  /// C++ method: <span style='color: green;'>```QScrollBar::QScrollBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QScrollBar::QScrollBar()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::Orientation) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QScrollBar::QScrollBar(Qt::Orientation arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>
    where Args: overloading::ScrollBarNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QScrollBar::QScrollBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QScrollBar::QScrollBar(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::qt::Orientation, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QScrollBar::QScrollBar(Qt::Orientation arg1, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>
    where Args: overloading::ScrollBarNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QScrollBar::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QScrollBar_qt_metacall(self as *mut ::scroll_bar::ScrollBar,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QScrollBar::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QScrollBar_qt_metacast(self as *mut ::scroll_bar::ScrollBar, arg1)
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QScrollBar::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QScrollBar_sizeHint_to_output(self as *const ::scroll_bar::ScrollBar, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QScrollBar::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QScrollBar_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QScrollBar::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QScrollBar_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::scroll_bar::ScrollBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QScrollBar_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ScrollBar`.
  pub struct Signals<'a>(&'a ::scroll_bar::ScrollBar);
  /// Represents a built-in Qt signal `QScrollBar::sliderReleased`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.signals().slider_released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct SliderReleased<'a>(&'a ::scroll_bar::ScrollBar);
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
  /// Represents a built-in Qt signal `QScrollBar::rangeChanged`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.signals().range_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct RangeChanged<'a>(&'a ::scroll_bar::ScrollBar);
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
  /// Represents a built-in Qt signal `QScrollBar::sliderMoved`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.signals().slider_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct SliderMoved<'a>(&'a ::scroll_bar::ScrollBar);
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
  /// Represents a built-in Qt signal `QScrollBar::actionTriggered`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.signals().action_triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct ActionTriggered<'a>(&'a ::scroll_bar::ScrollBar);
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
  /// Represents a built-in Qt signal `QScrollBar::sliderPressed`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.signals().slider_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct SliderPressed<'a>(&'a ::scroll_bar::ScrollBar);
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
  /// Represents a built-in Qt signal `QScrollBar::valueChanged`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct ValueChanged<'a>(&'a ::scroll_bar::ScrollBar);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QScrollBar::sliderReleased`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_released(&self) -> SliderReleased {
      SliderReleased(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScrollBar::rangeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn range_changed(&self) -> RangeChanged {
      RangeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScrollBar::sliderMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_moved(&self) -> SliderMoved {
      SliderMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScrollBar::actionTriggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn action_triggered(&self) -> ActionTriggered {
      ActionTriggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScrollBar::sliderPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slider_pressed(&self) -> SliderPressed {
      SliderPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QScrollBar::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ScrollBar`.
  pub struct Slots<'a>(&'a ::scroll_bar::ScrollBar);
  /// Represents a built-in Qt slot `QScrollBar::setValue`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.slots().set_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct SetValue<'a>(&'a ::scroll_bar::ScrollBar);
  impl<'a> ::qt_core::connection::Receiver for SetValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QScrollBar::setOrientation`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.slots().set_orientation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct SetOrientation<'a>(&'a ::scroll_bar::ScrollBar);
  impl<'a> ::qt_core::connection::Receiver for SetOrientation<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOrientation(Qt::Orientation)\0"
    }
  }
  /// Represents a built-in Qt slot `QScrollBar::setRange`.
  ///
  /// An object of this type can be created from `ScrollBar` with `object.slots().set_range()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ScrollBar` object.
  pub struct SetRange<'a>(&'a ::scroll_bar::ScrollBar);
  impl<'a> ::qt_core::connection::Receiver for SetRange<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRange(int,int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QScrollBar::setValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_value(&self) -> SetValue {
      SetValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QScrollBar::setOrientation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_orientation(&self) -> SetOrientation {
      SetOrientation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QScrollBar::setRange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_range(&self) -> SetRange {
      SetRange(self.0)
    }
  }
  impl ::scroll_bar::ScrollBar {
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

impl ::cpp_utils::DynamicCast<::scroll_bar::ScrollBar> for ::abstract_slider::AbstractSlider {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::scroll_bar::ScrollBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QAbstractSlider(self as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::scroll_bar::ScrollBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QAbstractSlider(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::scroll_bar::ScrollBar> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::scroll_bar::ScrollBar> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::scroll_bar::ScrollBar> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_dynamic_cast_QScrollBar_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::scroll_bar::ScrollBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QObject_ptr(self as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QObject_ptr(self as *const ::scroll_bar::ScrollBar as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::scroll_bar::ScrollBar {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QPaintDevice_ptr(self as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QPaintDevice_ptr(self as *const ::scroll_bar::ScrollBar as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_slider::AbstractSlider> for ::scroll_bar::ScrollBar {
  fn static_cast_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QAbstractSlider_ptr(self as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QAbstractSlider_ptr(self as *const ::scroll_bar::ScrollBar as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::scroll_bar::ScrollBar {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QWidget_ptr(self as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QWidget_ptr(self as *const ::scroll_bar::ScrollBar as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_bar::ScrollBar> for ::abstract_slider::AbstractSlider {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_bar::ScrollBar {
    let ffi_result = ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QAbstractSlider(self as *mut ::abstract_slider::AbstractSlider);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_bar::ScrollBar {
    let ffi_result = ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QAbstractSlider(self as *const ::abstract_slider::AbstractSlider as *mut ::abstract_slider::AbstractSlider);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_bar::ScrollBar> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_bar::ScrollBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_bar::ScrollBar {
    let ffi_result = ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_bar::ScrollBar> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_bar::ScrollBar {
    let ffi_result = ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_bar::ScrollBar {
    let ffi_result = ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_bar::ScrollBar> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_bar::ScrollBar {
    let ffi_result =
      ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_bar::ScrollBar {
    let ffi_result = ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QScrollBar_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::scroll_bar::ScrollBar {
  type Target = ::abstract_slider::AbstractSlider;
  fn deref(&self) -> &::abstract_slider::AbstractSlider {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QAbstractSlider_ptr(self as *const ::scroll_bar::ScrollBar as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::scroll_bar::ScrollBar {
  fn deref_mut(&mut self) -> &mut ::abstract_slider::AbstractSlider {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QScrollBar_G_static_cast_QAbstractSlider_ptr(self as *mut ::scroll_bar::ScrollBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ScrollBar::new](../struct.ScrollBar.html#method.new) method.
  pub trait ScrollBarNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>;
  }
  impl<'a> ScrollBarNewArgs for &'a ::qt_core::qt::Orientation {
    fn exec(self) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_new_arg1(arg1 as *const ::qt_core::qt::Orientation) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ScrollBarNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QScrollBar_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [ScrollBar::new_unsafe](../struct.ScrollBar.html#method.new_unsafe) method.
  pub trait ScrollBarNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar>;
  }
  impl<'a> ScrollBarNewUnsafeArgs for (&'a ::qt_core::qt::Orientation, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar> {
      let arg1 = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QScrollBar_new_arg1_parent(arg1 as *const ::qt_core::qt::Orientation,
                                                                      parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ScrollBarNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::scroll_bar::ScrollBar> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QScrollBar_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

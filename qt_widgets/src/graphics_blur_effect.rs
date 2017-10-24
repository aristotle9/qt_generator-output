/// C++ type: <span style='color: green;'>```QGraphicsBlurEffect::BlurHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BlurHint {
  /// C++ enum variant: <span style='color: green;'>```PerformanceHint = 0```</span>
  Performance = 0,
  /// C++ enum variant: <span style='color: green;'>```QualityHint = 1```</span>
  Quality = 1,
  /// C++ enum variant: <span style='color: green;'>```AnimationHint = 2```</span>
  Animation = 2,
}

impl ::qt_core::flags::FlaggableEnum for BlurHint {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "BlurHint"
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsBlurEffect```</span>
#[repr(C)]
pub struct GraphicsBlurEffect(u8);

impl GraphicsBlurEffect {
  /// C++ method: <span style='color: green;'>```QFlags<QGraphicsBlurEffect::BlurHint> QGraphicsBlurEffect::blurHints() const```</span>
  ///
  ///
  pub fn blur_hints(&self) -> ::qt_core::flags::Flags<::graphics_blur_effect::BlurHint> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsBlurEffect_blurHints(self as *const ::graphics_blur_effect::GraphicsBlurEffect)
      };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsBlurEffect::blurRadius() const```</span>
  ///
  ///
  pub fn blur_radius(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsBlurEffect_blurRadius(self as *const ::graphics_blur_effect::GraphicsBlurEffect)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF& rect) const```</span>
  ///
  ///
  pub fn bounding_rect_for(&self, rect: &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsBlurEffect_boundingRectFor_to_output(self as *const ::graphics_blur_effect::GraphicsBlurEffect, rect as *const ::qt_core::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsBlurEffect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsBlurEffect_metaObject(self as *const ::graphics_blur_effect::GraphicsBlurEffect)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsBlurEffect::QGraphicsBlurEffect()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_blur_effect::GraphicsBlurEffect> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_blur_effect::GraphicsBlurEffect> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsBlurEffect_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsBlurEffect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsBlurEffect_qt_metacall(self as *mut ::graphics_blur_effect::GraphicsBlurEffect,
                                                        arg1 as *const ::qt_core::meta_object::Call,
                                                        arg2,
                                                        arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsBlurEffect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsBlurEffect_qt_metacast(self as *mut ::graphics_blur_effect::GraphicsBlurEffect,
                                                        arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsBlurEffect::setBlurHints(QFlags<QGraphicsBlurEffect::BlurHint> hints)```</span>
  ///
  ///
  pub fn set_blur_hints(&mut self, hints: ::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsBlurEffect_setBlurHints(self as *mut ::graphics_blur_effect::GraphicsBlurEffect,
                                                           hints.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsBlurEffect::setBlurRadius(double blurRadius)```</span>
  ///
  ///
  pub fn set_blur_radius(&mut self, blur_radius: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsBlurEffect_setBlurRadius(self as *mut ::graphics_blur_effect::GraphicsBlurEffect,
                                                            blur_radius)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsBlurEffect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsBlurEffect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsBlurEffect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsBlurEffect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_blur_effect::GraphicsBlurEffect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsBlurEffect_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsBlurEffect`.
  pub struct Signals<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  /// Represents a built-in Qt signal `QGraphicsBlurEffect::blurHintsChanged`.
  ///
  /// An object of this type can be created from `GraphicsBlurEffect` with `object.signals().blur_hints_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsBlurEffect` object.
  pub struct BlurHintsChanged<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  impl<'a> ::qt_core::connection::Receiver for BlurHintsChanged<'a> {
    type Arguments = (::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2blurHintsChanged(QFlags< QGraphicsBlurEffect::BlurHint >)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BlurHintsChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsBlurEffect::blurRadiusChanged`.
  ///
  /// An object of this type can be created from `GraphicsBlurEffect` with `object.signals().blur_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsBlurEffect` object.
  pub struct BlurRadiusChanged<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  impl<'a> ::qt_core::connection::Receiver for BlurRadiusChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2blurRadiusChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BlurRadiusChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsBlurEffect::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsBlurEffect` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsBlurEffect` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsBlurEffect::blurHintsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn blur_hints_changed(&self) -> BlurHintsChanged {
      BlurHintsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsBlurEffect::blurRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn blur_radius_changed(&self) -> BlurRadiusChanged {
      BlurRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsBlurEffect::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsBlurEffect`.
  pub struct Slots<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  /// Represents a built-in Qt slot `QGraphicsBlurEffect::setEnabled`.
  ///
  /// An object of this type can be created from `GraphicsBlurEffect` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsBlurEffect` object.
  pub struct SetEnabled<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsBlurEffect::setBlurRadius`.
  ///
  /// An object of this type can be created from `GraphicsBlurEffect` with `object.slots().set_blur_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsBlurEffect` object.
  pub struct SetBlurRadius<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  impl<'a> ::qt_core::connection::Receiver for SetBlurRadius<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBlurRadius(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsBlurEffect::setBlurHints`.
  ///
  /// An object of this type can be created from `GraphicsBlurEffect` with `object.slots().set_blur_hints()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsBlurEffect` object.
  pub struct SetBlurHints<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  impl<'a> ::qt_core::connection::Receiver for SetBlurHints<'a> {
    type Arguments = (::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBlurHints(QFlags< QGraphicsBlurEffect::BlurHint >)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsBlurEffect::update`.
  ///
  /// An object of this type can be created from `GraphicsBlurEffect` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsBlurEffect` object.
  pub struct Update<'a>(&'a ::graphics_blur_effect::GraphicsBlurEffect);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsBlurEffect::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsBlurEffect::setBlurRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_blur_radius(&self) -> SetBlurRadius {
      SetBlurRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsBlurEffect::setBlurHints`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_blur_hints(&self) -> SetBlurHints {
      SetBlurHints(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsBlurEffect::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::graphics_blur_effect::GraphicsBlurEffect {
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

impl ::cpp_utils::DynamicCast<::graphics_blur_effect::GraphicsBlurEffect> for ::graphics_effect::GraphicsEffect {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_blur_effect::GraphicsBlurEffect> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_dynamic_cast_QGraphicsBlurEffect_ptr(self as *mut ::graphics_effect::GraphicsEffect) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_blur_effect::GraphicsBlurEffect> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_dynamic_cast_QGraphicsBlurEffect_ptr(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_blur_effect::GraphicsBlurEffect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QObject_ptr(self as *mut ::graphics_blur_effect::GraphicsBlurEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QObject_ptr(self as *const ::graphics_blur_effect::GraphicsBlurEffect as *mut ::graphics_blur_effect::GraphicsBlurEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_effect::GraphicsEffect> for ::graphics_blur_effect::GraphicsBlurEffect {
  fn static_cast_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_blur_effect::GraphicsBlurEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_blur_effect::GraphicsBlurEffect as *mut ::graphics_blur_effect::GraphicsBlurEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_blur_effect::GraphicsBlurEffect> for ::graphics_effect::GraphicsEffect {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_blur_effect::GraphicsBlurEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QGraphicsEffect(self as *mut ::graphics_effect::GraphicsEffect);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_blur_effect::GraphicsBlurEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QGraphicsEffect(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_blur_effect::GraphicsBlurEffect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_blur_effect::GraphicsBlurEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_blur_effect::GraphicsBlurEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsBlurEffect_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_blur_effect::GraphicsBlurEffect {
  type Target = ::graphics_effect::GraphicsEffect;
  fn deref(&self) -> &::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_blur_effect::GraphicsBlurEffect as *mut ::graphics_blur_effect::GraphicsBlurEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_blur_effect::GraphicsBlurEffect {
  fn deref_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsBlurEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_blur_effect::GraphicsBlurEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

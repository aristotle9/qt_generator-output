/// C++ type: <span style='color: green;'>```QGraphicsEffect::ChangeFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ChangeFlag {
  /// C++ enum variant: <span style='color: green;'>```SourceAttached = 1```</span>
  Attached = 1,
  /// C++ enum variant: <span style='color: green;'>```SourceDetached = 2```</span>
  Detached = 2,
  /// C++ enum variant: <span style='color: green;'>```SourceBoundingRectChanged = 4```</span>
  BoundingRectChanged = 4,
  /// C++ enum variant: <span style='color: green;'>```SourceInvalidated = 8```</span>
  Invalidated = 8,
}

impl ::qt_core::flags::FlaggableEnum for ChangeFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ChangeFlag"
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsEffect```</span>
#[repr(C)]
pub struct GraphicsEffect(u8);

impl GraphicsEffect {
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsEffect::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEffect_boundingRect_to_output(self as *const ::graphics_effect::GraphicsEffect,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsEffect::boundingRectFor(const QRectF& sourceRect) const```</span>
  ///
  ///
  pub fn bounding_rect_for(&self, source_rect: &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEffect_boundingRectFor_to_output(self as *const ::graphics_effect::GraphicsEffect, source_rect as *const ::qt_core::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsEffect::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsEffect_isEnabled(self as *const ::graphics_effect::GraphicsEffect) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsEffect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsEffect_metaObject(self as *const ::graphics_effect::GraphicsEffect) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsEffect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsEffect_qt_metacall(self as *mut ::graphics_effect::GraphicsEffect,
                                                    arg1 as *const ::qt_core::meta_object::Call,
                                                    arg2,
                                                    arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsEffect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsEffect_qt_metacast(self as *mut ::graphics_effect::GraphicsEffect, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsEffect::setEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsEffect_setEnabled(self as *mut ::graphics_effect::GraphicsEffect, enable) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsEffect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsEffect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsEffect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsEffect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsEffect::update()```</span>
  ///
  ///
  pub fn update(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsEffect_update(self as *mut ::graphics_effect::GraphicsEffect) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_effect::GraphicsEffect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsEffect_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsEffect`.
  pub struct Signals<'a>(&'a ::graphics_effect::GraphicsEffect);
  /// Represents a built-in Qt signal `QGraphicsEffect::objectNameChanged`.
  ///
  /// An object of this type can be created from `GraphicsEffect` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsEffect` object.
  pub struct ObjectNameChanged<'a>(&'a ::graphics_effect::GraphicsEffect);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsEffect::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsEffect` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsEffect` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_effect::GraphicsEffect);
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
    /// Returns an object representing a built-in Qt signal `QGraphicsEffect::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsEffect::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsEffect`.
  pub struct Slots<'a>(&'a ::graphics_effect::GraphicsEffect);
  /// Represents a built-in Qt slot `QGraphicsEffect::update`.
  ///
  /// An object of this type can be created from `GraphicsEffect` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsEffect` object.
  pub struct Update<'a>(&'a ::graphics_effect::GraphicsEffect);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsEffect::setEnabled`.
  ///
  /// An object of this type can be created from `GraphicsEffect` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsEffect` object.
  pub struct SetEnabled<'a>(&'a ::graphics_effect::GraphicsEffect);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsEffect::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsEffect::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::graphics_effect::GraphicsEffect {
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

/// C++ type: <span style='color: green;'>```QGraphicsEffect::PixmapPadMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PixmapPadMode {
  /// C++ enum variant: <span style='color: green;'>```NoPad = 0```</span>
  NoPad = 0,
  /// C++ enum variant: <span style='color: green;'>```PadToTransparentBorder = 1```</span>
  PadToTransparentBorder = 1,
  /// C++ enum variant: <span style='color: green;'>```PadToEffectiveBoundingRect = 2```</span>
  PadToEffectiveBoundingRect = 2,
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_effect::GraphicsEffect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEffect_G_static_cast_QObject_ptr(self as *mut ::graphics_effect::GraphicsEffect)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEffect_G_static_cast_QObject_ptr(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_effect::GraphicsEffect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
    let ffi_result =
      ::ffi::qt_widgets_c_QGraphicsEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_effect::GraphicsEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_effect::GraphicsEffect {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEffect_G_static_cast_QObject_ptr(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_effect::GraphicsEffect {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEffect_G_static_cast_QObject_ptr(self as *mut ::graphics_effect::GraphicsEffect)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

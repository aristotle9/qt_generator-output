/// C++ type: <span style='color: green;'>```QGraphicsOpacityEffect```</span>
#[repr(C)]
pub struct GraphicsOpacityEffect(u8);

impl GraphicsOpacityEffect {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsOpacityEffect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_metaObject(self as *const ::graphics_opacity_effect::GraphicsOpacityEffect) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsOpacityEffect::QGraphicsOpacityEffect()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_opacity_effect::GraphicsOpacityEffect> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsOpacityEffect::QGraphicsOpacityEffect(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_opacity_effect::GraphicsOpacityEffect> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsOpacityEffect_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsOpacityEffect::opacity() const```</span>
  ///
  ///
  pub fn opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_opacity(self as *const ::graphics_opacity_effect::GraphicsOpacityEffect) }
  }

  /// C++ method: <span style='color: green;'>```QBrush QGraphicsOpacityEffect::opacityMask() const```</span>
  ///
  ///
  pub fn opacity_mask(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsOpacityEffect_opacityMask_to_output(self as *const ::graphics_opacity_effect::GraphicsOpacityEffect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsOpacityEffect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsOpacityEffect_qt_metacall(self as *mut ::graphics_opacity_effect::GraphicsOpacityEffect, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsOpacityEffect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsOpacityEffect_qt_metacast(self as *mut ::graphics_opacity_effect::GraphicsOpacityEffect, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsOpacityEffect::setOpacity(double opacity)```</span>
  ///
  ///
  pub fn set_opacity(&mut self, opacity: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_setOpacity(self as *mut ::graphics_opacity_effect::GraphicsOpacityEffect, opacity) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsOpacityEffect::setOpacityMask(const QBrush& mask)```</span>
  ///
  ///
  pub fn set_opacity_mask(&mut self, mask: &::qt_gui::brush::Brush) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_setOpacityMask(self as *mut ::graphics_opacity_effect::GraphicsOpacityEffect, mask as *const ::qt_gui::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsOpacityEffect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsOpacityEffect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsOpacityEffect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsOpacityEffect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_opacity_effect::GraphicsOpacityEffect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsOpacityEffect_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsOpacityEffect`.
  pub struct Signals<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
  /// Represents a built-in Qt signal `QGraphicsOpacityEffect::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsOpacityEffect` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsOpacityEffect` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
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
  /// Represents a built-in Qt signal `QGraphicsOpacityEffect::opacityChanged`.
  ///
  /// An object of this type can be created from `GraphicsOpacityEffect` with `object.signals().opacity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsOpacityEffect` object.
  pub struct OpacityChanged<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
  impl<'a> ::qt_core::connection::Receiver for OpacityChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2opacityChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OpacityChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsOpacityEffect::opacityMaskChanged`.
  ///
  /// An object of this type can be created from `GraphicsOpacityEffect` with `object.signals().opacity_mask_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsOpacityEffect` object.
  pub struct OpacityMaskChanged<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
  impl<'a> ::qt_core::connection::Receiver for OpacityMaskChanged<'a> {
    type Arguments = (&'static ::qt_gui::brush::Brush,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2opacityMaskChanged(const QBrush&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OpacityMaskChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsOpacityEffect::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsOpacityEffect::opacityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_changed(&self) -> OpacityChanged {
      OpacityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsOpacityEffect::opacityMaskChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn opacity_mask_changed(&self) -> OpacityMaskChanged {
      OpacityMaskChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsOpacityEffect`.
  pub struct Slots<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
  /// Represents a built-in Qt slot `QGraphicsOpacityEffect::setOpacity`.
  ///
  /// An object of this type can be created from `GraphicsOpacityEffect` with `object.slots().set_opacity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsOpacityEffect` object.
  pub struct SetOpacity<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
  impl<'a> ::qt_core::connection::Receiver for SetOpacity<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOpacity(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsOpacityEffect::setOpacityMask`.
  ///
  /// An object of this type can be created from `GraphicsOpacityEffect` with `object.slots().set_opacity_mask()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsOpacityEffect` object.
  pub struct SetOpacityMask<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
  impl<'a> ::qt_core::connection::Receiver for SetOpacityMask<'a> {
    type Arguments = (&'static ::qt_gui::brush::Brush,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOpacityMask(const QBrush&)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsOpacityEffect::update`.
  ///
  /// An object of this type can be created from `GraphicsOpacityEffect` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsOpacityEffect` object.
  pub struct Update<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsOpacityEffect::setEnabled`.
  ///
  /// An object of this type can be created from `GraphicsOpacityEffect` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsOpacityEffect` object.
  pub struct SetEnabled<'a>(&'a ::graphics_opacity_effect::GraphicsOpacityEffect);
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
    /// Returns an object representing a built-in Qt slot `QGraphicsOpacityEffect::setOpacity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_opacity(&self) -> SetOpacity {
      SetOpacity(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsOpacityEffect::setOpacityMask`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_opacity_mask(&self) -> SetOpacityMask {
      SetOpacityMask(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsOpacityEffect::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsOpacityEffect::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::graphics_opacity_effect::GraphicsOpacityEffect {
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

impl ::cpp_utils::DynamicCast<::graphics_opacity_effect::GraphicsOpacityEffect> for ::graphics_effect::GraphicsEffect {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_opacity_effect::GraphicsOpacityEffect> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_dynamic_cast_QGraphicsOpacityEffect_ptr(self as *mut ::graphics_effect::GraphicsEffect) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_opacity_effect::GraphicsOpacityEffect> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_dynamic_cast_QGraphicsOpacityEffect_ptr(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_opacity_effect::GraphicsOpacityEffect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QObject_ptr(self as *mut ::graphics_opacity_effect::GraphicsOpacityEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QObject_ptr(self as *const ::graphics_opacity_effect::GraphicsOpacityEffect as *mut ::graphics_opacity_effect::GraphicsOpacityEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_effect::GraphicsEffect> for ::graphics_opacity_effect::GraphicsOpacityEffect {
  fn static_cast_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_opacity_effect::GraphicsOpacityEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_opacity_effect::GraphicsOpacityEffect as *mut ::graphics_opacity_effect::GraphicsOpacityEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_opacity_effect::GraphicsOpacityEffect> for ::graphics_effect::GraphicsEffect {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_opacity_effect::GraphicsOpacityEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QGraphicsEffect(self as *mut ::graphics_effect::GraphicsEffect);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_opacity_effect::GraphicsOpacityEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QGraphicsEffect(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_opacity_effect::GraphicsOpacityEffect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_opacity_effect::GraphicsOpacityEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_opacity_effect::GraphicsOpacityEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsOpacityEffect_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_opacity_effect::GraphicsOpacityEffect {
  type Target = ::graphics_effect::GraphicsEffect;
  fn deref(&self) -> &::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_opacity_effect::GraphicsOpacityEffect as *mut ::graphics_opacity_effect::GraphicsOpacityEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_opacity_effect::GraphicsOpacityEffect {
  fn deref_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsOpacityEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_opacity_effect::GraphicsOpacityEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

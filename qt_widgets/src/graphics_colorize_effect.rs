/// C++ type: <span style='color: green;'>```QGraphicsColorizeEffect```</span>
#[repr(C)]
pub struct GraphicsColorizeEffect(u8);

impl GraphicsColorizeEffect {
  /// C++ method: <span style='color: green;'>```QColor QGraphicsColorizeEffect::color() const```</span>
  ///
  ///
  pub fn color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsColorizeEffect_color_to_output(self as *const ::graphics_colorize_effect::GraphicsColorizeEffect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsColorizeEffect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_metaObject(self as *const ::graphics_colorize_effect::GraphicsColorizeEffect) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsColorizeEffect::QGraphicsColorizeEffect()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_colorize_effect::GraphicsColorizeEffect> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_colorize_effect::GraphicsColorizeEffect> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsColorizeEffect_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsColorizeEffect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsColorizeEffect_qt_metacall(self as *mut ::graphics_colorize_effect::GraphicsColorizeEffect, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsColorizeEffect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsColorizeEffect_qt_metacast(self as *mut ::graphics_colorize_effect::GraphicsColorizeEffect, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsColorizeEffect::setColor(const QColor& c)```</span>
  ///
  ///
  pub fn set_color(&mut self, c: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_setColor(self as *mut ::graphics_colorize_effect::GraphicsColorizeEffect, c as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsColorizeEffect::setStrength(double strength)```</span>
  ///
  ///
  pub fn set_strength(&mut self, strength: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_setStrength(self as *mut ::graphics_colorize_effect::GraphicsColorizeEffect, strength) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsColorizeEffect::strength() const```</span>
  ///
  ///
  pub fn strength(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_strength(self as *const ::graphics_colorize_effect::GraphicsColorizeEffect) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsColorizeEffect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsColorizeEffect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsColorizeEffect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsColorizeEffect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_colorize_effect::GraphicsColorizeEffect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsColorizeEffect_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsColorizeEffect`.
  pub struct Signals<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
  /// Represents a built-in Qt signal `QGraphicsColorizeEffect::strengthChanged`.
  ///
  /// An object of this type can be created from `GraphicsColorizeEffect` with `object.signals().strength_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsColorizeEffect` object.
  pub struct StrengthChanged<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
  impl<'a> ::qt_core::connection::Receiver for StrengthChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2strengthChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StrengthChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsColorizeEffect::colorChanged`.
  ///
  /// An object of this type can be created from `GraphicsColorizeEffect` with `object.signals().color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsColorizeEffect` object.
  pub struct ColorChanged<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
  impl<'a> ::qt_core::connection::Receiver for ColorChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2colorChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColorChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsColorizeEffect::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsColorizeEffect` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsColorizeEffect` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
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
    /// Returns an object representing a built-in Qt signal `QGraphicsColorizeEffect::strengthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn strength_changed(&self) -> StrengthChanged {
      StrengthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsColorizeEffect::colorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_changed(&self) -> ColorChanged {
      ColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsColorizeEffect::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsColorizeEffect`.
  pub struct Slots<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
  /// Represents a built-in Qt slot `QGraphicsColorizeEffect::setStrength`.
  ///
  /// An object of this type can be created from `GraphicsColorizeEffect` with `object.slots().set_strength()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsColorizeEffect` object.
  pub struct SetStrength<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
  impl<'a> ::qt_core::connection::Receiver for SetStrength<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStrength(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsColorizeEffect::update`.
  ///
  /// An object of this type can be created from `GraphicsColorizeEffect` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsColorizeEffect` object.
  pub struct Update<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsColorizeEffect::setColor`.
  ///
  /// An object of this type can be created from `GraphicsColorizeEffect` with `object.slots().set_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsColorizeEffect` object.
  pub struct SetColor<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
  impl<'a> ::qt_core::connection::Receiver for SetColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsColorizeEffect::setEnabled`.
  ///
  /// An object of this type can be created from `GraphicsColorizeEffect` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsColorizeEffect` object.
  pub struct SetEnabled<'a>(&'a ::graphics_colorize_effect::GraphicsColorizeEffect);
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
    /// Returns an object representing a built-in Qt slot `QGraphicsColorizeEffect::setStrength`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_strength(&self) -> SetStrength {
      SetStrength(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsColorizeEffect::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsColorizeEffect::setColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_color(&self) -> SetColor {
      SetColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsColorizeEffect::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::graphics_colorize_effect::GraphicsColorizeEffect {
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

impl ::cpp_utils::DynamicCast<::graphics_colorize_effect::GraphicsColorizeEffect> for ::graphics_effect::GraphicsEffect {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_colorize_effect::GraphicsColorizeEffect> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_dynamic_cast_QGraphicsColorizeEffect_ptr(self as *mut ::graphics_effect::GraphicsEffect) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_colorize_effect::GraphicsColorizeEffect> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_dynamic_cast_QGraphicsColorizeEffect_ptr(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_colorize_effect::GraphicsColorizeEffect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QObject_ptr(self as *mut ::graphics_colorize_effect::GraphicsColorizeEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QObject_ptr(self as *const ::graphics_colorize_effect::GraphicsColorizeEffect as *mut ::graphics_colorize_effect::GraphicsColorizeEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_effect::GraphicsEffect> for ::graphics_colorize_effect::GraphicsColorizeEffect {
fn static_cast_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_colorize_effect::GraphicsColorizeEffect) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_effect::GraphicsEffect {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_colorize_effect::GraphicsColorizeEffect as *mut ::graphics_colorize_effect::GraphicsColorizeEffect) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_colorize_effect::GraphicsColorizeEffect> for ::graphics_effect::GraphicsEffect {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_colorize_effect::GraphicsColorizeEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsColorizeEffect_ptr_QGraphicsEffect(self as *mut ::graphics_effect::GraphicsEffect);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_colorize_effect::GraphicsColorizeEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsColorizeEffect_ptr_QGraphicsEffect(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_colorize_effect::GraphicsColorizeEffect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_colorize_effect::GraphicsColorizeEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsColorizeEffect_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_colorize_effect::GraphicsColorizeEffect {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsColorizeEffect_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_colorize_effect::GraphicsColorizeEffect {
  type Target = ::graphics_effect::GraphicsEffect;
  fn deref(&self) -> &::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_colorize_effect::GraphicsColorizeEffect as *mut ::graphics_colorize_effect::GraphicsColorizeEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_colorize_effect::GraphicsColorizeEffect {
  fn deref_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsColorizeEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_colorize_effect::GraphicsColorizeEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

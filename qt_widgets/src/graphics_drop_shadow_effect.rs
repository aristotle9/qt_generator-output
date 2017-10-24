/// C++ type: <span style='color: green;'>```QGraphicsDropShadowEffect```</span>
#[repr(C)]
pub struct GraphicsDropShadowEffect(u8);

impl GraphicsDropShadowEffect {
  /// C++ method: <span style='color: green;'>```double QGraphicsDropShadowEffect::blurRadius() const```</span>
  ///
  ///
  pub fn blur_radius(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_blurRadius(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF& rect) const```</span>
  ///
  ///
  pub fn bounding_rect_for(&self, rect: &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_boundingRectFor_to_output(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, rect as *const ::qt_core::rect_f::RectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QGraphicsDropShadowEffect::color() const```</span>
  ///
  ///
  pub fn color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_color_to_output(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsDropShadowEffect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_metaObject(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_drop_shadow_effect::GraphicsDropShadowEffect> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_drop_shadow_effect::GraphicsDropShadowEffect> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsDropShadowEffect::offset() const```</span>
  ///
  ///
  pub fn offset(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_offset_to_output(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsDropShadowEffect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_qt_metacall(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsDropShadowEffect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_qt_metacast(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsDropShadowEffect::setBlurRadius(double blurRadius)```</span>
  ///
  ///
  pub fn set_blur_radius(&mut self, blur_radius: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_setBlurRadius(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, blur_radius) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsDropShadowEffect::setColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_setColor(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, color as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsDropShadowEffect::setOffset```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_offset(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsDropShadowEffect::setOffset(const QPointF& ofs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_offset(&mut self, ::libc::c_double) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsDropShadowEffect::setOffset(double d)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_offset(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsDropShadowEffect::setOffset(double dx, double dy)```</span>
  ///
  ///
  pub fn set_offset<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsDropShadowEffectSetOffsetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsDropShadowEffect::setXOffset(double dx)```</span>
  ///
  ///
  pub fn set_x_offset(&mut self, dx: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_setXOffset(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, dx) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QGraphicsDropShadowEffect::setYOffset(double dy)```</span>
  ///
  ///
  pub fn set_y_offset(&mut self, dy: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_setYOffset(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, dy) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsDropShadowEffect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsDropShadowEffect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsDropShadowEffect::xOffset() const```</span>
  ///
  ///
  pub fn x_offset(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_xOffset(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsDropShadowEffect::yOffset() const```</span>
  ///
  ///
  pub fn y_offset(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_yOffset(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsDropShadowEffect`.
  pub struct Signals<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  /// Represents a built-in Qt signal `QGraphicsDropShadowEffect::blurRadiusChanged`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.signals().blur_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct BlurRadiusChanged<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
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
  /// Represents a built-in Qt signal `QGraphicsDropShadowEffect::offsetChanged`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.signals().offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct OffsetChanged<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for OffsetChanged<'a> {
    type Arguments = (&'static ::qt_core::point_f::PointF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2offsetChanged(const QPointF&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OffsetChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsDropShadowEffect::enabledChanged`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct EnabledChanged<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
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
  /// Represents a built-in Qt signal `QGraphicsDropShadowEffect::colorChanged`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.signals().color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct ColorChanged<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsDropShadowEffect::blurRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn blur_radius_changed(&self) -> BlurRadiusChanged {
      BlurRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsDropShadowEffect::offsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn offset_changed(&self) -> OffsetChanged {
      OffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsDropShadowEffect::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsDropShadowEffect::colorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_changed(&self) -> ColorChanged {
      ColorChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsDropShadowEffect`.
  pub struct Slots<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setColor`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetColor<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setOffset`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_offset_qt_core_point_f_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetOffsetQtCorePointFRef<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetOffsetQtCorePointFRef<'a> {
    type Arguments = (&'static ::qt_core::point_f::PointF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOffset(const QPointF&)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setOffset`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_offset_c_double_c_double()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetOffsetCDoubleCDouble<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetOffsetCDoubleCDouble<'a> {
    type Arguments = (::libc::c_double, ::libc::c_double);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOffset(double,double)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setOffset`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_offset_c_double()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetOffsetCDouble<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetOffsetCDouble<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setOffset(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::update`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct Update<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for Update<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1update()\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setBlurRadius`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_blur_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetBlurRadius<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetBlurRadius<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBlurRadius(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setXOffset`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_x_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetXOffset<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetXOffset<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setXOffset(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setEnabled`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetEnabled<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QGraphicsDropShadowEffect::setYOffset`.
  ///
  /// An object of this type can be created from `GraphicsDropShadowEffect` with `object.slots().set_y_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsDropShadowEffect` object.
  pub struct SetYOffset<'a>(&'a ::graphics_drop_shadow_effect::GraphicsDropShadowEffect);
  impl<'a> ::qt_core::connection::Receiver for SetYOffset<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setYOffset(double)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_color(&self) -> SetColor {
      SetColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_offset_qt_core_point_f_ref(&self) -> SetOffsetQtCorePointFRef {
      SetOffsetQtCorePointFRef(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_offset_c_double_c_double(&self) -> SetOffsetCDoubleCDouble {
      SetOffsetCDoubleCDouble(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_offset_c_double(&self) -> SetOffsetCDouble {
      SetOffsetCDouble(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setBlurRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_blur_radius(&self) -> SetBlurRadius {
      SetBlurRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setXOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x_offset(&self) -> SetXOffset {
      SetXOffset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QGraphicsDropShadowEffect::setYOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y_offset(&self) -> SetYOffset {
      SetYOffset(self.0)
    }
  }
  impl ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
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

impl ::cpp_utils::DynamicCast<::graphics_drop_shadow_effect::GraphicsDropShadowEffect> for ::graphics_effect::GraphicsEffect {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_dynamic_cast_QGraphicsDropShadowEffect_ptr(self as *mut ::graphics_effect::GraphicsEffect) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_drop_shadow_effect::GraphicsDropShadowEffect> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_dynamic_cast_QGraphicsDropShadowEffect_ptr(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QObject_ptr(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QObject_ptr(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_effect::GraphicsEffect> for ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
fn static_cast_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_effect::GraphicsEffect {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_drop_shadow_effect::GraphicsDropShadowEffect> for ::graphics_effect::GraphicsEffect {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QGraphicsEffect(self as *mut ::graphics_effect::GraphicsEffect);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QGraphicsEffect(self as *const ::graphics_effect::GraphicsEffect as *mut ::graphics_effect::GraphicsEffect);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_drop_shadow_effect::GraphicsDropShadowEffect> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsDropShadowEffect_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
  type Target = ::graphics_effect::GraphicsEffect;
  fn deref(&self) -> &::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsEffect_ptr(self as *const ::graphics_drop_shadow_effect::GraphicsDropShadowEffect as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_drop_shadow_effect::GraphicsDropShadowEffect {
  fn deref_mut(&mut self) -> &mut ::graphics_effect::GraphicsEffect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_G_static_cast_QGraphicsEffect_ptr(self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsDropShadowEffect::set_offset](../struct.GraphicsDropShadowEffect.html#method.set_offset) method.
  pub trait GraphicsDropShadowEffectSetOffsetArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) -> ();
  }
  impl<'largs> GraphicsDropShadowEffectSetOffsetArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) -> () {
      let d = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_setOffset_d(original_self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, d) }
    }
  }
  impl<'largs> GraphicsDropShadowEffectSetOffsetArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_setOffset_dx_dy(original_self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, dx, dy) }
    }
  }
  impl<'largs> GraphicsDropShadowEffectSetOffsetArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect) -> () {
      let ofs = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsDropShadowEffect_setOffset_ofs(original_self as *mut ::graphics_drop_shadow_effect::GraphicsDropShadowEffect, ofs as *const ::qt_core::point_f::PointF) }
    }
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsScale```</span>
#[repr(C)]
pub struct GraphicsScale(u8);

impl GraphicsScale {
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsScale::applyTo(QMatrix4x4* matrix) const```</span>
  ///
  ///
  pub unsafe fn apply_to(&self, matrix: *mut ::qt_gui::matrix_4x4::Matrix4X4) {
    ::ffi::qt_widgets_c_QGraphicsScale_applyTo(self as *const ::graphics_scale::GraphicsScale, matrix)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsScale::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScale_metaObject(self as *const ::graphics_scale::GraphicsScale) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScale::QGraphicsScale()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_scale::GraphicsScale> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsScale::QGraphicsScale(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_scale::GraphicsScale> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsScale_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector3D QGraphicsScale::origin() const```</span>
  ///
  ///
  pub fn origin(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGraphicsScale_origin_as_ptr(self as *const ::graphics_scale::GraphicsScale) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsScale::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsScale_qt_metacall(self as *mut ::graphics_scale::GraphicsScale,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsScale::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsScale_qt_metacast(self as *mut ::graphics_scale::GraphicsScale, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScale::setOrigin(const QVector3D& point)```</span>
  ///
  ///
  pub fn set_origin(&mut self, point: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsScale_setOrigin(self as *mut ::graphics_scale::GraphicsScale,
                                                   point as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScale::setXScale(double arg1)```</span>
  ///
  ///
  pub fn set_x_scale(&mut self, arg1: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScale_setXScale(self as *mut ::graphics_scale::GraphicsScale, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScale::setYScale(double arg1)```</span>
  ///
  ///
  pub fn set_y_scale(&mut self, arg1: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScale_setYScale(self as *mut ::graphics_scale::GraphicsScale, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsScale::setZScale(double arg1)```</span>
  ///
  ///
  pub fn set_z_scale(&mut self, arg1: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScale_setZScale(self as *mut ::graphics_scale::GraphicsScale, arg1) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsScale::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsScale_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsScale::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsScale_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsScale::xScale() const```</span>
  ///
  ///
  pub fn x_scale(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScale_xScale(self as *const ::graphics_scale::GraphicsScale) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsScale::yScale() const```</span>
  ///
  ///
  pub fn y_scale(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScale_yScale(self as *const ::graphics_scale::GraphicsScale) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsScale::zScale() const```</span>
  ///
  ///
  pub fn z_scale(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsScale_zScale(self as *const ::graphics_scale::GraphicsScale) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scale::GraphicsScale {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsScale_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsScale`.
  pub struct Signals<'a>(&'a ::graphics_scale::GraphicsScale);
  /// Represents a built-in Qt signal `QGraphicsScale::originChanged`.
  ///
  /// An object of this type can be created from `GraphicsScale` with `object.signals().origin_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScale` object.
  pub struct OriginChanged<'a>(&'a ::graphics_scale::GraphicsScale);
  impl<'a> ::qt_core::connection::Receiver for OriginChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2originChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for OriginChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsScale::yScaleChanged`.
  ///
  /// An object of this type can be created from `GraphicsScale` with `object.signals().y_scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScale` object.
  pub struct YScaleChanged<'a>(&'a ::graphics_scale::GraphicsScale);
  impl<'a> ::qt_core::connection::Receiver for YScaleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yScaleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YScaleChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsScale::scaleChanged`.
  ///
  /// An object of this type can be created from `GraphicsScale` with `object.signals().scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScale` object.
  pub struct ScaleChanged<'a>(&'a ::graphics_scale::GraphicsScale);
  impl<'a> ::qt_core::connection::Receiver for ScaleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scaleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScaleChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsScale::xScaleChanged`.
  ///
  /// An object of this type can be created from `GraphicsScale` with `object.signals().x_scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScale` object.
  pub struct XScaleChanged<'a>(&'a ::graphics_scale::GraphicsScale);
  impl<'a> ::qt_core::connection::Receiver for XScaleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xScaleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XScaleChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsScale::zScaleChanged`.
  ///
  /// An object of this type can be created from `GraphicsScale` with `object.signals().z_scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScale` object.
  pub struct ZScaleChanged<'a>(&'a ::graphics_scale::GraphicsScale);
  impl<'a> ::qt_core::connection::Receiver for ZScaleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2zScaleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ZScaleChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsScale::originChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn origin_changed(&self) -> OriginChanged {
      OriginChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScale::yScaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_scale_changed(&self) -> YScaleChanged {
      YScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScale::scaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_changed(&self) -> ScaleChanged {
      ScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScale::xScaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_scale_changed(&self) -> XScaleChanged {
      XScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsScale::zScaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn z_scale_changed(&self) -> ZScaleChanged {
      ZScaleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsScale`.
  pub struct Slots<'a>(&'a ::graphics_scale::GraphicsScale);
  /// Represents a built-in Qt slot `QGraphicsScale::update`.
  ///
  /// An object of this type can be created from `GraphicsScale` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsScale` object.
  pub struct Update<'a>(&'a ::graphics_scale::GraphicsScale);
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
    /// Returns an object representing a built-in Qt slot `QGraphicsScale::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::graphics_scale::GraphicsScale {
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

impl ::cpp_utils::DynamicCast<::graphics_scale::GraphicsScale> for ::graphics_transform::GraphicsTransform {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scale::GraphicsScale> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_G_dynamic_cast_QGraphicsScale_ptr(self as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scale::GraphicsScale> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_G_dynamic_cast_QGraphicsScale_ptr(self as *const ::graphics_transform::GraphicsTransform as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_scale::GraphicsScale {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QObject_ptr(self as *mut ::graphics_scale::GraphicsScale)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QObject_ptr(self as *const ::graphics_scale::GraphicsScale as *mut ::graphics_scale::GraphicsScale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_transform::GraphicsTransform> for ::graphics_scale::GraphicsScale {
  fn static_cast_mut(&mut self) -> &mut ::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsTransform_ptr(self as *mut ::graphics_scale::GraphicsScale) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsTransform_ptr(self as *const ::graphics_scale::GraphicsScale as *mut ::graphics_scale::GraphicsScale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scale::GraphicsScale> for ::graphics_transform::GraphicsTransform {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scale::GraphicsScale {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QGraphicsTransform(self as *mut ::graphics_transform::GraphicsTransform);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_scale::GraphicsScale {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QGraphicsTransform(self as *const ::graphics_transform::GraphicsTransform as *mut ::graphics_transform::GraphicsTransform);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scale::GraphicsScale> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scale::GraphicsScale {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_scale::GraphicsScale {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsScale_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_scale::GraphicsScale {
  type Target = ::graphics_transform::GraphicsTransform;
  fn deref(&self) -> &::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsTransform_ptr(self as *const ::graphics_scale::GraphicsScale as *mut ::graphics_scale::GraphicsScale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scale::GraphicsScale {
  fn deref_mut(&mut self) -> &mut ::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsScale_G_static_cast_QGraphicsTransform_ptr(self as *mut ::graphics_scale::GraphicsScale) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

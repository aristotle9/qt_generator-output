/// C++ type: <span style='color: green;'>```QGraphicsRotation```</span>
#[repr(C)]
pub struct GraphicsRotation(u8);

impl GraphicsRotation {
  /// C++ method: <span style='color: green;'>```double QGraphicsRotation::angle() const```</span>
  ///
  ///
  pub fn angle(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_angle(self as *const ::graphics_rotation::GraphicsRotation) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsRotation::applyTo(QMatrix4x4* matrix) const```</span>
  ///
  ///
  pub unsafe fn apply_to(&self, matrix: *mut ::qt_gui::matrix_4x4::Matrix4X4) {
    ::ffi::qt_widgets_c_QGraphicsRotation_applyTo(self as *const ::graphics_rotation::GraphicsRotation, matrix)
  }

  /// C++ method: <span style='color: green;'>```QVector3D QGraphicsRotation::axis() const```</span>
  ///
  ///
  pub fn axis(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsRotation_axis_as_ptr(self as *const ::graphics_rotation::GraphicsRotation)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGraphicsRotation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_metaObject(self as *const ::graphics_rotation::GraphicsRotation) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRotation::QGraphicsRotation()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_rotation::GraphicsRotation> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRotation::QGraphicsRotation(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::graphics_rotation::GraphicsRotation> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsRotation_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector3D QGraphicsRotation::origin() const```</span>
  ///
  ///
  pub fn origin(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsRotation_origin_as_ptr(self as *const ::graphics_rotation::GraphicsRotation)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsRotation::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGraphicsRotation_qt_metacall(self as *mut ::graphics_rotation::GraphicsRotation,
                                                      arg1 as *const ::qt_core::meta_object::Call,
                                                      arg2,
                                                      arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGraphicsRotation::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGraphicsRotation_qt_metacast(self as *mut ::graphics_rotation::GraphicsRotation, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsRotation::setAngle(double arg1)```</span>
  ///
  ///
  pub fn set_angle(&mut self, arg1: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_setAngle(self as *mut ::graphics_rotation::GraphicsRotation, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsRotation::setAxis```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_axis(&mut self, &::qt_core::qt::Axis) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsRotation::setAxis(Qt::Axis axis)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_axis(&mut self, &::qt_gui::vector_3d::Vector3D) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsRotation::setAxis(const QVector3D& axis)```</span>
  ///
  ///
  pub fn set_axis<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsRotationSetAxisArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsRotation::setOrigin(const QVector3D& point)```</span>
  ///
  ///
  pub fn set_origin(&mut self, point: &::qt_gui::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsRotation_setOrigin(self as *mut ::graphics_rotation::GraphicsRotation,
                                                      point as *const ::qt_gui::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsRotation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsRotation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGraphicsRotation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGraphicsRotation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_rotation::GraphicsRotation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsRotation_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GraphicsRotation`.
  pub struct Signals<'a>(&'a ::graphics_rotation::GraphicsRotation);
  /// Represents a built-in Qt signal `QGraphicsRotation::originChanged`.
  ///
  /// An object of this type can be created from `GraphicsRotation` with `object.signals().origin_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsRotation` object.
  pub struct OriginChanged<'a>(&'a ::graphics_rotation::GraphicsRotation);
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
  /// Represents a built-in Qt signal `QGraphicsRotation::axisChanged`.
  ///
  /// An object of this type can be created from `GraphicsRotation` with `object.signals().axis_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsRotation` object.
  pub struct AxisChanged<'a>(&'a ::graphics_rotation::GraphicsRotation);
  impl<'a> ::qt_core::connection::Receiver for AxisChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2axisChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AxisChanged<'a> {}
  /// Represents a built-in Qt signal `QGraphicsRotation::angleChanged`.
  ///
  /// An object of this type can be created from `GraphicsRotation` with `object.signals().angle_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsRotation` object.
  pub struct AngleChanged<'a>(&'a ::graphics_rotation::GraphicsRotation);
  impl<'a> ::qt_core::connection::Receiver for AngleChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2angleChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AngleChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QGraphicsRotation::originChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn origin_changed(&self) -> OriginChanged {
      OriginChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsRotation::axisChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn axis_changed(&self) -> AxisChanged {
      AxisChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QGraphicsRotation::angleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn angle_changed(&self) -> AngleChanged {
      AngleChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GraphicsRotation`.
  pub struct Slots<'a>(&'a ::graphics_rotation::GraphicsRotation);
  /// Represents a built-in Qt slot `QGraphicsRotation::update`.
  ///
  /// An object of this type can be created from `GraphicsRotation` with `object.slots().update()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GraphicsRotation` object.
  pub struct Update<'a>(&'a ::graphics_rotation::GraphicsRotation);
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
    /// Returns an object representing a built-in Qt slot `QGraphicsRotation::update`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update(&self) -> Update {
      Update(self.0)
    }
  }
  impl ::graphics_rotation::GraphicsRotation {
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

impl ::cpp_utils::DynamicCast<::graphics_rotation::GraphicsRotation> for ::graphics_transform::GraphicsTransform {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_rotation::GraphicsRotation> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_dynamic_cast_QGraphicsRotation_ptr(self as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_rotation::GraphicsRotation> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_dynamic_cast_QGraphicsRotation_ptr(self as *const ::graphics_transform::GraphicsTransform as *mut ::graphics_transform::GraphicsTransform) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::graphics_rotation::GraphicsRotation {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QObject_ptr(self as *mut ::graphics_rotation::GraphicsRotation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QObject_ptr(self as *const ::graphics_rotation::GraphicsRotation as *mut ::graphics_rotation::GraphicsRotation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_transform::GraphicsTransform> for ::graphics_rotation::GraphicsRotation {
  fn static_cast_mut(&mut self) -> &mut ::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsTransform_ptr(self as *mut ::graphics_rotation::GraphicsRotation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsTransform_ptr(self as *const ::graphics_rotation::GraphicsRotation as *mut ::graphics_rotation::GraphicsRotation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_rotation::GraphicsRotation> for ::graphics_transform::GraphicsTransform {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_rotation::GraphicsRotation {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QGraphicsTransform(self as *mut ::graphics_transform::GraphicsTransform);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_rotation::GraphicsRotation {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QGraphicsTransform(self as *const ::graphics_transform::GraphicsTransform as *mut ::graphics_transform::GraphicsTransform);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_rotation::GraphicsRotation> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_rotation::GraphicsRotation {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_rotation::GraphicsRotation {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsRotation_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_rotation::GraphicsRotation {
  type Target = ::graphics_transform::GraphicsTransform;
  fn deref(&self) -> &::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsTransform_ptr(self as *const ::graphics_rotation::GraphicsRotation as *mut ::graphics_rotation::GraphicsRotation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_rotation::GraphicsRotation {
  fn deref_mut(&mut self) -> &mut ::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_G_static_cast_QGraphicsTransform_ptr(self as *mut ::graphics_rotation::GraphicsRotation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsRotation::set_axis](../struct.GraphicsRotation.html#method.set_axis) method.
  pub trait GraphicsRotationSetAxisArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_rotation::GraphicsRotation) -> ();
  }
  impl<'largs> GraphicsRotationSetAxisArgs<'largs> for &'largs ::qt_gui::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::graphics_rotation::GraphicsRotation) -> () {
      let axis = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_setAxis_QVector3D(original_self as *mut ::graphics_rotation::GraphicsRotation, axis as *const ::qt_gui::vector_3d::Vector3D) }
    }
  }
  impl<'largs> GraphicsRotationSetAxisArgs<'largs> for &'largs ::qt_core::qt::Axis {
    fn exec(self, original_self: &'largs mut ::graphics_rotation::GraphicsRotation) -> () {
      let axis = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsRotation_setAxis_Qt_Axis(original_self as *mut ::graphics_rotation::GraphicsRotation, axis as *const ::qt_core::qt::Axis) }
    }
  }
}

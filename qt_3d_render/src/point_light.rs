/// C++ type: <span style='color: green;'>```Qt3DRender::QPointLight```</span>
#[repr(C)]
pub struct PointLight(u8);

impl PointLight {
  /// C++ method: <span style='color: green;'>```float Qt3DRender::QPointLight::constantAttenuation() const```</span>
  ///
  ///
  pub fn constant_attenuation(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_constantAttenuation(self as *const ::point_light::PointLight)
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QPointLight::linearAttenuation() const```</span>
  ///
  ///
  pub fn linear_attenuation(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_linearAttenuation(self as *const ::point_light::PointLight) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QPointLight::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_metaObject(self as *const ::point_light::PointLight) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPointLight::QPointLight()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::point_light::PointLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPointLight::QPointLight(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::point_light::PointLight> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QPointLight::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_qt_metacall(self as *mut ::point_light::PointLight,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QPointLight::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_qt_metacast(self as *mut ::point_light::PointLight, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QPointLight::quadraticAttenuation() const```</span>
  ///
  ///
  pub fn quadratic_attenuation(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_quadraticAttenuation(self as *const ::point_light::PointLight)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPointLight::setConstantAttenuation(float value)```</span>
  ///
  ///
  pub fn set_constant_attenuation(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_setConstantAttenuation(self as *mut ::point_light::PointLight, value)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPointLight::setLinearAttenuation(float value)```</span>
  ///
  ///
  pub fn set_linear_attenuation(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_setLinearAttenuation(self as *mut ::point_light::PointLight, value)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPointLight::setQuadraticAttenuation(float value)```</span>
  ///
  ///
  pub fn set_quadratic_attenuation(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_setQuadraticAttenuation(self as *mut ::point_light::PointLight,
                                                                           value)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPointLight::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPointLight::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::point_light::PointLight {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QPointLight_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PointLight`.
  pub struct Signals<'a>(&'a ::point_light::PointLight);
  /// Represents a built-in Qt signal `Qt3DRender::QPointLight::colorChanged`.
  ///
  /// An object of this type can be created from `PointLight` with `object.signals().color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct ColorChanged<'a>(&'a ::point_light::PointLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QPointLight::linearAttenuationChanged`.
  ///
  /// An object of this type can be created from `PointLight` with `object.signals().linear_attenuation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct LinearAttenuationChanged<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for LinearAttenuationChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2linearAttenuationChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LinearAttenuationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPointLight::constantAttenuationChanged`.
  ///
  /// An object of this type can be created from `PointLight` with `object.signals().constant_attenuation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct ConstantAttenuationChanged<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for ConstantAttenuationChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2constantAttenuationChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ConstantAttenuationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPointLight::intensityChanged`.
  ///
  /// An object of this type can be created from `PointLight` with `object.signals().intensity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct IntensityChanged<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for IntensityChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2intensityChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IntensityChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPointLight::quadraticAttenuationChanged`.
  ///
  /// An object of this type can be created from `PointLight` with `object.signals().quadratic_attenuation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct QuadraticAttenuationChanged<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for QuadraticAttenuationChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2quadraticAttenuationChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for QuadraticAttenuationChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPointLight::colorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_changed(&self) -> ColorChanged {
      ColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPointLight::linearAttenuationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn linear_attenuation_changed(&self) -> LinearAttenuationChanged {
      LinearAttenuationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPointLight::constantAttenuationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn constant_attenuation_changed(&self) -> ConstantAttenuationChanged {
      ConstantAttenuationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPointLight::intensityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn intensity_changed(&self) -> IntensityChanged {
      IntensityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPointLight::quadraticAttenuationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn quadratic_attenuation_changed(&self) -> QuadraticAttenuationChanged {
      QuadraticAttenuationChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PointLight`.
  pub struct Slots<'a>(&'a ::point_light::PointLight);
  /// Represents a built-in Qt slot `Qt3DRender::QPointLight::setIntensity`.
  ///
  /// An object of this type can be created from `PointLight` with `object.slots().set_intensity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct SetIntensity<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for SetIntensity<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIntensity(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPointLight::setLinearAttenuation`.
  ///
  /// An object of this type can be created from `PointLight` with `object.slots().set_linear_attenuation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct SetLinearAttenuation<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for SetLinearAttenuation<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLinearAttenuation(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPointLight::setConstantAttenuation`.
  ///
  /// An object of this type can be created from `PointLight` with `object.slots().set_constant_attenuation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct SetConstantAttenuation<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for SetConstantAttenuation<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setConstantAttenuation(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPointLight::setQuadraticAttenuation`.
  ///
  /// An object of this type can be created from `PointLight` with `object.slots().set_quadratic_attenuation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct SetQuadraticAttenuation<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for SetQuadraticAttenuation<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setQuadraticAttenuation(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPointLight::setColor`.
  ///
  /// An object of this type can be created from `PointLight` with `object.slots().set_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PointLight` object.
  pub struct SetColor<'a>(&'a ::point_light::PointLight);
  impl<'a> ::qt_core::connection::Receiver for SetColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setColor(const QColor&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPointLight::setIntensity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_intensity(&self) -> SetIntensity {
      SetIntensity(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPointLight::setLinearAttenuation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_linear_attenuation(&self) -> SetLinearAttenuation {
      SetLinearAttenuation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPointLight::setConstantAttenuation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_constant_attenuation(&self) -> SetConstantAttenuation {
      SetConstantAttenuation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPointLight::setQuadraticAttenuation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_quadratic_attenuation(&self) -> SetQuadraticAttenuation {
      SetQuadraticAttenuation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPointLight::setColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_color(&self) -> SetColor {
      SetColor(self.0)
    }
  }
  impl ::point_light::PointLight {
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

impl ::cpp_utils::DynamicCast<::point_light::PointLight> for ::abstract_light::AbstractLight {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::point_light::PointLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_dynamic_cast_Qt3DRender_QPointLight_ptr(self as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::point_light::PointLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_dynamic_cast_Qt3DRender_QPointLight_ptr(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::point_light::PointLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::point_light::PointLight)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::point_light::PointLight as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::point_light::PointLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::point_light::PointLight)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::point_light::PointLight as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_light::AbstractLight> for ::point_light::PointLight {
  fn static_cast_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *const ::point_light::PointLight as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::point_light::PointLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_QObject_ptr(self as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_QObject_ptr(self as *const ::point_light::PointLight as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::point_light::PointLight> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::point_light::PointLight> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::point_light::PointLight> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::point_light::PointLight> for ::abstract_light::AbstractLight {
  unsafe fn static_cast_mut(&mut self) -> &mut ::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DRender_QAbstractLight(self as *mut ::abstract_light::AbstractLight);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::point_light::PointLight {
    let ffi_result = ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DRender_QAbstractLight(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::point_light::PointLight {
  type Target = ::abstract_light::AbstractLight;
  fn deref(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *const ::point_light::PointLight as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::point_light::PointLight {
  fn deref_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *mut ::point_light::PointLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

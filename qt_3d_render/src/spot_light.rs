/// C++ type: <span style='color: green;'>```Qt3DRender::QSpotLight```</span>
#[repr(C)]
pub struct SpotLight(u8);

impl SpotLight {
  /// C++ method: <span style='color: green;'>```float Qt3DRender::QSpotLight::constantAttenuation() const```</span>
  ///
  ///
  pub fn constant_attenuation(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_constantAttenuation(self as *const ::spot_light::SpotLight) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QSpotLight::cutOffAngle() const```</span>
  ///
  ///
  pub fn cut_off_angle(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_cutOffAngle(self as *const ::spot_light::SpotLight) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QSpotLight::linearAttenuation() const```</span>
  ///
  ///
  pub fn linear_attenuation(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_linearAttenuation(self as *const ::spot_light::SpotLight) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D Qt3DRender::QSpotLight::localDirection() const```</span>
  ///
  ///
  pub fn local_direction(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_3d::Vector3D> {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_localDirection_as_ptr(self as *const ::spot_light::SpotLight)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QSpotLight::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_metaObject(self as *const ::spot_light::SpotLight) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QSpotLight::QSpotLight()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::spot_light::SpotLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QSpotLight::QSpotLight(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::spot_light::SpotLight> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QSpotLight::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_qt_metacall(self as *mut ::spot_light::SpotLight,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QSpotLight::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_qt_metacast(self as *mut ::spot_light::SpotLight, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QSpotLight::quadraticAttenuation() const```</span>
  ///
  ///
  pub fn quadratic_attenuation(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_quadraticAttenuation(self as *const ::spot_light::SpotLight) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSpotLight::setConstantAttenuation(float value)```</span>
  ///
  ///
  pub fn set_constant_attenuation(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_setConstantAttenuation(self as *mut ::spot_light::SpotLight, value)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSpotLight::setCutOffAngle(float cutOffAngle)```</span>
  ///
  ///
  pub fn set_cut_off_angle(&mut self, cut_off_angle: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_setCutOffAngle(self as *mut ::spot_light::SpotLight, cut_off_angle)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSpotLight::setLinearAttenuation(float value)```</span>
  ///
  ///
  pub fn set_linear_attenuation(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_setLinearAttenuation(self as *mut ::spot_light::SpotLight, value)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSpotLight::setLocalDirection(const QVector3D& localDirection)```</span>
  ///
  ///
  pub fn set_local_direction(&mut self, local_direction: &::qt_gui::vector_3d::Vector3D) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_setLocalDirection(self as *mut ::spot_light::SpotLight, local_direction as *const ::qt_gui::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSpotLight::setQuadraticAttenuation(float value)```</span>
  ///
  ///
  pub fn set_quadratic_attenuation(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_setQuadraticAttenuation(self as *mut ::spot_light::SpotLight, value)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QSpotLight::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QSpotLight::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::spot_light::SpotLight {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QSpotLight_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SpotLight`.
  pub struct Signals<'a>(&'a ::spot_light::SpotLight);
  /// Represents a built-in Qt signal `Qt3DRender::QSpotLight::cutOffAngleChanged`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.signals().cut_off_angle_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct CutOffAngleChanged<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for CutOffAngleChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cutOffAngleChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CutOffAngleChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QSpotLight::intensityChanged`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.signals().intensity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct IntensityChanged<'a>(&'a ::spot_light::SpotLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QSpotLight::localDirectionChanged`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.signals().local_direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct LocalDirectionChanged<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for LocalDirectionChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2localDirectionChanged(const QVector3D&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LocalDirectionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QSpotLight::colorChanged`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.signals().color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct ColorChanged<'a>(&'a ::spot_light::SpotLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QSpotLight::quadraticAttenuationChanged`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.signals().quadratic_attenuation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct QuadraticAttenuationChanged<'a>(&'a ::spot_light::SpotLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QSpotLight::constantAttenuationChanged`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.signals().constant_attenuation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct ConstantAttenuationChanged<'a>(&'a ::spot_light::SpotLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QSpotLight::linearAttenuationChanged`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.signals().linear_attenuation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct LinearAttenuationChanged<'a>(&'a ::spot_light::SpotLight);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSpotLight::cutOffAngleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cut_off_angle_changed(&self) -> CutOffAngleChanged {
      CutOffAngleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSpotLight::intensityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn intensity_changed(&self) -> IntensityChanged {
      IntensityChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSpotLight::localDirectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn local_direction_changed(&self) -> LocalDirectionChanged {
      LocalDirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSpotLight::colorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_changed(&self) -> ColorChanged {
      ColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSpotLight::quadraticAttenuationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn quadratic_attenuation_changed(&self) -> QuadraticAttenuationChanged {
      QuadraticAttenuationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSpotLight::constantAttenuationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn constant_attenuation_changed(&self) -> ConstantAttenuationChanged {
      ConstantAttenuationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSpotLight::linearAttenuationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn linear_attenuation_changed(&self) -> LinearAttenuationChanged {
      LinearAttenuationChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SpotLight`.
  pub struct Slots<'a>(&'a ::spot_light::SpotLight);
  /// Represents a built-in Qt slot `Qt3DRender::QSpotLight::setQuadraticAttenuation`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.slots().set_quadratic_attenuation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct SetQuadraticAttenuation<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for SetQuadraticAttenuation<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setQuadraticAttenuation(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSpotLight::setLinearAttenuation`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.slots().set_linear_attenuation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct SetLinearAttenuation<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for SetLinearAttenuation<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLinearAttenuation(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSpotLight::setCutOffAngle`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.slots().set_cut_off_angle()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct SetCutOffAngle<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for SetCutOffAngle<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCutOffAngle(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSpotLight::setColor`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.slots().set_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct SetColor<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for SetColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSpotLight::setConstantAttenuation`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.slots().set_constant_attenuation()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct SetConstantAttenuation<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for SetConstantAttenuation<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setConstantAttenuation(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSpotLight::setIntensity`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.slots().set_intensity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct SetIntensity<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for SetIntensity<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIntensity(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSpotLight::setLocalDirection`.
  ///
  /// An object of this type can be created from `SpotLight` with `object.slots().set_local_direction()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SpotLight` object.
  pub struct SetLocalDirection<'a>(&'a ::spot_light::SpotLight);
  impl<'a> ::qt_core::connection::Receiver for SetLocalDirection<'a> {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLocalDirection(const QVector3D&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSpotLight::setQuadraticAttenuation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_quadratic_attenuation(&self) -> SetQuadraticAttenuation {
      SetQuadraticAttenuation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSpotLight::setLinearAttenuation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_linear_attenuation(&self) -> SetLinearAttenuation {
      SetLinearAttenuation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSpotLight::setCutOffAngle`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_cut_off_angle(&self) -> SetCutOffAngle {
      SetCutOffAngle(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSpotLight::setColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_color(&self) -> SetColor {
      SetColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSpotLight::setConstantAttenuation`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_constant_attenuation(&self) -> SetConstantAttenuation {
      SetConstantAttenuation(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSpotLight::setIntensity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_intensity(&self) -> SetIntensity {
      SetIntensity(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSpotLight::setLocalDirection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_local_direction(&self) -> SetLocalDirection {
      SetLocalDirection(self.0)
    }
  }
  impl ::spot_light::SpotLight {
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

impl ::cpp_utils::DynamicCast<::spot_light::SpotLight> for ::abstract_light::AbstractLight {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::spot_light::SpotLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_dynamic_cast_Qt3DRender_QSpotLight_ptr(self as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::spot_light::SpotLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_dynamic_cast_Qt3DRender_QSpotLight_ptr(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::spot_light::SpotLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::spot_light::SpotLight)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::spot_light::SpotLight as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::spot_light::SpotLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::spot_light::SpotLight)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::spot_light::SpotLight as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_light::AbstractLight> for ::spot_light::SpotLight {
  fn static_cast_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *const ::spot_light::SpotLight as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::spot_light::SpotLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_QObject_ptr(self as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_QObject_ptr(self as *const ::spot_light::SpotLight as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spot_light::SpotLight> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spot_light::SpotLight> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spot_light::SpotLight> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::spot_light::SpotLight> for ::abstract_light::AbstractLight {
  unsafe fn static_cast_mut(&mut self) -> &mut ::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DRender_QAbstractLight(self as *mut ::abstract_light::AbstractLight);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::spot_light::SpotLight {
    let ffi_result = ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DRender_QAbstractLight(self as *const ::abstract_light::AbstractLight as *mut ::abstract_light::AbstractLight);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::spot_light::SpotLight {
  type Target = ::abstract_light::AbstractLight;
  fn deref(&self) -> &::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *const ::spot_light::SpotLight as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::spot_light::SpotLight {
  fn deref_mut(&mut self) -> &mut ::abstract_light::AbstractLight {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(self as *mut ::spot_light::SpotLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// C++ type: <span style='color: green;'>```Qt3DExtras::QMorphPhongMaterial```</span>
#[repr(C)]
pub struct MorphPhongMaterial(u8);

impl MorphPhongMaterial {
  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QMorphPhongMaterial::ambient() const```</span>
  ///
  ///
  pub fn ambient(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_ambient_to_output(self as *const ::morph_phong_material::MorphPhongMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QMorphPhongMaterial::diffuse() const```</span>
  ///
  ///
  pub fn diffuse(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_diffuse_to_output(self as *const ::morph_phong_material::MorphPhongMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QMorphPhongMaterial::interpolator() const```</span>
  ///
  ///
  pub fn interpolator(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_interpolator(self as *const ::morph_phong_material::MorphPhongMaterial) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QMorphPhongMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_metaObject(self as *const ::morph_phong_material::MorphPhongMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QMorphPhongMaterial::QMorphPhongMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::morph_phong_material::MorphPhongMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QMorphPhongMaterial::QMorphPhongMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::morph_phong_material::MorphPhongMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QMorphPhongMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_qt_metacall(self as *mut ::morph_phong_material::MorphPhongMaterial, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QMorphPhongMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_qt_metacast(self as *mut ::morph_phong_material::MorphPhongMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMorphPhongMaterial::setAmbient(const QColor& ambient)```</span>
  ///
  ///
  pub fn set_ambient(&mut self, ambient: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setAmbient(self as *mut ::morph_phong_material::MorphPhongMaterial, ambient as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMorphPhongMaterial::setDiffuse(const QColor& diffuse)```</span>
  ///
  ///
  pub fn set_diffuse(&mut self, diffuse: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setDiffuse(self as *mut ::morph_phong_material::MorphPhongMaterial, diffuse as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMorphPhongMaterial::setInterpolator(float interpolator)```</span>
  ///
  ///
  pub fn set_interpolator(&mut self, interpolator: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setInterpolator(self as *mut ::morph_phong_material::MorphPhongMaterial, interpolator) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMorphPhongMaterial::setShininess(float shininess)```</span>
  ///
  ///
  pub fn set_shininess(&mut self, shininess: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setShininess(self as *mut ::morph_phong_material::MorphPhongMaterial, shininess) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMorphPhongMaterial::setSpecular(const QColor& specular)```</span>
  ///
  ///
  pub fn set_specular(&mut self, specular: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_setSpecular(self as *mut ::morph_phong_material::MorphPhongMaterial, specular as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QMorphPhongMaterial::shininess() const```</span>
  ///
  ///
  pub fn shininess(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_shininess(self as *const ::morph_phong_material::MorphPhongMaterial) }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QMorphPhongMaterial::specular() const```</span>
  ///
  ///
  pub fn specular(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_specular_to_output(self as *const ::morph_phong_material::MorphPhongMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QMorphPhongMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QMorphPhongMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::morph_phong_material::MorphPhongMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QMorphPhongMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MorphPhongMaterial`.
  pub struct Signals<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::diffuseChanged`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.signals().diffuse_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct DiffuseChanged<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for DiffuseChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2diffuseChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DiffuseChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::ambientChanged`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.signals().ambient_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct AmbientChanged<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for AmbientChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2ambientChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AmbientChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct EffectChanged<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for EffectChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2effectChanged(Qt3DRender::QEffect*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EffectChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::shininessChanged`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.signals().shininess_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct ShininessChanged<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for ShininessChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shininessChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShininessChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::specularChanged`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.signals().specular_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct SpecularChanged<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for SpecularChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2specularChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SpecularChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::interpolatorChanged`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.signals().interpolator_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct InterpolatorChanged<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for InterpolatorChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2interpolatorChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for InterpolatorChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::diffuseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn diffuse_changed(&self) -> DiffuseChanged {
      DiffuseChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::ambientChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ambient_changed(&self) -> AmbientChanged {
      AmbientChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::shininessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shininess_changed(&self) -> ShininessChanged {
      ShininessChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::specularChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn specular_changed(&self) -> SpecularChanged {
      SpecularChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMorphPhongMaterial::interpolatorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn interpolator_changed(&self) -> InterpolatorChanged {
      InterpolatorChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MorphPhongMaterial`.
  pub struct Slots<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setShininess`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.slots().set_shininess()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct SetShininess<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetShininess<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShininess(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setAmbient`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.slots().set_ambient()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct SetAmbient<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetAmbient<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAmbient(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setSpecular`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.slots().set_specular()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct SetSpecular<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetSpecular<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSpecular(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setDiffuse`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.slots().set_diffuse()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct SetDiffuse<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetDiffuse<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDiffuse(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setInterpolator`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.slots().set_interpolator()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct SetInterpolator<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetInterpolator<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setInterpolator(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setEffect`.
  ///
  /// An object of this type can be created from `MorphPhongMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MorphPhongMaterial` object.
  pub struct SetEffect<'a>(&'a ::morph_phong_material::MorphPhongMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setShininess`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shininess(&self) -> SetShininess {
      SetShininess(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setAmbient`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_ambient(&self) -> SetAmbient {
      SetAmbient(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setSpecular`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_specular(&self) -> SetSpecular {
      SetSpecular(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setDiffuse`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_diffuse(&self) -> SetDiffuse {
      SetDiffuse(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setInterpolator`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_interpolator(&self) -> SetInterpolator {
      SetInterpolator(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMorphPhongMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
  }
  impl ::morph_phong_material::MorphPhongMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::morph_phong_material::MorphPhongMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::morph_phong_material::MorphPhongMaterial as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::morph_phong_material::MorphPhongMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::morph_phong_material::MorphPhongMaterial as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::morph_phong_material::MorphPhongMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::morph_phong_material::MorphPhongMaterial as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::morph_phong_material::MorphPhongMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_QObject_ptr(self as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_QObject_ptr(self as *const ::morph_phong_material::MorphPhongMaterial as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::morph_phong_material::MorphPhongMaterial> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::morph_phong_material::MorphPhongMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::morph_phong_material::MorphPhongMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::morph_phong_material::MorphPhongMaterial> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::morph_phong_material::MorphPhongMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::morph_phong_material::MorphPhongMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::morph_phong_material::MorphPhongMaterial> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::morph_phong_material::MorphPhongMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::morph_phong_material::MorphPhongMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::morph_phong_material::MorphPhongMaterial> for ::qt_3d_render::material::Material {
unsafe fn static_cast_mut(&mut self) -> &mut ::morph_phong_material::MorphPhongMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::morph_phong_material::MorphPhongMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DExtras_QMorphPhongMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::morph_phong_material::MorphPhongMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::morph_phong_material::MorphPhongMaterial as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::morph_phong_material::MorphPhongMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMorphPhongMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::morph_phong_material::MorphPhongMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

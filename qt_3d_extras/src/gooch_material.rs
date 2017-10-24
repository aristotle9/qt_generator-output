/// C++ type: <span style='color: green;'>```Qt3DExtras::QGoochMaterial```</span>
#[repr(C)]
pub struct GoochMaterial(u8);

impl GoochMaterial {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QGoochMaterial::alpha() const```</span>
  ///
  ///
  pub fn alpha(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_alpha(self as *const ::gooch_material::GoochMaterial) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QGoochMaterial::beta() const```</span>
  ///
  ///
  pub fn beta(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_beta(self as *const ::gooch_material::GoochMaterial) }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QGoochMaterial::cool() const```</span>
  ///
  ///
  pub fn cool(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_cool_to_output(self as *const ::gooch_material::GoochMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QGoochMaterial::diffuse() const```</span>
  ///
  ///
  pub fn diffuse(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_diffuse_to_output(self as *const ::gooch_material::GoochMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QGoochMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_metaObject(self as *const ::gooch_material::GoochMaterial)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QGoochMaterial::QGoochMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::gooch_material::GoochMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QGoochMaterial::QGoochMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::gooch_material::GoochMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QGoochMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_qt_metacall(self as *mut ::gooch_material::GoochMaterial,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QGoochMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_qt_metacast(self as *mut ::gooch_material::GoochMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QGoochMaterial::setAlpha(float alpha)```</span>
  ///
  ///
  pub fn set_alpha(&mut self, alpha: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setAlpha(self as *mut ::gooch_material::GoochMaterial, alpha)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QGoochMaterial::setBeta(float beta)```</span>
  ///
  ///
  pub fn set_beta(&mut self, beta: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setBeta(self as *mut ::gooch_material::GoochMaterial, beta)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QGoochMaterial::setCool(const QColor& cool)```</span>
  ///
  ///
  pub fn set_cool(&mut self, cool: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setCool(self as *mut ::gooch_material::GoochMaterial,
                                                              cool as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QGoochMaterial::setDiffuse(const QColor& diffuse)```</span>
  ///
  ///
  pub fn set_diffuse(&mut self, diffuse: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setDiffuse(self as *mut ::gooch_material::GoochMaterial,
                                                                 diffuse as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QGoochMaterial::setShininess(float shininess)```</span>
  ///
  ///
  pub fn set_shininess(&mut self, shininess: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setShininess(self as *mut ::gooch_material::GoochMaterial,
                                                                   shininess)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QGoochMaterial::setSpecular(const QColor& specular)```</span>
  ///
  ///
  pub fn set_specular(&mut self, specular: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setSpecular(self as *mut ::gooch_material::GoochMaterial,
                                                                  specular as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QGoochMaterial::setWarm(const QColor& warm)```</span>
  ///
  ///
  pub fn set_warm(&mut self, warm: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_setWarm(self as *mut ::gooch_material::GoochMaterial,
                                                              warm as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QGoochMaterial::shininess() const```</span>
  ///
  ///
  pub fn shininess(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_shininess(self as *const ::gooch_material::GoochMaterial) }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QGoochMaterial::specular() const```</span>
  ///
  ///
  pub fn specular(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_specular_to_output(self as *const ::gooch_material::GoochMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QGoochMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QGoochMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QGoochMaterial::warm() const```</span>
  ///
  ///
  pub fn warm(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_warm_to_output(self as *const ::gooch_material::GoochMaterial, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::gooch_material::GoochMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QGoochMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GoochMaterial`.
  pub struct Signals<'a>(&'a ::gooch_material::GoochMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct EffectChanged<'a>(&'a ::gooch_material::GoochMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::specularChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().specular_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SpecularChanged<'a>(&'a ::gooch_material::GoochMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::betaChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().beta_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct BetaChanged<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for BetaChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2betaChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BetaChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::alphaChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().alpha_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct AlphaChanged<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for AlphaChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2alphaChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AlphaChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::warmChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().warm_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct WarmChanged<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for WarmChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2warmChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WarmChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::shininessChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().shininess_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct ShininessChanged<'a>(&'a ::gooch_material::GoochMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::coolChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().cool_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct CoolChanged<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for CoolChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2coolChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CoolChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QGoochMaterial::diffuseChanged`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.signals().diffuse_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct DiffuseChanged<'a>(&'a ::gooch_material::GoochMaterial);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::specularChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn specular_changed(&self) -> SpecularChanged {
      SpecularChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::betaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn beta_changed(&self) -> BetaChanged {
      BetaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::alphaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn alpha_changed(&self) -> AlphaChanged {
      AlphaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::warmChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn warm_changed(&self) -> WarmChanged {
      WarmChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::shininessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shininess_changed(&self) -> ShininessChanged {
      ShininessChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::coolChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cool_changed(&self) -> CoolChanged {
      CoolChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QGoochMaterial::diffuseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn diffuse_changed(&self) -> DiffuseChanged {
      DiffuseChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GoochMaterial`.
  pub struct Slots<'a>(&'a ::gooch_material::GoochMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setSpecular`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_specular()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetSpecular<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetSpecular<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSpecular(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setAlpha`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_alpha()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetAlpha<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetAlpha<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAlpha(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setDiffuse`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_diffuse()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetDiffuse<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetDiffuse<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDiffuse(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setCool`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_cool()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetCool<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetCool<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCool(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setWarm`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_warm()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetWarm<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetWarm<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWarm(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setShininess`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_shininess()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetShininess<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetShininess<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShininess(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setEffect`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetEffect<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QGoochMaterial::setBeta`.
  ///
  /// An object of this type can be created from `GoochMaterial` with `object.slots().set_beta()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GoochMaterial` object.
  pub struct SetBeta<'a>(&'a ::gooch_material::GoochMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetBeta<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBeta(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setSpecular`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_specular(&self) -> SetSpecular {
      SetSpecular(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setAlpha`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_alpha(&self) -> SetAlpha {
      SetAlpha(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setDiffuse`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_diffuse(&self) -> SetDiffuse {
      SetDiffuse(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setCool`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_cool(&self) -> SetCool {
      SetCool(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setWarm`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_warm(&self) -> SetWarm {
      SetWarm(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setShininess`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shininess(&self) -> SetShininess {
      SetShininess(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QGoochMaterial::setBeta`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_beta(&self) -> SetBeta {
      SetBeta(self.0)
    }
  }
  impl ::gooch_material::GoochMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::gooch_material::GoochMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::gooch_material::GoochMaterial as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::gooch_material::GoochMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::gooch_material::GoochMaterial as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::gooch_material::GoochMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::gooch_material::GoochMaterial as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::gooch_material::GoochMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_QObject_ptr(self as *mut ::gooch_material::GoochMaterial)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_QObject_ptr(self as *const ::gooch_material::GoochMaterial as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::gooch_material::GoochMaterial> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::gooch_material::GoochMaterial> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::gooch_material::GoochMaterial> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::gooch_material::GoochMaterial> for ::qt_3d_render::material::Material {
  unsafe fn static_cast_mut(&mut self) -> &mut ::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::gooch_material::GoochMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DExtras_QGoochMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::gooch_material::GoochMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::gooch_material::GoochMaterial as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::gooch_material::GoochMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QGoochMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::gooch_material::GoochMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

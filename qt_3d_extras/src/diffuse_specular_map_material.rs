/// C++ type: <span style='color: green;'>```Qt3DExtras::QDiffuseSpecularMapMaterial```</span>
#[repr(C)]
pub struct DiffuseSpecularMapMaterial(u8);

impl DiffuseSpecularMapMaterial {
  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QDiffuseSpecularMapMaterial::ambient() const```</span>
  ///
  ///
  pub fn ambient(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_ambient_to_output(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QDiffuseSpecularMapMaterial::diffuse() const```</span>
  ///
  ///
  pub fn diffuse(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_diffuse(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QDiffuseSpecularMapMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_metaObject(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QDiffuseSpecularMapMaterial::QDiffuseSpecularMapMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::diffuse_specular_map_material::DiffuseSpecularMapMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QDiffuseSpecularMapMaterial::QDiffuseSpecularMapMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::diffuse_specular_map_material::DiffuseSpecularMapMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QDiffuseSpecularMapMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_qt_metacall(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QDiffuseSpecularMapMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_qt_metacast(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QDiffuseSpecularMapMaterial::setAmbient(const QColor& ambient)```</span>
  ///
  ///
  pub fn set_ambient(&mut self, ambient: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setAmbient(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, ambient as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QDiffuseSpecularMapMaterial::setDiffuse(Qt3DRender::QAbstractTexture* diffuse)```</span>
  ///
  ///
  pub unsafe fn set_diffuse(&mut self, diffuse: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setDiffuse(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, diffuse)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QDiffuseSpecularMapMaterial::setShininess(float shininess)```</span>
  ///
  ///
  pub fn set_shininess(&mut self, shininess: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setShininess(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, shininess) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QDiffuseSpecularMapMaterial::setSpecular(Qt3DRender::QAbstractTexture* specular)```</span>
  ///
  ///
  pub unsafe fn set_specular(&mut self, specular: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setSpecular(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, specular)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QDiffuseSpecularMapMaterial::setTextureScale(float textureScale)```</span>
  ///
  ///
  pub fn set_texture_scale(&mut self, texture_scale: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_setTextureScale(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial, texture_scale) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QDiffuseSpecularMapMaterial::shininess() const```</span>
  ///
  ///
  pub fn shininess(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_shininess(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QDiffuseSpecularMapMaterial::specular() const```</span>
  ///
  ///
  pub fn specular(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_specular(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QDiffuseSpecularMapMaterial::textureScale() const```</span>
  ///
  ///
  pub fn texture_scale(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_textureScale(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QDiffuseSpecularMapMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QDiffuseSpecularMapMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QDiffuseSpecularMapMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DiffuseSpecularMapMaterial`.
  pub struct Signals<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct EffectChanged<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::diffuseChanged`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.signals().diffuse_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct DiffuseChanged<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for DiffuseChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2diffuseChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DiffuseChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::textureScaleChanged`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.signals().texture_scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct TextureScaleChanged<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for TextureScaleChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textureScaleChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextureScaleChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::specularChanged`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.signals().specular_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct SpecularChanged<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SpecularChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2specularChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SpecularChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::shininessChanged`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.signals().shininess_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct ShininessChanged<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::ambientChanged`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.signals().ambient_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct AmbientChanged<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::diffuseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn diffuse_changed(&self) -> DiffuseChanged {
      DiffuseChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::textureScaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn texture_scale_changed(&self) -> TextureScaleChanged {
      TextureScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::specularChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn specular_changed(&self) -> SpecularChanged {
      SpecularChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::shininessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shininess_changed(&self) -> ShininessChanged {
      ShininessChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QDiffuseSpecularMapMaterial::ambientChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ambient_changed(&self) -> AmbientChanged {
      AmbientChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DiffuseSpecularMapMaterial`.
  pub struct Slots<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setAmbient`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.slots().set_ambient()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct SetAmbient<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetAmbient<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAmbient(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setShininess`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.slots().set_shininess()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct SetShininess<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetShininess<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShininess(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setEffect`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct SetEffect<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setSpecular`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.slots().set_specular()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct SetSpecular<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetSpecular<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSpecular(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setDiffuse`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.slots().set_diffuse()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct SetDiffuse<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetDiffuse<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDiffuse(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setTextureScale`.
  ///
  /// An object of this type can be created from `DiffuseSpecularMapMaterial` with `object.slots().set_texture_scale()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DiffuseSpecularMapMaterial` object.
  pub struct SetTextureScale<'a>(&'a ::diffuse_specular_map_material::DiffuseSpecularMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetTextureScale<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTextureScale(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setAmbient`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_ambient(&self) -> SetAmbient {
      SetAmbient(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setShininess`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shininess(&self) -> SetShininess {
      SetShininess(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setSpecular`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_specular(&self) -> SetSpecular {
      SetSpecular(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setDiffuse`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_diffuse(&self) -> SetDiffuse {
      SetDiffuse(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QDiffuseSpecularMapMaterial::setTextureScale`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_texture_scale(&self) -> SetTextureScale {
      SetTextureScale(self.0)
    }
  }
  impl ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::node::Node {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_QObject_ptr(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_QObject_ptr(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::diffuse_specular_map_material::DiffuseSpecularMapMaterial> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::diffuse_specular_map_material::DiffuseSpecularMapMaterial> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::diffuse_specular_map_material::DiffuseSpecularMapMaterial> for ::qt_3d_core::node::Node {
unsafe fn static_cast_mut(&mut self) -> &mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::diffuse_specular_map_material::DiffuseSpecularMapMaterial> for ::qt_3d_render::material::Material {
unsafe fn static_cast_mut(&mut self) -> &mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DExtras_QDiffuseSpecularMapMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::diffuse_specular_map_material::DiffuseSpecularMapMaterial as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::diffuse_specular_map_material::DiffuseSpecularMapMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QDiffuseSpecularMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::diffuse_specular_map_material::DiffuseSpecularMapMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

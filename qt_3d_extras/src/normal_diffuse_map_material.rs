/// C++ type: <span style='color: green;'>```Qt3DExtras::QNormalDiffuseMapMaterial```</span>
#[repr(C)]
pub struct NormalDiffuseMapMaterial(u8);

impl NormalDiffuseMapMaterial {
  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QNormalDiffuseMapMaterial::ambient() const```</span>
  ///
  ///
  pub fn ambient(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_ambient_to_output(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QNormalDiffuseMapMaterial::diffuse() const```</span>
  ///
  ///
  pub fn diffuse(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_diffuse(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QNormalDiffuseMapMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_metaObject(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QNormalDiffuseMapMaterial::QNormalDiffuseMapMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::normal_diffuse_map_material::NormalDiffuseMapMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QNormalDiffuseMapMaterial::QNormalDiffuseMapMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::normal_diffuse_map_material::NormalDiffuseMapMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QNormalDiffuseMapMaterial::normal() const```</span>
  ///
  ///
  pub fn normal(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_normal(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QNormalDiffuseMapMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_qt_metacall(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QNormalDiffuseMapMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_qt_metacast(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QNormalDiffuseMapMaterial::setAmbient(const QColor& ambient)```</span>
  ///
  ///
  pub fn set_ambient(&mut self, ambient: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_setAmbient(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, ambient as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QNormalDiffuseMapMaterial::setDiffuse(Qt3DRender::QAbstractTexture* diffuse)```</span>
  ///
  ///
  pub unsafe fn set_diffuse(&mut self, diffuse: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_setDiffuse(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, diffuse)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QNormalDiffuseMapMaterial::setNormal(Qt3DRender::QAbstractTexture* normal)```</span>
  ///
  ///
  pub unsafe fn set_normal(&mut self, normal: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_setNormal(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, normal)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QNormalDiffuseMapMaterial::setShininess(float shininess)```</span>
  ///
  ///
  pub fn set_shininess(&mut self, shininess: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_setShininess(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, shininess) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QNormalDiffuseMapMaterial::setSpecular(const QColor& specular)```</span>
  ///
  ///
  pub fn set_specular(&mut self, specular: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_setSpecular(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, specular as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QNormalDiffuseMapMaterial::setTextureScale(float textureScale)```</span>
  ///
  ///
  pub fn set_texture_scale(&mut self, texture_scale: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_setTextureScale(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial, texture_scale) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QNormalDiffuseMapMaterial::shininess() const```</span>
  ///
  ///
  pub fn shininess(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_shininess(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QNormalDiffuseMapMaterial::specular() const```</span>
  ///
  ///
  pub fn specular(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_specular_to_output(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QNormalDiffuseMapMaterial::textureScale() const```</span>
  ///
  ///
  pub fn texture_scale(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_textureScale(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QNormalDiffuseMapMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QNormalDiffuseMapMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QNormalDiffuseMapMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `NormalDiffuseMapMaterial`.
  pub struct Signals<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::shininessChanged`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.signals().shininess_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct ShininessChanged<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct EffectChanged<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::diffuseChanged`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.signals().diffuse_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct DiffuseChanged<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::ambientChanged`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.signals().ambient_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct AmbientChanged<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::specularChanged`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.signals().specular_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SpecularChanged<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::textureScaleChanged`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.signals().texture_scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct TextureScaleChanged<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::normalChanged`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.signals().normal_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct NormalChanged<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for NormalChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2normalChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NormalChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::shininessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shininess_changed(&self) -> ShininessChanged {
      ShininessChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::diffuseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn diffuse_changed(&self) -> DiffuseChanged {
      DiffuseChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::ambientChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ambient_changed(&self) -> AmbientChanged {
      AmbientChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::specularChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn specular_changed(&self) -> SpecularChanged {
      SpecularChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::textureScaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn texture_scale_changed(&self) -> TextureScaleChanged {
      TextureScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QNormalDiffuseMapMaterial::normalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn normal_changed(&self) -> NormalChanged {
      NormalChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `NormalDiffuseMapMaterial`.
  pub struct Slots<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setAmbient`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.slots().set_ambient()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SetAmbient<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetAmbient<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAmbient(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setSpecular`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.slots().set_specular()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SetSpecular<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetSpecular<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSpecular(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setTextureScale`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.slots().set_texture_scale()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SetTextureScale<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetTextureScale<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTextureScale(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setDiffuse`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.slots().set_diffuse()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SetDiffuse<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetDiffuse<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDiffuse(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setShininess`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.slots().set_shininess()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SetShininess<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetShininess<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShininess(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setEffect`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SetEffect<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setNormal`.
  ///
  /// An object of this type can be created from `NormalDiffuseMapMaterial` with `object.slots().set_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `NormalDiffuseMapMaterial` object.
  pub struct SetNormal<'a>(&'a ::normal_diffuse_map_material::NormalDiffuseMapMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetNormal<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNormal(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setAmbient`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_ambient(&self) -> SetAmbient {
      SetAmbient(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setSpecular`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_specular(&self) -> SetSpecular {
      SetSpecular(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setTextureScale`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_texture_scale(&self) -> SetTextureScale {
      SetTextureScale(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setDiffuse`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_diffuse(&self) -> SetDiffuse {
      SetDiffuse(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setShininess`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shininess(&self) -> SetShininess {
      SetShininess(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QNormalDiffuseMapMaterial::setNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_normal(&self) -> SetNormal {
      SetNormal(self.0)
    }
  }
  impl ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_QObject_ptr(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_QObject_ptr(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::normal_diffuse_map_material::NormalDiffuseMapMaterial> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::normal_diffuse_map_material::NormalDiffuseMapMaterial> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::normal_diffuse_map_material::NormalDiffuseMapMaterial> for ::qt_3d_core::node::Node {
unsafe fn static_cast_mut(&mut self) -> &mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::normal_diffuse_map_material::NormalDiffuseMapMaterial> for ::qt_3d_render::material::Material {
unsafe fn static_cast_mut(&mut self) -> &mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::normal_diffuse_map_material::NormalDiffuseMapMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DExtras_QNormalDiffuseMapMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::normal_diffuse_map_material::NormalDiffuseMapMaterial as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::normal_diffuse_map_material::NormalDiffuseMapMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QNormalDiffuseMapMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::normal_diffuse_map_material::NormalDiffuseMapMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

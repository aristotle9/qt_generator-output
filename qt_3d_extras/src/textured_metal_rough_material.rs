/// C++ type: <span style='color: green;'>```Qt3DExtras::QTexturedMetalRoughMaterial```</span>
#[repr(C)]
pub struct TexturedMetalRoughMaterial(u8);

impl TexturedMetalRoughMaterial {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QTexturedMetalRoughMaterial::ambientOcclusion() const```</span>
  ///
  ///
  pub fn ambient_occlusion(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_ambientOcclusion(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QTexturedMetalRoughMaterial::baseColor() const```</span>
  ///
  ///
  pub fn base_color(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_baseColor(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QTexturedMetalRoughMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_metaObject(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QTexturedMetalRoughMaterial::metalness() const```</span>
  ///
  ///
  pub fn metalness(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_metalness(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTexturedMetalRoughMaterial::QTexturedMetalRoughMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::textured_metal_rough_material::TexturedMetalRoughMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTexturedMetalRoughMaterial::QTexturedMetalRoughMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::textured_metal_rough_material::TexturedMetalRoughMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QTexturedMetalRoughMaterial::normal() const```</span>
  ///
  ///
  pub fn normal(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_normal(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QTexturedMetalRoughMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_qt_metacall(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QTexturedMetalRoughMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_qt_metacast(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QTexturedMetalRoughMaterial::roughness() const```</span>
  ///
  ///
  pub fn roughness(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_roughness(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTexturedMetalRoughMaterial::setAmbientOcclusion(Qt3DRender::QAbstractTexture* ambientOcclusion)```</span>
  ///
  ///
  pub unsafe fn set_ambient_occlusion(&mut self,
                                      ambient_occlusion: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_setAmbientOcclusion(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial, ambient_occlusion)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTexturedMetalRoughMaterial::setBaseColor(Qt3DRender::QAbstractTexture* baseColor)```</span>
  ///
  ///
  pub unsafe fn set_base_color(&mut self, base_color: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_setBaseColor(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial, base_color)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTexturedMetalRoughMaterial::setMetalness(Qt3DRender::QAbstractTexture* metalness)```</span>
  ///
  ///
  pub unsafe fn set_metalness(&mut self, metalness: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_setMetalness(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial, metalness)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTexturedMetalRoughMaterial::setNormal(Qt3DRender::QAbstractTexture* normal)```</span>
  ///
  ///
  pub unsafe fn set_normal(&mut self, normal: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_setNormal(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial, normal)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTexturedMetalRoughMaterial::setRoughness(Qt3DRender::QAbstractTexture* roughness)```</span>
  ///
  ///
  pub unsafe fn set_roughness(&mut self, roughness: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_setRoughness(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial, roughness)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTexturedMetalRoughMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTexturedMetalRoughMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::textured_metal_rough_material::TexturedMetalRoughMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTexturedMetalRoughMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TexturedMetalRoughMaterial`.
  pub struct Signals<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::baseColorChanged`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.signals().base_color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct BaseColorChanged<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for BaseColorChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2baseColorChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BaseColorChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::roughnessChanged`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.signals().roughness_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct RoughnessChanged<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for RoughnessChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2roughnessChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RoughnessChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::normalChanged`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.signals().normal_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct NormalChanged<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct EffectChanged<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::ambientOcclusionChanged`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.signals().ambient_occlusion_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct AmbientOcclusionChanged<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for AmbientOcclusionChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2ambientOcclusionChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AmbientOcclusionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::metalnessChanged`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.signals().metalness_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct MetalnessChanged<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for MetalnessChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2metalnessChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MetalnessChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::baseColorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn base_color_changed(&self) -> BaseColorChanged {
      BaseColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::roughnessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn roughness_changed(&self) -> RoughnessChanged {
      RoughnessChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::normalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn normal_changed(&self) -> NormalChanged {
      NormalChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::ambientOcclusionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ambient_occlusion_changed(&self) -> AmbientOcclusionChanged {
      AmbientOcclusionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTexturedMetalRoughMaterial::metalnessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn metalness_changed(&self) -> MetalnessChanged {
      MetalnessChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TexturedMetalRoughMaterial`.
  pub struct Slots<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setRoughness`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.slots().set_roughness()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct SetRoughness<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetRoughness<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRoughness(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setBaseColor`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.slots().set_base_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct SetBaseColor<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetBaseColor<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBaseColor(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setEffect`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct SetEffect<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setAmbientOcclusion`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.slots().set_ambient_occlusion()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct SetAmbientOcclusion<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetAmbientOcclusion<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAmbientOcclusion(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setMetalness`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.slots().set_metalness()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct SetMetalness<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetMetalness<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMetalness(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setNormal`.
  ///
  /// An object of this type can be created from `TexturedMetalRoughMaterial` with `object.slots().set_normal()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TexturedMetalRoughMaterial` object.
  pub struct SetNormal<'a>(&'a ::textured_metal_rough_material::TexturedMetalRoughMaterial);
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
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setRoughness`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_roughness(&self) -> SetRoughness {
      SetRoughness(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setBaseColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_base_color(&self) -> SetBaseColor {
      SetBaseColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setAmbientOcclusion`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_ambient_occlusion(&self) -> SetAmbientOcclusion {
      SetAmbientOcclusion(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setMetalness`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_metalness(&self) -> SetMetalness {
      SetMetalness(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTexturedMetalRoughMaterial::setNormal`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_normal(&self) -> SetNormal {
      SetNormal(self.0)
    }
  }
  impl ::textured_metal_rough_material::TexturedMetalRoughMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::textured_metal_rough_material::TexturedMetalRoughMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::textured_metal_rough_material::TexturedMetalRoughMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::node::Node {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::textured_metal_rough_material::TexturedMetalRoughMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::textured_metal_rough_material::TexturedMetalRoughMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_QObject_ptr(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_QObject_ptr(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::textured_metal_rough_material::TexturedMetalRoughMaterial> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::textured_metal_rough_material::TexturedMetalRoughMaterial> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::textured_metal_rough_material::TexturedMetalRoughMaterial> for ::qt_3d_core::node::Node {
unsafe fn static_cast_mut(&mut self) -> &mut ::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::textured_metal_rough_material::TexturedMetalRoughMaterial> for ::qt_3d_render::material::Material {
unsafe fn static_cast_mut(&mut self) -> &mut ::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::textured_metal_rough_material::TexturedMetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DExtras_QTexturedMetalRoughMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::textured_metal_rough_material::TexturedMetalRoughMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::textured_metal_rough_material::TexturedMetalRoughMaterial as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::textured_metal_rough_material::TexturedMetalRoughMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTexturedMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::textured_metal_rough_material::TexturedMetalRoughMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

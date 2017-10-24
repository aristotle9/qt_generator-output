/// C++ type: <span style='color: green;'>```Qt3DExtras::QTextureMaterial```</span>
#[repr(C)]
pub struct TextureMaterial(u8);

impl TextureMaterial {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QTextureMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_metaObject(self as *const ::texture_material::TextureMaterial)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTextureMaterial::QTextureMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::texture_material::TextureMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTextureMaterial::QTextureMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::texture_material::TextureMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QTextureMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_qt_metacall(self as *mut ::texture_material::TextureMaterial,
                                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                                  arg2,
                                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QTextureMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_qt_metacast(self as *mut ::texture_material::TextureMaterial,
                                                                  arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTextureMaterial::setTexture(Qt3DRender::QAbstractTexture* texture)```</span>
  ///
  ///
  pub unsafe fn set_texture(&mut self, texture: *mut ::qt_3d_render::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_setTexture(self as *mut ::texture_material::TextureMaterial,
                                                                 texture)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTextureMaterial::setTextureOffset(QVector2D textureOffset)```</span>
  ///
  ///
  pub fn set_texture_offset(&mut self, texture_offset: &::qt_gui::vector_2d::Vector2D) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_setTextureOffset(self as *mut ::texture_material::TextureMaterial, texture_offset as *const ::qt_gui::vector_2d::Vector2D) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DExtras::QTextureMaterial::texture() const```</span>
  ///
  ///
  pub fn texture(&self) -> *mut ::qt_3d_render::abstract_texture::AbstractTexture {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_texture(self as *const ::texture_material::TextureMaterial)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector2D Qt3DExtras::QTextureMaterial::textureOffset() const```</span>
  ///
  ///
  pub fn texture_offset(&self) -> ::cpp_utils::CppBox<::qt_gui::vector_2d::Vector2D> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_textureOffset_as_ptr(self as *const ::texture_material::TextureMaterial) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTextureMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTextureMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::texture_material::TextureMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTextureMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextureMaterial`.
  pub struct Signals<'a>(&'a ::texture_material::TextureMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QTextureMaterial::textureChanged`.
  ///
  /// An object of this type can be created from `TextureMaterial` with `object.signals().texture_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureMaterial` object.
  pub struct TextureChanged<'a>(&'a ::texture_material::TextureMaterial);
  impl<'a> ::qt_core::connection::Receiver for TextureChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textureChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextureChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QTextureMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `TextureMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureMaterial` object.
  pub struct EffectChanged<'a>(&'a ::texture_material::TextureMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTextureMaterial::textureOffsetChanged`.
  ///
  /// An object of this type can be created from `TextureMaterial` with `object.signals().texture_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureMaterial` object.
  pub struct TextureOffsetChanged<'a>(&'a ::texture_material::TextureMaterial);
  impl<'a> ::qt_core::connection::Receiver for TextureOffsetChanged<'a> {
    type Arguments = (&'static ::qt_gui::vector_2d::Vector2D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textureOffsetChanged(QVector2D)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextureOffsetChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTextureMaterial::textureChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn texture_changed(&self) -> TextureChanged {
      TextureChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTextureMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTextureMaterial::textureOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn texture_offset_changed(&self) -> TextureOffsetChanged {
      TextureOffsetChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TextureMaterial`.
  pub struct Slots<'a>(&'a ::texture_material::TextureMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QTextureMaterial::setEffect`.
  ///
  /// An object of this type can be created from `TextureMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureMaterial` object.
  pub struct SetEffect<'a>(&'a ::texture_material::TextureMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTextureMaterial::setTextureOffset`.
  ///
  /// An object of this type can be created from `TextureMaterial` with `object.slots().set_texture_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureMaterial` object.
  pub struct SetTextureOffset<'a>(&'a ::texture_material::TextureMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetTextureOffset<'a> {
    type Arguments = (&'static ::qt_gui::vector_2d::Vector2D,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTextureOffset(QVector2D)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTextureMaterial::setTexture`.
  ///
  /// An object of this type can be created from `TextureMaterial` with `object.slots().set_texture()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureMaterial` object.
  pub struct SetTexture<'a>(&'a ::texture_material::TextureMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetTexture<'a> {
    type Arguments = (*mut ::qt_3d_render::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTexture(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTextureMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTextureMaterial::setTextureOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_texture_offset(&self) -> SetTextureOffset {
      SetTextureOffset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTextureMaterial::setTexture`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_texture(&self) -> SetTexture {
      SetTexture(self.0)
    }
  }
  impl ::texture_material::TextureMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::texture_material::TextureMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::texture_material::TextureMaterial as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::texture_material::TextureMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::texture_material::TextureMaterial as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::texture_material::TextureMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::texture_material::TextureMaterial as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::texture_material::TextureMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_QObject_ptr(self as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_QObject_ptr(self as *const ::texture_material::TextureMaterial as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_material::TextureMaterial> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_material::TextureMaterial> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_material::TextureMaterial> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_material::TextureMaterial> for ::qt_3d_render::material::Material {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_material::TextureMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DExtras_QTextureMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::texture_material::TextureMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::texture_material::TextureMaterial as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::texture_material::TextureMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTextureMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::texture_material::TextureMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

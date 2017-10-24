/// C++ type: <span style='color: green;'>```Qt3DExtras::QMetalRoughMaterial```</span>
#[repr(C)]
pub struct MetalRoughMaterial(u8);

impl MetalRoughMaterial {
  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QMetalRoughMaterial::baseColor() const```</span>
  ///
  ///
  pub fn base_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_baseColor_to_output(self as *const ::metal_rough_material::MetalRoughMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QMetalRoughMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_metaObject(self as *const ::metal_rough_material::MetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QMetalRoughMaterial::metalness() const```</span>
  ///
  ///
  pub fn metalness(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_metalness(self as *const ::metal_rough_material::MetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QMetalRoughMaterial::QMetalRoughMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::metal_rough_material::MetalRoughMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QMetalRoughMaterial::QMetalRoughMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::metal_rough_material::MetalRoughMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QMetalRoughMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_qt_metacall(self as *mut ::metal_rough_material::MetalRoughMaterial, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QMetalRoughMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_qt_metacast(self as *mut ::metal_rough_material::MetalRoughMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QMetalRoughMaterial::roughness() const```</span>
  ///
  ///
  pub fn roughness(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_roughness(self as *const ::metal_rough_material::MetalRoughMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMetalRoughMaterial::setBaseColor(const QColor& baseColor)```</span>
  ///
  ///
  pub fn set_base_color(&mut self, base_color: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_setBaseColor(self as *mut ::metal_rough_material::MetalRoughMaterial, base_color as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMetalRoughMaterial::setMetalness(float metalness)```</span>
  ///
  ///
  pub fn set_metalness(&mut self, metalness: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_setMetalness(self as *mut ::metal_rough_material::MetalRoughMaterial, metalness) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QMetalRoughMaterial::setRoughness(float roughness)```</span>
  ///
  ///
  pub fn set_roughness(&mut self, roughness: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_setRoughness(self as *mut ::metal_rough_material::MetalRoughMaterial, roughness) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QMetalRoughMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QMetalRoughMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::metal_rough_material::MetalRoughMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QMetalRoughMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MetalRoughMaterial`.
  pub struct Signals<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::metalnessChanged`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.signals().metalness_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct MetalnessChanged<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for MetalnessChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2metalnessChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MetalnessChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct EffectChanged<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::roughnessChanged`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.signals().roughness_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct RoughnessChanged<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for RoughnessChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2roughnessChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RoughnessChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::baseColorChanged`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.signals().base_color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct BaseColorChanged<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for BaseColorChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2baseColorChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BaseColorChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::metalnessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn metalness_changed(&self) -> MetalnessChanged {
      MetalnessChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::roughnessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn roughness_changed(&self) -> RoughnessChanged {
      RoughnessChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QMetalRoughMaterial::baseColorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn base_color_changed(&self) -> BaseColorChanged {
      BaseColorChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MetalRoughMaterial`.
  pub struct Slots<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setEffect`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct SetEffect<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setRoughness`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.slots().set_roughness()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct SetRoughness<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetRoughness<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRoughness(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setBaseColor`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.slots().set_base_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct SetBaseColor<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetBaseColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBaseColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setMetalness`.
  ///
  /// An object of this type can be created from `MetalRoughMaterial` with `object.slots().set_metalness()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MetalRoughMaterial` object.
  pub struct SetMetalness<'a>(&'a ::metal_rough_material::MetalRoughMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetMetalness<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMetalness(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setRoughness`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_roughness(&self) -> SetRoughness {
      SetRoughness(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setBaseColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_base_color(&self) -> SetBaseColor {
      SetBaseColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QMetalRoughMaterial::setMetalness`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_metalness(&self) -> SetMetalness {
      SetMetalness(self.0)
    }
  }
  impl ::metal_rough_material::MetalRoughMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::metal_rough_material::MetalRoughMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::metal_rough_material::MetalRoughMaterial as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::metal_rough_material::MetalRoughMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::metal_rough_material::MetalRoughMaterial as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::metal_rough_material::MetalRoughMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::metal_rough_material::MetalRoughMaterial as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::metal_rough_material::MetalRoughMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_QObject_ptr(self as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_QObject_ptr(self as *const ::metal_rough_material::MetalRoughMaterial as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::metal_rough_material::MetalRoughMaterial> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::metal_rough_material::MetalRoughMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::metal_rough_material::MetalRoughMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::metal_rough_material::MetalRoughMaterial> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::metal_rough_material::MetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::metal_rough_material::MetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::metal_rough_material::MetalRoughMaterial> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::metal_rough_material::MetalRoughMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::metal_rough_material::MetalRoughMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::metal_rough_material::MetalRoughMaterial> for ::qt_3d_render::material::Material {
unsafe fn static_cast_mut(&mut self) -> &mut ::metal_rough_material::MetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::metal_rough_material::MetalRoughMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DExtras_QMetalRoughMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::metal_rough_material::MetalRoughMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::metal_rough_material::MetalRoughMaterial as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::metal_rough_material::MetalRoughMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QMetalRoughMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::metal_rough_material::MetalRoughMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

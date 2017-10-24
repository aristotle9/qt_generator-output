/// C++ type: <span style='color: green;'>```Qt3DExtras::QPhongAlphaMaterial```</span>
#[repr(C)]
pub struct PhongAlphaMaterial(u8);

impl PhongAlphaMaterial {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QPhongAlphaMaterial::alpha() const```</span>
  ///
  ///
  pub fn alpha(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_alpha(self as *const ::phong_alpha_material::PhongAlphaMaterial) }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QPhongAlphaMaterial::ambient() const```</span>
  ///
  ///
  pub fn ambient(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_ambient_to_output(self as *const ::phong_alpha_material::PhongAlphaMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QPhongAlphaMaterial::diffuse() const```</span>
  ///
  ///
  pub fn diffuse(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_diffuse_to_output(self as *const ::phong_alpha_material::PhongAlphaMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QPhongAlphaMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_metaObject(self as *const ::phong_alpha_material::PhongAlphaMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPhongAlphaMaterial::QPhongAlphaMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::phong_alpha_material::PhongAlphaMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPhongAlphaMaterial::QPhongAlphaMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::phong_alpha_material::PhongAlphaMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QPhongAlphaMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_qt_metacall(self as *mut ::phong_alpha_material::PhongAlphaMaterial, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QPhongAlphaMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_qt_metacast(self as *mut ::phong_alpha_material::PhongAlphaMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setAlpha(float alpha)```</span>
  ///
  ///
  pub fn set_alpha(&mut self, alpha: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setAlpha(self as *mut ::phong_alpha_material::PhongAlphaMaterial, alpha) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setAmbient(const QColor& ambient)```</span>
  ///
  ///
  pub fn set_ambient(&mut self, ambient: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setAmbient(self as *mut ::phong_alpha_material::PhongAlphaMaterial, ambient as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setBlendFunctionArg(Qt3DRender::QBlendEquation::BlendFunction blendFunctionArg)```</span>
  ///
  ///
  pub fn set_blend_function_arg(&mut self, blend_function_arg: &::qt_3d_render::blend_equation::BlendFunction) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setBlendFunctionArg(self as *mut ::phong_alpha_material::PhongAlphaMaterial, blend_function_arg as *const ::qt_3d_render::blend_equation::BlendFunction) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setDestinationAlphaArg(Qt3DRender::QBlendEquationArguments::Blending destinationAlphaArg)```</span>
  ///
  ///
  pub fn set_destination_alpha_arg(&mut self,
                                   destination_alpha_arg: &::qt_3d_render::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setDestinationAlphaArg(self as *mut ::phong_alpha_material::PhongAlphaMaterial, destination_alpha_arg as *const ::qt_3d_render::blend_equation_arguments::Blending) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setDestinationRgbArg(Qt3DRender::QBlendEquationArguments::Blending destinationRgbArg)```</span>
  ///
  ///
  pub fn set_destination_rgb_arg(&mut self, destination_rgb_arg: &::qt_3d_render::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setDestinationRgbArg(self as *mut ::phong_alpha_material::PhongAlphaMaterial, destination_rgb_arg as *const ::qt_3d_render::blend_equation_arguments::Blending) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setDiffuse(const QColor& diffuse)```</span>
  ///
  ///
  pub fn set_diffuse(&mut self, diffuse: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setDiffuse(self as *mut ::phong_alpha_material::PhongAlphaMaterial, diffuse as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setShininess(float shininess)```</span>
  ///
  ///
  pub fn set_shininess(&mut self, shininess: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setShininess(self as *mut ::phong_alpha_material::PhongAlphaMaterial, shininess) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setSourceAlphaArg(Qt3DRender::QBlendEquationArguments::Blending sourceAlphaArg)```</span>
  ///
  ///
  pub fn set_source_alpha_arg(&mut self, source_alpha_arg: &::qt_3d_render::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setSourceAlphaArg(self as *mut ::phong_alpha_material::PhongAlphaMaterial, source_alpha_arg as *const ::qt_3d_render::blend_equation_arguments::Blending) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setSourceRgbArg(Qt3DRender::QBlendEquationArguments::Blending sourceRgbArg)```</span>
  ///
  ///
  pub fn set_source_rgb_arg(&mut self, source_rgb_arg: &::qt_3d_render::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setSourceRgbArg(self as *mut ::phong_alpha_material::PhongAlphaMaterial, source_rgb_arg as *const ::qt_3d_render::blend_equation_arguments::Blending) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPhongAlphaMaterial::setSpecular(const QColor& specular)```</span>
  ///
  ///
  pub fn set_specular(&mut self, specular: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_setSpecular(self as *mut ::phong_alpha_material::PhongAlphaMaterial, specular as *const ::qt_gui::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QPhongAlphaMaterial::shininess() const```</span>
  ///
  ///
  pub fn shininess(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_shininess(self as *const ::phong_alpha_material::PhongAlphaMaterial) }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QPhongAlphaMaterial::specular() const```</span>
  ///
  ///
  pub fn specular(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_specular_to_output(self as *const ::phong_alpha_material::PhongAlphaMaterial, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPhongAlphaMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPhongAlphaMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::phong_alpha_material::PhongAlphaMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPhongAlphaMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PhongAlphaMaterial`.
  pub struct Signals<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::specularChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().specular_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SpecularChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::sourceAlphaArgChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().source_alpha_arg_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SourceAlphaArgChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SourceAlphaArgChanged<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceAlphaArgChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceAlphaArgChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::blendFunctionArgChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().blend_function_arg_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct BlendFunctionArgChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for BlendFunctionArgChanged<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation::BlendFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2blendFunctionArgChanged(Qt3DRender::QBlendEquation::BlendFunction)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BlendFunctionArgChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::sourceRgbArgChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().source_rgb_arg_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SourceRgbArgChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SourceRgbArgChanged<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceRgbArgChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceRgbArgChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::destinationAlphaArgChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().destination_alpha_arg_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct DestinationAlphaArgChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for DestinationAlphaArgChanged<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2destinationAlphaArgChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DestinationAlphaArgChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct EffectChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::diffuseChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().diffuse_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct DiffuseChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::alphaChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().alpha_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct AlphaChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::destinationRgbArgChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().destination_rgb_arg_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct DestinationRgbArgChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for DestinationRgbArgChanged<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2destinationRgbArgChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DestinationRgbArgChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::ambientChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().ambient_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct AmbientChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::shininessChanged`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.signals().shininess_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct ShininessChanged<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::specularChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn specular_changed(&self) -> SpecularChanged {
      SpecularChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::sourceAlphaArgChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_alpha_arg_changed(&self) -> SourceAlphaArgChanged {
      SourceAlphaArgChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::blendFunctionArgChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn blend_function_arg_changed(&self) -> BlendFunctionArgChanged {
      BlendFunctionArgChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::sourceRgbArgChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_rgb_arg_changed(&self) -> SourceRgbArgChanged {
      SourceRgbArgChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::destinationAlphaArgChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn destination_alpha_arg_changed(&self) -> DestinationAlphaArgChanged {
      DestinationAlphaArgChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::diffuseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn diffuse_changed(&self) -> DiffuseChanged {
      DiffuseChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::alphaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn alpha_changed(&self) -> AlphaChanged {
      AlphaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::destinationRgbArgChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn destination_rgb_arg_changed(&self) -> DestinationRgbArgChanged {
      DestinationRgbArgChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::ambientChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ambient_changed(&self) -> AmbientChanged {
      AmbientChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPhongAlphaMaterial::shininessChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shininess_changed(&self) -> ShininessChanged {
      ShininessChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PhongAlphaMaterial`.
  pub struct Slots<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setDestinationAlphaArg`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_destination_alpha_arg()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetDestinationAlphaArg<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetDestinationAlphaArg<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDestinationAlphaArg(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setAlpha`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_alpha()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetAlpha<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetAlpha<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAlpha(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setSourceRgbArg`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_source_rgb_arg()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetSourceRgbArg<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetSourceRgbArg<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceRgbArg(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setBlendFunctionArg`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_blend_function_arg()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetBlendFunctionArg<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetBlendFunctionArg<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation::BlendFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBlendFunctionArg(Qt3DRender::QBlendEquation::BlendFunction)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setSourceAlphaArg`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_source_alpha_arg()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetSourceAlphaArg<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetSourceAlphaArg<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceAlphaArg(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setShininess`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_shininess()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetShininess<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetShininess<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShininess(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setDestinationRgbArg`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_destination_rgb_arg()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetDestinationRgbArg<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetDestinationRgbArg<'a> {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDestinationRgbArg(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setEffect`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetEffect<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::qt_3d_render::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setAmbient`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_ambient()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetAmbient<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetAmbient<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAmbient(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setDiffuse`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_diffuse()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetDiffuse<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetDiffuse<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDiffuse(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setSpecular`.
  ///
  /// An object of this type can be created from `PhongAlphaMaterial` with `object.slots().set_specular()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PhongAlphaMaterial` object.
  pub struct SetSpecular<'a>(&'a ::phong_alpha_material::PhongAlphaMaterial);
  impl<'a> ::qt_core::connection::Receiver for SetSpecular<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSpecular(const QColor&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setDestinationAlphaArg`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_destination_alpha_arg(&self) -> SetDestinationAlphaArg {
      SetDestinationAlphaArg(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setAlpha`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_alpha(&self) -> SetAlpha {
      SetAlpha(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setSourceRgbArg`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_rgb_arg(&self) -> SetSourceRgbArg {
      SetSourceRgbArg(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setBlendFunctionArg`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_blend_function_arg(&self) -> SetBlendFunctionArg {
      SetBlendFunctionArg(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setSourceAlphaArg`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_alpha_arg(&self) -> SetSourceAlphaArg {
      SetSourceAlphaArg(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setShininess`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shininess(&self) -> SetShininess {
      SetShininess(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setDestinationRgbArg`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_destination_rgb_arg(&self) -> SetDestinationRgbArg {
      SetDestinationRgbArg(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setAmbient`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_ambient(&self) -> SetAmbient {
      SetAmbient(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setDiffuse`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_diffuse(&self) -> SetDiffuse {
      SetDiffuse(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPhongAlphaMaterial::setSpecular`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_specular(&self) -> SetSpecular {
      SetSpecular(self.0)
    }
  }
  impl ::phong_alpha_material::PhongAlphaMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::phong_alpha_material::PhongAlphaMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::phong_alpha_material::PhongAlphaMaterial as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::phong_alpha_material::PhongAlphaMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::phong_alpha_material::PhongAlphaMaterial as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::phong_alpha_material::PhongAlphaMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::phong_alpha_material::PhongAlphaMaterial as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::phong_alpha_material::PhongAlphaMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_QObject_ptr(self as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_QObject_ptr(self as *const ::phong_alpha_material::PhongAlphaMaterial as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::phong_alpha_material::PhongAlphaMaterial> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::phong_alpha_material::PhongAlphaMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::phong_alpha_material::PhongAlphaMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::phong_alpha_material::PhongAlphaMaterial> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::phong_alpha_material::PhongAlphaMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::phong_alpha_material::PhongAlphaMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::phong_alpha_material::PhongAlphaMaterial> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::phong_alpha_material::PhongAlphaMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::phong_alpha_material::PhongAlphaMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::phong_alpha_material::PhongAlphaMaterial> for ::qt_3d_render::material::Material {
unsafe fn static_cast_mut(&mut self) -> &mut ::phong_alpha_material::PhongAlphaMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::phong_alpha_material::PhongAlphaMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DExtras_QPhongAlphaMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::phong_alpha_material::PhongAlphaMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::phong_alpha_material::PhongAlphaMaterial as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::phong_alpha_material::PhongAlphaMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPhongAlphaMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::phong_alpha_material::PhongAlphaMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

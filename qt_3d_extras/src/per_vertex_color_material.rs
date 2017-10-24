/// C++ type: <span style='color: green;'>```Qt3DExtras::QPerVertexColorMaterial```</span>
#[repr(C)]
pub struct PerVertexColorMaterial(u8);

impl PerVertexColorMaterial {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QPerVertexColorMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_metaObject(self as *const ::per_vertex_color_material::PerVertexColorMaterial) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPerVertexColorMaterial::QPerVertexColorMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::per_vertex_color_material::PerVertexColorMaterial> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPerVertexColorMaterial::QPerVertexColorMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::per_vertex_color_material::PerVertexColorMaterial> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QPerVertexColorMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_qt_metacall(self as *mut ::per_vertex_color_material::PerVertexColorMaterial, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QPerVertexColorMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_qt_metacast(self as *mut ::per_vertex_color_material::PerVertexColorMaterial, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPerVertexColorMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPerVertexColorMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::per_vertex_color_material::PerVertexColorMaterial {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPerVertexColorMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PerVertexColorMaterial`.
  pub struct Signals<'a>(&'a ::per_vertex_color_material::PerVertexColorMaterial);
  /// Represents a built-in Qt signal `Qt3DExtras::QPerVertexColorMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `PerVertexColorMaterial` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PerVertexColorMaterial` object.
  pub struct EffectChanged<'a>(&'a ::per_vertex_color_material::PerVertexColorMaterial);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPerVertexColorMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PerVertexColorMaterial`.
  pub struct Slots<'a>(&'a ::per_vertex_color_material::PerVertexColorMaterial);
  /// Represents a built-in Qt slot `Qt3DExtras::QPerVertexColorMaterial::setEffect`.
  ///
  /// An object of this type can be created from `PerVertexColorMaterial` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PerVertexColorMaterial` object.
  pub struct SetEffect<'a>(&'a ::per_vertex_color_material::PerVertexColorMaterial);
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
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPerVertexColorMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
  }
  impl ::per_vertex_color_material::PerVertexColorMaterial {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::per_vertex_color_material::PerVertexColorMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::component::Component {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::per_vertex_color_material::PerVertexColorMaterial as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::per_vertex_color_material::PerVertexColorMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::per_vertex_color_material::PerVertexColorMaterial as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::material::Material> for ::per_vertex_color_material::PerVertexColorMaterial {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_render::material::Material {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::per_vertex_color_material::PerVertexColorMaterial as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::per_vertex_color_material::PerVertexColorMaterial {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_QObject_ptr(self as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_QObject_ptr(self as *const ::per_vertex_color_material::PerVertexColorMaterial as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::per_vertex_color_material::PerVertexColorMaterial> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::per_vertex_color_material::PerVertexColorMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::per_vertex_color_material::PerVertexColorMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::per_vertex_color_material::PerVertexColorMaterial> for ::qt_3d_core::component::Component {
unsafe fn static_cast_mut(&mut self) -> &mut ::per_vertex_color_material::PerVertexColorMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::per_vertex_color_material::PerVertexColorMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::per_vertex_color_material::PerVertexColorMaterial> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::per_vertex_color_material::PerVertexColorMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::per_vertex_color_material::PerVertexColorMaterial {
    let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::per_vertex_color_material::PerVertexColorMaterial> for ::qt_3d_render::material::Material {
unsafe fn static_cast_mut(&mut self) -> &mut ::per_vertex_color_material::PerVertexColorMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_Qt3DRender_QMaterial(self as *mut ::qt_3d_render::material::Material);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::per_vertex_color_material::PerVertexColorMaterial {
let ffi_result = ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DExtras_QPerVertexColorMaterial_ptr_Qt3DRender_QMaterial(self as *const ::qt_3d_render::material::Material as *mut ::qt_3d_render::material::Material);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::per_vertex_color_material::PerVertexColorMaterial {
  type Target = ::qt_3d_render::material::Material;
  fn deref(&self) -> &::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *const ::per_vertex_color_material::PerVertexColorMaterial as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::per_vertex_color_material::PerVertexColorMaterial {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::material::Material {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPerVertexColorMaterial_G_static_cast_Qt3DRender_QMaterial_ptr(self as *mut ::per_vertex_color_material::PerVertexColorMaterial) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

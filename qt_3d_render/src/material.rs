/// C++ type: <span style='color: green;'>```Qt3DRender::QMaterial```</span>
#[repr(C)]
pub struct Material(u8);

impl Material {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QMaterial::addParameter(Qt3DRender::QParameter* parameter)```</span>
  ///
  ///
  pub unsafe fn add_parameter(&mut self, parameter: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_addParameter(self as *mut ::material::Material, parameter)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QEffect* Qt3DRender::QMaterial::effect() const```</span>
  ///
  ///
  pub fn effect(&self) -> *mut ::effect::Effect {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_effect(self as *const ::material::Material) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QMaterial::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_metaObject(self as *const ::material::Material) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMaterial::QMaterial()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::material::Material> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMaterial::QMaterial(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::material::Material> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*> Qt3DRender::QMaterial::parameters() const```</span>
  ///
  ///
  pub fn parameters(&self) -> ::vector::VectorParameterMutPtr {
    {
      let mut object: ::vector::VectorParameterMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_parameters_to_output(self as *const ::material::Material,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QMaterial::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_qt_metacall(self as *mut ::material::Material,
                                                           arg1 as *const ::qt_core::meta_object::Call,
                                                           arg2,
                                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QMaterial::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_qt_metacast(self as *mut ::material::Material, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QMaterial::removeParameter(Qt3DRender::QParameter* parameter)```</span>
  ///
  ///
  pub unsafe fn remove_parameter(&mut self, parameter: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_removeParameter(self as *mut ::material::Material, parameter)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QMaterial::setEffect(Qt3DRender::QEffect* effect)```</span>
  ///
  ///
  pub unsafe fn set_effect(&mut self, effect: *mut ::effect::Effect) {
    ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_setEffect(self as *mut ::material::Material, effect)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMaterial::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMaterial::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::material::Material {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QMaterial_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Material`.
  pub struct Signals<'a>(&'a ::material::Material);
  /// Represents a built-in Qt signal `Qt3DRender::QMaterial::removedFromEntity`.
  ///
  /// An object of this type can be created from `Material` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Material` object.
  pub struct RemovedFromEntity<'a>(&'a ::material::Material);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QMaterial::shareableChanged`.
  ///
  /// An object of this type can be created from `Material` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Material` object.
  pub struct ShareableChanged<'a>(&'a ::material::Material);
  impl<'a> ::qt_core::connection::Receiver for ShareableChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shareableChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShareableChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QMaterial::effectChanged`.
  ///
  /// An object of this type can be created from `Material` with `object.signals().effect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Material` object.
  pub struct EffectChanged<'a>(&'a ::material::Material);
  impl<'a> ::qt_core::connection::Receiver for EffectChanged<'a> {
    type Arguments = (*mut ::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2effectChanged(Qt3DRender::QEffect*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EffectChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QMaterial::addedToEntity`.
  ///
  /// An object of this type can be created from `Material` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Material` object.
  pub struct AddedToEntity<'a>(&'a ::material::Material);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMaterial::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMaterial::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMaterial::effectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn effect_changed(&self) -> EffectChanged {
      EffectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMaterial::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Material`.
  pub struct Slots<'a>(&'a ::material::Material);
  /// Represents a built-in Qt slot `Qt3DRender::QMaterial::setShareable`.
  ///
  /// An object of this type can be created from `Material` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Material` object.
  pub struct SetShareable<'a>(&'a ::material::Material);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMaterial::setEffect`.
  ///
  /// An object of this type can be created from `Material` with `object.slots().set_effect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Material` object.
  pub struct SetEffect<'a>(&'a ::material::Material);
  impl<'a> ::qt_core::connection::Receiver for SetEffect<'a> {
    type Arguments = (*mut ::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEffect(Qt3DRender::QEffect*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMaterial::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMaterial::setEffect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_effect(&self) -> SetEffect {
      SetEffect(self.0)
    }
  }
  impl ::material::Material {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::material::Material {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::material::Material)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::material::Material as *mut ::material::Material) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::material::Material {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::material::Material) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::material::Material as *mut ::material::Material) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::material::Material {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QMaterial_G_static_cast_QObject_ptr(self as *mut ::material::Material) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMaterial_G_static_cast_QObject_ptr(self as *const ::material::Material as *mut ::material::Material) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::material::Material> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::material::Material {
    let ffi_result = ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::material::Material {
    let ffi_result = ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::material::Material> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::material::Material {
    let ffi_result = ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::material::Material {
    let ffi_result = ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::material::Material> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::material::Material {
    let ffi_result = ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::material::Material {
    let ffi_result = ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DRender_QMaterial_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::material::Material {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::material::Material as *mut ::material::Material) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::material::Material {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QMaterial_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::material::Material)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

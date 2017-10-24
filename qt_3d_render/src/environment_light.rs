/// C++ type: <span style='color: green;'>```Qt3DRender::QEnvironmentLight```</span>
#[repr(C)]
pub struct EnvironmentLight(u8);

impl EnvironmentLight {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DRender::QEnvironmentLight::irradiance() const```</span>
  ///
  ///
  pub fn irradiance(&self) -> *mut ::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_irradiance(self as *const ::environment_light::EnvironmentLight) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QEnvironmentLight::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_metaObject(self as *const ::environment_light::EnvironmentLight) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QEnvironmentLight::QEnvironmentLight()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::environment_light::EnvironmentLight> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QEnvironmentLight::QEnvironmentLight(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::environment_light::EnvironmentLight> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QEnvironmentLight::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_qt_metacall(self as *mut ::environment_light::EnvironmentLight,
                                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                                   arg2,
                                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QEnvironmentLight::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_qt_metacast(self as *mut ::environment_light::EnvironmentLight,
                                                                   arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QEnvironmentLight::setIrradiance(Qt3DRender::QAbstractTexture* irradiance)```</span>
  ///
  ///
  pub unsafe fn set_irradiance(&mut self, irradiance: *mut ::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_setIrradiance(self as *mut ::environment_light::EnvironmentLight, irradiance)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QEnvironmentLight::setSpecular(Qt3DRender::QAbstractTexture* specular)```</span>
  ///
  ///
  pub unsafe fn set_specular(&mut self, specular: *mut ::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_setSpecular(self as *mut ::environment_light::EnvironmentLight,
                                                                   specular)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DRender::QEnvironmentLight::specular() const```</span>
  ///
  ///
  pub fn specular(&self) -> *mut ::abstract_texture::AbstractTexture {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_specular(self as *const ::environment_light::EnvironmentLight)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QEnvironmentLight::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QEnvironmentLight::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::environment_light::EnvironmentLight {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QEnvironmentLight_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `EnvironmentLight`.
  pub struct Signals<'a>(&'a ::environment_light::EnvironmentLight);
  /// Represents a built-in Qt signal `Qt3DRender::QEnvironmentLight::irradianceChanged`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.signals().irradiance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct IrradianceChanged<'a>(&'a ::environment_light::EnvironmentLight);
  impl<'a> ::qt_core::connection::Receiver for IrradianceChanged<'a> {
    type Arguments = (*mut ::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2irradianceChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IrradianceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QEnvironmentLight::removedFromEntity`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct RemovedFromEntity<'a>(&'a ::environment_light::EnvironmentLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QEnvironmentLight::addedToEntity`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct AddedToEntity<'a>(&'a ::environment_light::EnvironmentLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QEnvironmentLight::shareableChanged`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct ShareableChanged<'a>(&'a ::environment_light::EnvironmentLight);
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
  /// Represents a built-in Qt signal `Qt3DRender::QEnvironmentLight::specularChanged`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.signals().specular_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct SpecularChanged<'a>(&'a ::environment_light::EnvironmentLight);
  impl<'a> ::qt_core::connection::Receiver for SpecularChanged<'a> {
    type Arguments = (*mut ::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2specularChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SpecularChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QEnvironmentLight::irradianceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn irradiance_changed(&self) -> IrradianceChanged {
      IrradianceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QEnvironmentLight::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QEnvironmentLight::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QEnvironmentLight::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QEnvironmentLight::specularChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn specular_changed(&self) -> SpecularChanged {
      SpecularChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `EnvironmentLight`.
  pub struct Slots<'a>(&'a ::environment_light::EnvironmentLight);
  /// Represents a built-in Qt slot `Qt3DRender::QEnvironmentLight::setIrradiance`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.slots().set_irradiance()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct SetIrradiance<'a>(&'a ::environment_light::EnvironmentLight);
  impl<'a> ::qt_core::connection::Receiver for SetIrradiance<'a> {
    type Arguments = (*mut ::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIrradiance(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QEnvironmentLight::setSpecular`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.slots().set_specular()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct SetSpecular<'a>(&'a ::environment_light::EnvironmentLight);
  impl<'a> ::qt_core::connection::Receiver for SetSpecular<'a> {
    type Arguments = (*mut ::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSpecular(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QEnvironmentLight::setShareable`.
  ///
  /// An object of this type can be created from `EnvironmentLight` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EnvironmentLight` object.
  pub struct SetShareable<'a>(&'a ::environment_light::EnvironmentLight);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QEnvironmentLight::setIrradiance`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_irradiance(&self) -> SetIrradiance {
      SetIrradiance(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QEnvironmentLight::setSpecular`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_specular(&self) -> SetSpecular {
      SetSpecular(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QEnvironmentLight::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::environment_light::EnvironmentLight {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::environment_light::EnvironmentLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::environment_light::EnvironmentLight as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::environment_light::EnvironmentLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::environment_light::EnvironmentLight as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::environment_light::EnvironmentLight {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_QObject_ptr(self as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_QObject_ptr(self as *const ::environment_light::EnvironmentLight as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::environment_light::EnvironmentLight> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::environment_light::EnvironmentLight {
    let ffi_result = ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::environment_light::EnvironmentLight {
    let ffi_result = ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::environment_light::EnvironmentLight> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::environment_light::EnvironmentLight {
    let ffi_result = ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::environment_light::EnvironmentLight {
    let ffi_result = ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::environment_light::EnvironmentLight> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::environment_light::EnvironmentLight {
    let ffi_result = ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::environment_light::EnvironmentLight {
    let ffi_result = ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::environment_light::EnvironmentLight {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::environment_light::EnvironmentLight as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::environment_light::EnvironmentLight {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::environment_light::EnvironmentLight) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

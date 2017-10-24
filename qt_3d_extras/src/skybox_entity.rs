/// C++ type: <span style='color: green;'>```Qt3DExtras::QSkyboxEntity```</span>
#[repr(C)]
pub struct SkyboxEntity(u8);

impl SkyboxEntity {
  /// C++ method: <span style='color: green;'>```QString Qt3DExtras::QSkyboxEntity::baseName() const```</span>
  ///
  ///
  pub fn base_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_baseName_to_output(self as *const ::skybox_entity::SkyboxEntity, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DExtras::QSkyboxEntity::extension() const```</span>
  ///
  ///
  pub fn extension(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_extension_to_output(self as *const ::skybox_entity::SkyboxEntity, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QSkyboxEntity::isGammaCorrectEnabled() const```</span>
  ///
  ///
  pub fn is_gamma_correct_enabled(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_isGammaCorrectEnabled(self as *const ::skybox_entity::SkyboxEntity)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QSkyboxEntity::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_metaObject(self as *const ::skybox_entity::SkyboxEntity) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QSkyboxEntity::QSkyboxEntity()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::skybox_entity::SkyboxEntity> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QSkyboxEntity::QSkyboxEntity(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::skybox_entity::SkyboxEntity> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QSkyboxEntity::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_qt_metacall(self as *mut ::skybox_entity::SkyboxEntity,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QSkyboxEntity::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_qt_metacast(self as *mut ::skybox_entity::SkyboxEntity, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QSkyboxEntity::setBaseName(const QString& path)```</span>
  ///
  ///
  pub fn set_base_name(&mut self, path: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_setBaseName(self as *mut ::skybox_entity::SkyboxEntity,
                                                                 path as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QSkyboxEntity::setExtension(const QString& extension)```</span>
  ///
  ///
  pub fn set_extension(&mut self, extension: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_setExtension(self as *mut ::skybox_entity::SkyboxEntity,
                                                                  extension as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QSkyboxEntity::setGammaCorrectEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_gamma_correct_enabled(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_setGammaCorrectEnabled(self as *mut ::skybox_entity::SkyboxEntity, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QSkyboxEntity::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QSkyboxEntity::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::skybox_entity::SkyboxEntity {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QSkyboxEntity_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SkyboxEntity`.
  pub struct Signals<'a>(&'a ::skybox_entity::SkyboxEntity);
  /// Represents a built-in Qt signal `Qt3DExtras::QSkyboxEntity::gammaCorrectEnabledChanged`.
  ///
  /// An object of this type can be created from `SkyboxEntity` with `object.signals().gamma_correct_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SkyboxEntity` object.
  pub struct GammaCorrectEnabledChanged<'a>(&'a ::skybox_entity::SkyboxEntity);
  impl<'a> ::qt_core::connection::Receiver for GammaCorrectEnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2gammaCorrectEnabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GammaCorrectEnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QSkyboxEntity::extensionChanged`.
  ///
  /// An object of this type can be created from `SkyboxEntity` with `object.signals().extension_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SkyboxEntity` object.
  pub struct ExtensionChanged<'a>(&'a ::skybox_entity::SkyboxEntity);
  impl<'a> ::qt_core::connection::Receiver for ExtensionChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2extensionChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ExtensionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QSkyboxEntity::baseNameChanged`.
  ///
  /// An object of this type can be created from `SkyboxEntity` with `object.signals().base_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SkyboxEntity` object.
  pub struct BaseNameChanged<'a>(&'a ::skybox_entity::SkyboxEntity);
  impl<'a> ::qt_core::connection::Receiver for BaseNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2baseNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BaseNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSkyboxEntity::gammaCorrectEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn gamma_correct_enabled_changed(&self) -> GammaCorrectEnabledChanged {
      GammaCorrectEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSkyboxEntity::extensionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn extension_changed(&self) -> ExtensionChanged {
      ExtensionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSkyboxEntity::baseNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn base_name_changed(&self) -> BaseNameChanged {
      BaseNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SkyboxEntity`.
  pub struct Slots<'a>(&'a ::skybox_entity::SkyboxEntity);
  /// Represents a built-in Qt slot `Qt3DExtras::QSkyboxEntity::setExtension`.
  ///
  /// An object of this type can be created from `SkyboxEntity` with `object.slots().set_extension()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SkyboxEntity` object.
  pub struct SetExtension<'a>(&'a ::skybox_entity::SkyboxEntity);
  impl<'a> ::qt_core::connection::Receiver for SetExtension<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setExtension(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QSkyboxEntity::setGammaCorrectEnabled`.
  ///
  /// An object of this type can be created from `SkyboxEntity` with `object.slots().set_gamma_correct_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SkyboxEntity` object.
  pub struct SetGammaCorrectEnabled<'a>(&'a ::skybox_entity::SkyboxEntity);
  impl<'a> ::qt_core::connection::Receiver for SetGammaCorrectEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGammaCorrectEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QSkyboxEntity::setBaseName`.
  ///
  /// An object of this type can be created from `SkyboxEntity` with `object.slots().set_base_name()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SkyboxEntity` object.
  pub struct SetBaseName<'a>(&'a ::skybox_entity::SkyboxEntity);
  impl<'a> ::qt_core::connection::Receiver for SetBaseName<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBaseName(const QString&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSkyboxEntity::setExtension`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_extension(&self) -> SetExtension {
      SetExtension(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSkyboxEntity::setGammaCorrectEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_gamma_correct_enabled(&self) -> SetGammaCorrectEnabled {
      SetGammaCorrectEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSkyboxEntity::setBaseName`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_base_name(&self) -> SetBaseName {
      SetBaseName(self.0)
    }
  }
  impl ::skybox_entity::SkyboxEntity {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::entity::Entity> for ::skybox_entity::SkyboxEntity {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::skybox_entity::SkyboxEntity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::skybox_entity::SkyboxEntity as *mut ::skybox_entity::SkyboxEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::skybox_entity::SkyboxEntity {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::skybox_entity::SkyboxEntity)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::skybox_entity::SkyboxEntity as *mut ::skybox_entity::SkyboxEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::skybox_entity::SkyboxEntity {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_QObject_ptr(self as *mut ::skybox_entity::SkyboxEntity)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_QObject_ptr(self as *const ::skybox_entity::SkyboxEntity as *mut ::skybox_entity::SkyboxEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::skybox_entity::SkyboxEntity> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::skybox_entity::SkyboxEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::skybox_entity::SkyboxEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::skybox_entity::SkyboxEntity> for ::qt_3d_core::entity::Entity {
  unsafe fn static_cast_mut(&mut self) -> &mut ::skybox_entity::SkyboxEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_Qt3DCore_QEntity(self as *mut ::qt_3d_core::entity::Entity);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::skybox_entity::SkyboxEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_Qt3DCore_QEntity(self as *const ::qt_3d_core::entity::Entity as *mut ::qt_3d_core::entity::Entity);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::skybox_entity::SkyboxEntity> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::skybox_entity::SkyboxEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::skybox_entity::SkyboxEntity {
    let ffi_result = ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DExtras_QSkyboxEntity_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::skybox_entity::SkyboxEntity {
  type Target = ::qt_3d_core::entity::Entity;
  fn deref(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::skybox_entity::SkyboxEntity as *mut ::skybox_entity::SkyboxEntity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::skybox_entity::SkyboxEntity {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSkyboxEntity_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::skybox_entity::SkyboxEntity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

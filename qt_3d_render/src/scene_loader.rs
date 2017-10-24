/// C++ type: <span style='color: green;'>```Qt3DRender::QSceneLoader::ComponentType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ComponentType {
  /// C++ enum variant: <span style='color: green;'>```UnknownComponent = 0```</span>
  Unknown = 0,
  /// C++ enum variant: <span style='color: green;'>```GeometryRendererComponent = 1```</span>
  GeometryRenderer = 1,
  /// C++ enum variant: <span style='color: green;'>```TransformComponent = 2```</span>
  Transform = 2,
  /// C++ enum variant: <span style='color: green;'>```MaterialComponent = 3```</span>
  Material = 3,
  /// C++ enum variant: <span style='color: green;'>```LightComponent = 4```</span>
  Light = 4,
  /// C++ enum variant: <span style='color: green;'>```CameraLensComponent = 5```</span>
  CameraLens = 5,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QSceneLoader```</span>
#[repr(C)]
pub struct SceneLoader(u8);

impl SceneLoader {
  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* Qt3DRender::QSceneLoader::component(const QString& entityName, Qt3DRender::QSceneLoader::ComponentType componentType) const```</span>
  ///
  ///
  pub fn component(&self,
                   entity_name: &::qt_core::string::String,
                   component_type: ::scene_loader::ComponentType)
                   -> *mut ::qt_3d_core::component::Component {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_component(self as *const ::scene_loader::SceneLoader,
                                                              entity_name as *const ::qt_core::string::String,
                                                              component_type)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* Qt3DRender::QSceneLoader::entity(const QString& entityName) const```</span>
  ///
  ///
  pub fn entity(&self, entity_name: &::qt_core::string::String) -> *mut ::qt_3d_core::entity::Entity {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_entity(self as *const ::scene_loader::SceneLoader,
                                                           entity_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList Qt3DRender::QSceneLoader::entityNames() const```</span>
  ///
  ///
  pub fn entity_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_entityNames_to_output(self as *const ::scene_loader::SceneLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QSceneLoader::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_metaObject(self as *const ::scene_loader::SceneLoader) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QSceneLoader::QSceneLoader()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::scene_loader::SceneLoader> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QSceneLoader::QSceneLoader(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::scene_loader::SceneLoader> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QSceneLoader::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_qt_metacall(self as *mut ::scene_loader::SceneLoader,
                                                              arg1 as *const ::qt_core::meta_object::Call,
                                                              arg2,
                                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QSceneLoader::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_qt_metacast(self as *mut ::scene_loader::SceneLoader, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSceneLoader::setSource(const QUrl& arg)```</span>
  ///
  ///
  pub fn set_source(&mut self, arg: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_setSource(self as *mut ::scene_loader::SceneLoader,
                                                              arg as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QSceneLoader::setStatus(Qt3DRender::QSceneLoader::Status status)```</span>
  ///
  ///
  pub fn set_status(&mut self, status: ::scene_loader::Status) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_setStatus(self as *mut ::scene_loader::SceneLoader, status) }
  }

  /// C++ method: <span style='color: green;'>```QUrl Qt3DRender::QSceneLoader::source() const```</span>
  ///
  ///
  pub fn source(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_source_to_output(self as *const ::scene_loader::SceneLoader,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSceneLoader::Status Qt3DRender::QSceneLoader::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::scene_loader::Status {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_status(self as *const ::scene_loader::SceneLoader) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QSceneLoader::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QSceneLoader::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::scene_loader::SceneLoader {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QSceneLoader_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SceneLoader`.
  pub struct Signals<'a>(&'a ::scene_loader::SceneLoader);
  /// Represents a built-in Qt signal `Qt3DRender::QSceneLoader::sourceChanged`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.signals().source_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct SourceChanged<'a>(&'a ::scene_loader::SceneLoader);
  impl<'a> ::qt_core::connection::Receiver for SourceChanged<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceChanged(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QSceneLoader::removedFromEntity`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct RemovedFromEntity<'a>(&'a ::scene_loader::SceneLoader);
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
  /// Represents a built-in Qt signal `Qt3DRender::QSceneLoader::shareableChanged`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct ShareableChanged<'a>(&'a ::scene_loader::SceneLoader);
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
  /// Represents a built-in Qt signal `Qt3DRender::QSceneLoader::statusChanged`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.signals().status_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct StatusChanged<'a>(&'a ::scene_loader::SceneLoader);
  impl<'a> ::qt_core::connection::Receiver for StatusChanged<'a> {
    type Arguments = (::scene_loader::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2statusChanged(Qt3DRender::QSceneLoader::Status)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StatusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QSceneLoader::addedToEntity`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct AddedToEntity<'a>(&'a ::scene_loader::SceneLoader);
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
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSceneLoader::sourceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_changed(&self) -> SourceChanged {
      SourceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSceneLoader::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSceneLoader::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSceneLoader::statusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn status_changed(&self) -> StatusChanged {
      StatusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QSceneLoader::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SceneLoader`.
  pub struct Slots<'a>(&'a ::scene_loader::SceneLoader);
  /// Represents a built-in Qt slot `Qt3DRender::QSceneLoader::setSource`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.slots().set_source()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct SetSource<'a>(&'a ::scene_loader::SceneLoader);
  impl<'a> ::qt_core::connection::Receiver for SetSource<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSource(const QUrl&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSceneLoader::setShareable`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct SetShareable<'a>(&'a ::scene_loader::SceneLoader);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QSceneLoader::setStatus`.
  ///
  /// An object of this type can be created from `SceneLoader` with `object.slots().set_status()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SceneLoader` object.
  pub struct SetStatus<'a>(&'a ::scene_loader::SceneLoader);
  impl<'a> ::qt_core::connection::Receiver for SetStatus<'a> {
    type Arguments = (::scene_loader::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setStatus(Qt3DRender::QSceneLoader::Status)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSceneLoader::setSource`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source(&self) -> SetSource {
      SetSource(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSceneLoader::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QSceneLoader::setStatus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_status(&self) -> SetStatus {
      SetStatus(self.0)
    }
  }
  impl ::scene_loader::SceneLoader {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QSceneLoader::Status```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Status {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Loading = 1```</span>
  Loading = 1,
  /// C++ enum variant: <span style='color: green;'>```Ready = 2```</span>
  Ready = 2,
  /// C++ enum variant: <span style='color: green;'>```Error = 3```</span>
  Error = 3,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::scene_loader::SceneLoader {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::scene_loader::SceneLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::scene_loader::SceneLoader as *mut ::scene_loader::SceneLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::scene_loader::SceneLoader {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::scene_loader::SceneLoader)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::scene_loader::SceneLoader as *mut ::scene_loader::SceneLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::scene_loader::SceneLoader {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_QObject_ptr(self as *mut ::scene_loader::SceneLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_QObject_ptr(self as *const ::scene_loader::SceneLoader as *mut ::scene_loader::SceneLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scene_loader::SceneLoader> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scene_loader::SceneLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scene_loader::SceneLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scene_loader::SceneLoader> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scene_loader::SceneLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scene_loader::SceneLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scene_loader::SceneLoader> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scene_loader::SceneLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scene_loader::SceneLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DRender_QSceneLoader_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::scene_loader::SceneLoader {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::scene_loader::SceneLoader as *mut ::scene_loader::SceneLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::scene_loader::SceneLoader {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QSceneLoader_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::scene_loader::SceneLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

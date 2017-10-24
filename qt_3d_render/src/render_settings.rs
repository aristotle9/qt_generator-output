/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderSettings::RenderPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RenderPolicy {
  /// C++ enum variant: <span style='color: green;'>```OnDemand = 0```</span>
  OnDemand = 0,
  /// C++ enum variant: <span style='color: green;'>```Always = 1```</span>
  Always = 1,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderSettings```</span>
#[repr(C)]
pub struct RenderSettings(u8);

impl RenderSettings {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QFrameGraphNode* Qt3DRender::QRenderSettings::activeFrameGraph() const```</span>
  ///
  ///
  pub fn active_frame_graph(&self) -> *mut ::frame_graph_node::FrameGraphNode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_activeFrameGraph(self as *const ::render_settings::RenderSettings) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderSettings::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_metaObject(self as *const ::render_settings::RenderSettings)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderSettings::QRenderSettings()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_settings::RenderSettings> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderSettings::QRenderSettings(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::render_settings::RenderSettings> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QPickingSettings* Qt3DRender::QRenderSettings::pickingSettings()```</span>
  ///
  ///
  pub fn picking_settings(&mut self) -> *mut ::picking_settings::PickingSettings {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_pickingSettings(self as *mut ::render_settings::RenderSettings)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderSettings::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_qt_metacall(self as *mut ::render_settings::RenderSettings,
                                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                                 arg2,
                                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderSettings::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_qt_metacast(self as *mut ::render_settings::RenderSettings, arg1)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderSettings::RenderPolicy Qt3DRender::QRenderSettings::renderPolicy() const```</span>
  ///
  ///
  pub fn render_policy(&self) -> ::render_settings::RenderPolicy {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_renderPolicy(self as *const ::render_settings::RenderSettings)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderSettings::setActiveFrameGraph(Qt3DRender::QFrameGraphNode* activeFrameGraph)```</span>
  ///
  ///
  pub unsafe fn set_active_frame_graph(&mut self, active_frame_graph: *mut ::frame_graph_node::FrameGraphNode) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_setActiveFrameGraph(self as *mut ::render_settings::RenderSettings, active_frame_graph)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderSettings::setRenderPolicy(Qt3DRender::QRenderSettings::RenderPolicy renderPolicy)```</span>
  ///
  ///
  pub fn set_render_policy(&mut self, render_policy: ::render_settings::RenderPolicy) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_setRenderPolicy(self as *mut ::render_settings::RenderSettings,
                                                                       render_policy)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderSettings::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderSettings::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_settings::RenderSettings {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSettings_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RenderSettings`.
  pub struct Signals<'a>(&'a ::render_settings::RenderSettings);
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSettings::addedToEntity`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct AddedToEntity<'a>(&'a ::render_settings::RenderSettings);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSettings::renderPolicyChanged`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.signals().render_policy_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct RenderPolicyChanged<'a>(&'a ::render_settings::RenderSettings);
  impl<'a> ::qt_core::connection::Receiver for RenderPolicyChanged<'a> {
    type Arguments = (::render_settings::RenderPolicy,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2renderPolicyChanged(Qt3DRender::QRenderSettings::RenderPolicy)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RenderPolicyChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSettings::activeFrameGraphChanged`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.signals().active_frame_graph_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct ActiveFrameGraphChanged<'a>(&'a ::render_settings::RenderSettings);
  impl<'a> ::qt_core::connection::Receiver for ActiveFrameGraphChanged<'a> {
    type Arguments = (*mut ::frame_graph_node::FrameGraphNode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activeFrameGraphChanged(Qt3DRender::QFrameGraphNode*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActiveFrameGraphChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSettings::shareableChanged`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct ShareableChanged<'a>(&'a ::render_settings::RenderSettings);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSettings::removedFromEntity`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct RemovedFromEntity<'a>(&'a ::render_settings::RenderSettings);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSettings::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSettings::renderPolicyChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn render_policy_changed(&self) -> RenderPolicyChanged {
      RenderPolicyChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSettings::activeFrameGraphChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_frame_graph_changed(&self) -> ActiveFrameGraphChanged {
      ActiveFrameGraphChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSettings::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSettings::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `RenderSettings`.
  pub struct Slots<'a>(&'a ::render_settings::RenderSettings);
  /// Represents a built-in Qt slot `Qt3DRender::QRenderSettings::setActiveFrameGraph`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.slots().set_active_frame_graph()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct SetActiveFrameGraph<'a>(&'a ::render_settings::RenderSettings);
  impl<'a> ::qt_core::connection::Receiver for SetActiveFrameGraph<'a> {
    type Arguments = (*mut ::frame_graph_node::FrameGraphNode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setActiveFrameGraph(Qt3DRender::QFrameGraphNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderSettings::setRenderPolicy`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.slots().set_render_policy()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct SetRenderPolicy<'a>(&'a ::render_settings::RenderSettings);
  impl<'a> ::qt_core::connection::Receiver for SetRenderPolicy<'a> {
    type Arguments = (::render_settings::RenderPolicy,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRenderPolicy(Qt3DRender::QRenderSettings::RenderPolicy)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderSettings::setShareable`.
  ///
  /// An object of this type can be created from `RenderSettings` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSettings` object.
  pub struct SetShareable<'a>(&'a ::render_settings::RenderSettings);
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
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderSettings::setActiveFrameGraph`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_active_frame_graph(&self) -> SetActiveFrameGraph {
      SetActiveFrameGraph(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderSettings::setRenderPolicy`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_render_policy(&self) -> SetRenderPolicy {
      SetRenderPolicy(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderSettings::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::render_settings::RenderSettings {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::render_settings::RenderSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::render_settings::RenderSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::render_settings::RenderSettings as *mut ::render_settings::RenderSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_settings::RenderSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_settings::RenderSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_settings::RenderSettings as *mut ::render_settings::RenderSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_settings::RenderSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_QObject_ptr(self as *mut ::render_settings::RenderSettings)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_QObject_ptr(self as *const ::render_settings::RenderSettings as *mut ::render_settings::RenderSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_settings::RenderSettings> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_settings::RenderSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_settings::RenderSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_settings::RenderSettings> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_settings::RenderSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_settings::RenderSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_settings::RenderSettings> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_settings::RenderSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_settings::RenderSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DRender_QRenderSettings_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::render_settings::RenderSettings {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::render_settings::RenderSettings as *mut ::render_settings::RenderSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_settings::RenderSettings {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSettings_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::render_settings::RenderSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

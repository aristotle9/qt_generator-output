/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderPass```</span>
#[repr(C)]
pub struct RenderPass(u8);

impl RenderPass {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPass::addFilterKey(Qt3DRender::QFilterKey* filterKey)```</span>
  ///
  ///
  pub unsafe fn add_filter_key(&mut self, filter_key: *mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_addFilterKey(self as *mut ::render_pass::RenderPass, filter_key)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPass::addParameter(Qt3DRender::QParameter* p)```</span>
  ///
  ///
  pub unsafe fn add_parameter(&mut self, p: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_addParameter(self as *mut ::render_pass::RenderPass, p)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPass::addRenderState(Qt3DRender::QRenderState* state)```</span>
  ///
  ///
  pub unsafe fn add_render_state(&mut self, state: *mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_addRenderState(self as *mut ::render_pass::RenderPass, state)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*> Qt3DRender::QRenderPass::filterKeys() const```</span>
  ///
  ///
  pub fn filter_keys(&self) -> ::vector::VectorFilterKeyMutPtr {
    {
      let mut object: ::vector::VectorFilterKeyMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_filterKeys_to_output(self as *const ::render_pass::RenderPass,
                                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderPass::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_metaObject(self as *const ::render_pass::RenderPass) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderPass::QRenderPass()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_pass::RenderPass> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderPass::QRenderPass(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::render_pass::RenderPass> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*> Qt3DRender::QRenderPass::parameters() const```</span>
  ///
  ///
  pub fn parameters(&self) -> ::vector::VectorParameterMutPtr {
    {
      let mut object: ::vector::VectorParameterMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_parameters_to_output(self as *const ::render_pass::RenderPass,
                                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderPass::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_qt_metacall(self as *mut ::render_pass::RenderPass,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderPass::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_qt_metacast(self as *mut ::render_pass::RenderPass, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPass::removeFilterKey(Qt3DRender::QFilterKey* filterKey)```</span>
  ///
  ///
  pub unsafe fn remove_filter_key(&mut self, filter_key: *mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_removeFilterKey(self as *mut ::render_pass::RenderPass, filter_key)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPass::removeParameter(Qt3DRender::QParameter* p)```</span>
  ///
  ///
  pub unsafe fn remove_parameter(&mut self, p: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_removeParameter(self as *mut ::render_pass::RenderPass, p)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPass::removeRenderState(Qt3DRender::QRenderState* state)```</span>
  ///
  ///
  pub unsafe fn remove_render_state(&mut self, state: *mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_removeRenderState(self as *mut ::render_pass::RenderPass, state)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*> Qt3DRender::QRenderPass::renderStates() const```</span>
  ///
  ///
  pub fn render_states(&self) -> ::vector::VectorRenderStateMutPtr {
    {
      let mut object: ::vector::VectorRenderStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_renderStates_to_output(self as *const ::render_pass::RenderPass,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderPass::setShaderProgram(Qt3DRender::QShaderProgram* shaderProgram)```</span>
  ///
  ///
  pub unsafe fn set_shader_program(&mut self, shader_program: *mut ::shader_program::ShaderProgram) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_setShaderProgram(self as *mut ::render_pass::RenderPass,
                                                                  shader_program)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QShaderProgram* Qt3DRender::QRenderPass::shaderProgram() const```</span>
  ///
  ///
  pub fn shader_program(&self) -> *mut ::shader_program::ShaderProgram {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_shaderProgram(self as *const ::render_pass::RenderPass) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderPass::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderPass::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_pass::RenderPass {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPass_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RenderPass`.
  pub struct Signals<'a>(&'a ::render_pass::RenderPass);
  /// Represents a built-in Qt signal `Qt3DRender::QRenderPass::nodeDestroyed`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct NodeDestroyed<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for NodeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nodeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NodeDestroyed<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderPass::enabledChanged`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct EnabledChanged<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderPass::shaderProgramChanged`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.signals().shader_program_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct ShaderProgramChanged<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for ShaderProgramChanged<'a> {
    type Arguments = (*mut ::shader_program::ShaderProgram,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shaderProgramChanged(Qt3DRender::QShaderProgram*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShaderProgramChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderPass::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderPass::parentChanged`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct ParentChanged<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderPass::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderPass::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderPass::shaderProgramChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shader_program_changed(&self) -> ShaderProgramChanged {
      ShaderProgramChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderPass::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderPass::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `RenderPass`.
  pub struct Slots<'a>(&'a ::render_pass::RenderPass);
  /// Represents a built-in Qt slot `Qt3DRender::QRenderPass::setEnabled`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct SetEnabled<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderPass::setParent`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct SetParent<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderPass::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderPass::setShaderProgram`.
  ///
  /// An object of this type can be created from `RenderPass` with `object.slots().set_shader_program()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderPass` object.
  pub struct SetShaderProgram<'a>(&'a ::render_pass::RenderPass);
  impl<'a> ::qt_core::connection::Receiver for SetShaderProgram<'a> {
    type Arguments = (*mut ::shader_program::ShaderProgram,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShaderProgram(Qt3DRender::QShaderProgram*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderPass::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderPass::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderPass::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderPass::setShaderProgram`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shader_program(&self) -> SetShaderProgram {
      SetShaderProgram(self.0)
    }
  }
  impl ::render_pass::RenderPass {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_pass::RenderPass {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_pass::RenderPass)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_pass::RenderPass as *mut ::render_pass::RenderPass) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_pass::RenderPass {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_QObject_ptr(self as *mut ::render_pass::RenderPass) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_QObject_ptr(self as *const ::render_pass::RenderPass as *mut ::render_pass::RenderPass) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_pass::RenderPass> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_pass::RenderPass {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_pass::RenderPass {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_pass::RenderPass> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_pass::RenderPass {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_pass::RenderPass {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DRender_QRenderPass_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::render_pass::RenderPass {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_pass::RenderPass as *mut ::render_pass::RenderPass) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_pass::RenderPass {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QRenderPass_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_pass::RenderPass)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

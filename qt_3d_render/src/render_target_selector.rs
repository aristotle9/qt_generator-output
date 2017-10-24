/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderTargetSelector```</span>
#[repr(C)]
pub struct RenderTargetSelector(u8);

impl RenderTargetSelector {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderTargetSelector::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_metaObject(self as *const ::render_target_selector::RenderTargetSelector) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderTargetSelector::QRenderTargetSelector()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_target_selector::RenderTargetSelector> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderTargetSelector::QRenderTargetSelector(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::render_target_selector::RenderTargetSelector> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint> Qt3DRender::QRenderTargetSelector::outputs() const```</span>
  ///
  ///
  pub fn outputs(&self) -> ::vector::VectorRenderTargetOutputAttachmentPoint {
    {
      let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_outputs_to_output(self as *const ::render_target_selector::RenderTargetSelector, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderTargetSelector::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_qt_metacall(self as *mut ::render_target_selector::RenderTargetSelector, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderTargetSelector::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_qt_metacast(self as *mut ::render_target_selector::RenderTargetSelector, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderTargetSelector::setOutputs(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& buffers)```</span>
  ///
  ///
  pub fn set_outputs(&mut self, buffers: &::vector::VectorRenderTargetOutputAttachmentPoint) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_setOutputs(self as *mut ::render_target_selector::RenderTargetSelector, buffers as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderTargetSelector::setTarget(Qt3DRender::QRenderTarget* target)```</span>
  ///
  ///
  pub unsafe fn set_target(&mut self, target: *mut ::render_target::RenderTarget) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_setTarget(self as *mut ::render_target_selector::RenderTargetSelector, target)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTarget* Qt3DRender::QRenderTargetSelector::target() const```</span>
  ///
  ///
  pub fn target(&self) -> *mut ::render_target::RenderTarget {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_target(self as *const ::render_target_selector::RenderTargetSelector) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderTargetSelector::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderTargetSelector::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_target_selector::RenderTargetSelector {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetSelector_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RenderTargetSelector`.
  pub struct Signals<'a>(&'a ::render_target_selector::RenderTargetSelector);
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetSelector::targetChanged`.
  ///
  /// An object of this type can be created from `RenderTargetSelector` with `object.signals().target_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetSelector` object.
  pub struct TargetChanged<'a>(&'a ::render_target_selector::RenderTargetSelector);
  impl<'a> ::qt_core::connection::Receiver for TargetChanged<'a> {
    type Arguments = (*mut ::render_target::RenderTarget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2targetChanged(Qt3DRender::QRenderTarget*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TargetChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetSelector::targetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_changed(&self) -> TargetChanged {
      TargetChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `RenderTargetSelector`.
  pub struct Slots<'a>(&'a ::render_target_selector::RenderTargetSelector);
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetSelector::setTarget`.
  ///
  /// An object of this type can be created from `RenderTargetSelector` with `object.slots().set_target()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetSelector` object.
  pub struct SetTarget<'a>(&'a ::render_target_selector::RenderTargetSelector);
  impl<'a> ::qt_core::connection::Receiver for SetTarget<'a> {
    type Arguments = (*mut ::render_target::RenderTarget,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTarget(Qt3DRender::QRenderTarget*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetSelector::setTarget`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_target(&self) -> SetTarget {
      SetTarget(self.0)
    }
  }
  impl ::render_target_selector::RenderTargetSelector {
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

impl ::cpp_utils::DynamicCast<::render_target_selector::RenderTargetSelector> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::render_target_selector::RenderTargetSelector> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_dynamic_cast_Qt3DRender_QRenderTargetSelector_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::render_target_selector::RenderTargetSelector> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_dynamic_cast_Qt3DRender_QRenderTargetSelector_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_target_selector::RenderTargetSelector {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_target_selector::RenderTargetSelector as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::render_target_selector::RenderTargetSelector {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_target_selector::RenderTargetSelector as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_target_selector::RenderTargetSelector {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_QObject_ptr(self as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_QObject_ptr(self as *const ::render_target_selector::RenderTargetSelector as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target_selector::RenderTargetSelector> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_target_selector::RenderTargetSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_target_selector::RenderTargetSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target_selector::RenderTargetSelector> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_target_selector::RenderTargetSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_target_selector::RenderTargetSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target_selector::RenderTargetSelector> for ::frame_graph_node::FrameGraphNode {
unsafe fn static_cast_mut(&mut self) -> &mut ::render_target_selector::RenderTargetSelector {
let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::render_target_selector::RenderTargetSelector {
let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QRenderTargetSelector_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::render_target_selector::RenderTargetSelector {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_target_selector::RenderTargetSelector as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_target_selector::RenderTargetSelector {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_target_selector::RenderTargetSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

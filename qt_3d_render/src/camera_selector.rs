/// C++ type: <span style='color: green;'>```Qt3DRender::QCameraSelector```</span>
#[repr(C)]
pub struct CameraSelector(u8);

impl CameraSelector {
  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* Qt3DRender::QCameraSelector::camera() const```</span>
  ///
  ///
  pub fn camera(&self) -> *mut ::qt_3d_core::entity::Entity {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_camera(self as *const ::camera_selector::CameraSelector) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QCameraSelector::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_metaObject(self as *const ::camera_selector::CameraSelector)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCameraSelector::QCameraSelector()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::camera_selector::CameraSelector> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCameraSelector::QCameraSelector(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::camera_selector::CameraSelector> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QCameraSelector::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_qt_metacall(self as *mut ::camera_selector::CameraSelector,
                                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                                 arg2,
                                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QCameraSelector::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_qt_metacast(self as *mut ::camera_selector::CameraSelector, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCameraSelector::setCamera(Qt3DCore::QEntity* camera)```</span>
  ///
  ///
  pub unsafe fn set_camera(&mut self, camera: *mut ::qt_3d_core::entity::Entity) {
    ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_setCamera(self as *mut ::camera_selector::CameraSelector, camera)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCameraSelector::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCameraSelector::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::camera_selector::CameraSelector {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QCameraSelector_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CameraSelector`.
  pub struct Signals<'a>(&'a ::camera_selector::CameraSelector);
  /// Represents a built-in Qt signal `Qt3DRender::QCameraSelector::cameraChanged`.
  ///
  /// An object of this type can be created from `CameraSelector` with `object.signals().camera_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraSelector` object.
  pub struct CameraChanged<'a>(&'a ::camera_selector::CameraSelector);
  impl<'a> ::qt_core::connection::Receiver for CameraChanged<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cameraChanged(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CameraChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCameraSelector::cameraChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn camera_changed(&self) -> CameraChanged {
      CameraChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CameraSelector`.
  pub struct Slots<'a>(&'a ::camera_selector::CameraSelector);
  /// Represents a built-in Qt slot `Qt3DRender::QCameraSelector::setCamera`.
  ///
  /// An object of this type can be created from `CameraSelector` with `object.slots().set_camera()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CameraSelector` object.
  pub struct SetCamera<'a>(&'a ::camera_selector::CameraSelector);
  impl<'a> ::qt_core::connection::Receiver for SetCamera<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCamera(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCameraSelector::setCamera`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_camera(&self) -> SetCamera {
      SetCamera(self.0)
    }
  }
  impl ::camera_selector::CameraSelector {
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

impl ::cpp_utils::DynamicCast<::camera_selector::CameraSelector> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::camera_selector::CameraSelector> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_dynamic_cast_Qt3DRender_QCameraSelector_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::camera_selector::CameraSelector> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_dynamic_cast_Qt3DRender_QCameraSelector_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::camera_selector::CameraSelector {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::camera_selector::CameraSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::camera_selector::CameraSelector as *mut ::camera_selector::CameraSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::camera_selector::CameraSelector {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::camera_selector::CameraSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::camera_selector::CameraSelector as *mut ::camera_selector::CameraSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::camera_selector::CameraSelector {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_QObject_ptr(self as *mut ::camera_selector::CameraSelector)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_QObject_ptr(self as *const ::camera_selector::CameraSelector as *mut ::camera_selector::CameraSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera_selector::CameraSelector> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera_selector::CameraSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera_selector::CameraSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera_selector::CameraSelector> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera_selector::CameraSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera_selector::CameraSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::camera_selector::CameraSelector> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::camera_selector::CameraSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::camera_selector::CameraSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::camera_selector::CameraSelector {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::camera_selector::CameraSelector as *mut ::camera_selector::CameraSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::camera_selector::CameraSelector {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::camera_selector::CameraSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

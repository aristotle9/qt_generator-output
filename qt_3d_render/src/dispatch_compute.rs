/// C++ type: <span style='color: green;'>```Qt3DRender::QDispatchCompute```</span>
#[repr(C)]
pub struct DispatchCompute(u8);

impl DispatchCompute {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QDispatchCompute::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_metaObject(self as *const ::dispatch_compute::DispatchCompute)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QDispatchCompute::QDispatchCompute()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::dispatch_compute::DispatchCompute> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QDispatchCompute::QDispatchCompute(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::dispatch_compute::DispatchCompute> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QDispatchCompute::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_qt_metacall(self as *mut ::dispatch_compute::DispatchCompute,
                                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                                  arg2,
                                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QDispatchCompute::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_qt_metacast(self as *mut ::dispatch_compute::DispatchCompute,
                                                                  arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QDispatchCompute::setWorkGroupX(int workGroupX)```</span>
  ///
  ///
  pub fn set_work_group_x(&mut self, work_group_x: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_setWorkGroupX(self as *mut ::dispatch_compute::DispatchCompute, work_group_x)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QDispatchCompute::setWorkGroupY(int workGroupY)```</span>
  ///
  ///
  pub fn set_work_group_y(&mut self, work_group_y: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_setWorkGroupY(self as *mut ::dispatch_compute::DispatchCompute, work_group_y)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QDispatchCompute::setWorkGroupZ(int workGroupZ)```</span>
  ///
  ///
  pub fn set_work_group_z(&mut self, work_group_z: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_setWorkGroupZ(self as *mut ::dispatch_compute::DispatchCompute, work_group_z)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QDispatchCompute::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QDispatchCompute::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QDispatchCompute::workGroupX() const```</span>
  ///
  ///
  pub fn work_group_x(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_workGroupX(self as *const ::dispatch_compute::DispatchCompute)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QDispatchCompute::workGroupY() const```</span>
  ///
  ///
  pub fn work_group_y(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_workGroupY(self as *const ::dispatch_compute::DispatchCompute)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QDispatchCompute::workGroupZ() const```</span>
  ///
  ///
  pub fn work_group_z(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_workGroupZ(self as *const ::dispatch_compute::DispatchCompute)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::dispatch_compute::DispatchCompute {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QDispatchCompute_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DispatchCompute`.
  pub struct Signals<'a>(&'a ::dispatch_compute::DispatchCompute);
  /// Represents a built-in Qt signal `Qt3DRender::QDispatchCompute::workGroupXChanged`.
  ///
  /// An object of this type can be created from `DispatchCompute` with `object.signals().work_group_x_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DispatchCompute` object.
  pub struct WorkGroupXChanged<'a>(&'a ::dispatch_compute::DispatchCompute);
  impl<'a> ::qt_core::connection::Receiver for WorkGroupXChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2workGroupXChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorkGroupXChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QDispatchCompute::workGroupYChanged`.
  ///
  /// An object of this type can be created from `DispatchCompute` with `object.signals().work_group_y_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DispatchCompute` object.
  pub struct WorkGroupYChanged<'a>(&'a ::dispatch_compute::DispatchCompute);
  impl<'a> ::qt_core::connection::Receiver for WorkGroupYChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2workGroupYChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorkGroupYChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QDispatchCompute::workGroupZChanged`.
  ///
  /// An object of this type can be created from `DispatchCompute` with `object.signals().work_group_z_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DispatchCompute` object.
  pub struct WorkGroupZChanged<'a>(&'a ::dispatch_compute::DispatchCompute);
  impl<'a> ::qt_core::connection::Receiver for WorkGroupZChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2workGroupZChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WorkGroupZChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QDispatchCompute::workGroupXChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn work_group_x_changed(&self) -> WorkGroupXChanged {
      WorkGroupXChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QDispatchCompute::workGroupYChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn work_group_y_changed(&self) -> WorkGroupYChanged {
      WorkGroupYChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QDispatchCompute::workGroupZChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn work_group_z_changed(&self) -> WorkGroupZChanged {
      WorkGroupZChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DispatchCompute`.
  pub struct Slots<'a>(&'a ::dispatch_compute::DispatchCompute);
  /// Represents a built-in Qt slot `Qt3DRender::QDispatchCompute::setWorkGroupZ`.
  ///
  /// An object of this type can be created from `DispatchCompute` with `object.slots().set_work_group_z()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DispatchCompute` object.
  pub struct SetWorkGroupZ<'a>(&'a ::dispatch_compute::DispatchCompute);
  impl<'a> ::qt_core::connection::Receiver for SetWorkGroupZ<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWorkGroupZ(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QDispatchCompute::setWorkGroupY`.
  ///
  /// An object of this type can be created from `DispatchCompute` with `object.slots().set_work_group_y()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DispatchCompute` object.
  pub struct SetWorkGroupY<'a>(&'a ::dispatch_compute::DispatchCompute);
  impl<'a> ::qt_core::connection::Receiver for SetWorkGroupY<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWorkGroupY(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QDispatchCompute::setWorkGroupX`.
  ///
  /// An object of this type can be created from `DispatchCompute` with `object.slots().set_work_group_x()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DispatchCompute` object.
  pub struct SetWorkGroupX<'a>(&'a ::dispatch_compute::DispatchCompute);
  impl<'a> ::qt_core::connection::Receiver for SetWorkGroupX<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWorkGroupX(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QDispatchCompute::setWorkGroupZ`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_work_group_z(&self) -> SetWorkGroupZ {
      SetWorkGroupZ(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QDispatchCompute::setWorkGroupY`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_work_group_y(&self) -> SetWorkGroupY {
      SetWorkGroupY(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QDispatchCompute::setWorkGroupX`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_work_group_x(&self) -> SetWorkGroupX {
      SetWorkGroupX(self.0)
    }
  }
  impl ::dispatch_compute::DispatchCompute {
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

impl ::cpp_utils::DynamicCast<::dispatch_compute::DispatchCompute> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dispatch_compute::DispatchCompute> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_dynamic_cast_Qt3DRender_QDispatchCompute_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::dispatch_compute::DispatchCompute> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_dynamic_cast_Qt3DRender_QDispatchCompute_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::dispatch_compute::DispatchCompute {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::dispatch_compute::DispatchCompute as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::dispatch_compute::DispatchCompute {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::dispatch_compute::DispatchCompute as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::dispatch_compute::DispatchCompute {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_QObject_ptr(self as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_QObject_ptr(self as *const ::dispatch_compute::DispatchCompute as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dispatch_compute::DispatchCompute> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dispatch_compute::DispatchCompute {
    let ffi_result = ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dispatch_compute::DispatchCompute {
    let ffi_result = ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dispatch_compute::DispatchCompute> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dispatch_compute::DispatchCompute {
    let ffi_result = ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dispatch_compute::DispatchCompute {
    let ffi_result = ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dispatch_compute::DispatchCompute> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dispatch_compute::DispatchCompute {
    let ffi_result = ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dispatch_compute::DispatchCompute {
    let ffi_result = ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::dispatch_compute::DispatchCompute {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::dispatch_compute::DispatchCompute as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dispatch_compute::DispatchCompute {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::dispatch_compute::DispatchCompute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

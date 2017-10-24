/// C++ type: <span style='color: green;'>```Qt3DRender::QCullFace```</span>
#[repr(C)]
pub struct CullFace(u8);

impl CullFace {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QCullFace::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_metaObject(self as *const ::cull_face::CullFace) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QCullFace::CullingMode Qt3DRender::QCullFace::mode() const```</span>
  ///
  ///
  pub fn mode(&self) -> ::cull_face::CullingMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_mode(self as *const ::cull_face::CullFace) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCullFace::QCullFace()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::cull_face::CullFace> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QCullFace::QCullFace(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::cull_face::CullFace> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QCullFace::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_qt_metacall(self as *mut ::cull_face::CullFace,
                                                           arg1 as *const ::qt_core::meta_object::Call,
                                                           arg2,
                                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QCullFace::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_qt_metacast(self as *mut ::cull_face::CullFace, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QCullFace::setMode(Qt3DRender::QCullFace::CullingMode mode)```</span>
  ///
  ///
  pub fn set_mode(&mut self, mode: ::cull_face::CullingMode) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_setMode(self as *mut ::cull_face::CullFace, mode) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCullFace::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QCullFace::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::cull_face::CullFace {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QCullFace_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CullFace`.
  pub struct Signals<'a>(&'a ::cull_face::CullFace);
  /// Represents a built-in Qt signal `Qt3DRender::QCullFace::modeChanged`.
  ///
  /// An object of this type can be created from `CullFace` with `object.signals().mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CullFace` object.
  pub struct ModeChanged<'a>(&'a ::cull_face::CullFace);
  impl<'a> ::qt_core::connection::Receiver for ModeChanged<'a> {
    type Arguments = (::cull_face::CullingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modeChanged(Qt3DRender::QCullFace::CullingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ModeChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QCullFace::modeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mode_changed(&self) -> ModeChanged {
      ModeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CullFace`.
  pub struct Slots<'a>(&'a ::cull_face::CullFace);
  /// Represents a built-in Qt slot `Qt3DRender::QCullFace::setMode`.
  ///
  /// An object of this type can be created from `CullFace` with `object.slots().set_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CullFace` object.
  pub struct SetMode<'a>(&'a ::cull_face::CullFace);
  impl<'a> ::qt_core::connection::Receiver for SetMode<'a> {
    type Arguments = (::cull_face::CullingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMode(Qt3DRender::QCullFace::CullingMode)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QCullFace::setMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mode(&self) -> SetMode {
      SetMode(self.0)
    }
  }
  impl ::cull_face::CullFace {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QCullFace::CullingMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CullingMode {
  /// C++ enum variant: <span style='color: green;'>```NoCulling = 0```</span>
  NoCulling = 0,
  /// C++ enum variant: <span style='color: green;'>```Front = 1028```</span>
  Front = 1028,
  /// C++ enum variant: <span style='color: green;'>```Back = 1029```</span>
  Back = 1029,
  /// C++ enum variant: <span style='color: green;'>```FrontAndBack = 1032```</span>
  FrontAndBack = 1032,
}

impl ::cpp_utils::DynamicCast<::cull_face::CullFace> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::cull_face::CullFace> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCullFace_G_dynamic_cast_Qt3DRender_QCullFace_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::cull_face::CullFace> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCullFace_G_dynamic_cast_Qt3DRender_QCullFace_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::cull_face::CullFace {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::cull_face::CullFace) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::cull_face::CullFace as *mut ::cull_face::CullFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::cull_face::CullFace {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::cull_face::CullFace)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::cull_face::CullFace as *mut ::cull_face::CullFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::cull_face::CullFace {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QCullFace_G_static_cast_QObject_ptr(self as *mut ::cull_face::CullFace) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCullFace_G_static_cast_QObject_ptr(self as *const ::cull_face::CullFace as *mut ::cull_face::CullFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cull_face::CullFace> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cull_face::CullFace {
    let ffi_result = ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cull_face::CullFace {
    let ffi_result = ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cull_face::CullFace> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cull_face::CullFace {
    let ffi_result = ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cull_face::CullFace {
    let ffi_result = ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cull_face::CullFace> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cull_face::CullFace {
    let ffi_result = ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cull_face::CullFace {
    let ffi_result = ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QCullFace_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::cull_face::CullFace {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::cull_face::CullFace as *mut ::cull_face::CullFace) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::cull_face::CullFace {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QCullFace_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::cull_face::CullFace)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

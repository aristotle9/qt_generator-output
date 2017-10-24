/// C++ type: <span style='color: green;'>```Qt3DRender::QPolygonOffset```</span>
#[repr(C)]
pub struct PolygonOffset(u8);

impl PolygonOffset {
  /// C++ method: <span style='color: green;'>```float Qt3DRender::QPolygonOffset::depthSteps() const```</span>
  ///
  ///
  pub fn depth_steps(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_depthSteps(self as *const ::polygon_offset::PolygonOffset)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QPolygonOffset::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_metaObject(self as *const ::polygon_offset::PolygonOffset)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPolygonOffset::QPolygonOffset()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::polygon_offset::PolygonOffset> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPolygonOffset::QPolygonOffset(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::polygon_offset::PolygonOffset> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QPolygonOffset::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_qt_metacall(self as *mut ::polygon_offset::PolygonOffset,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QPolygonOffset::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_qt_metacast(self as *mut ::polygon_offset::PolygonOffset, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QPolygonOffset::scaleFactor() const```</span>
  ///
  ///
  pub fn scale_factor(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_scaleFactor(self as *const ::polygon_offset::PolygonOffset)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPolygonOffset::setDepthSteps(float depthSteps)```</span>
  ///
  ///
  pub fn set_depth_steps(&mut self, depth_steps: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_setDepthSteps(self as *mut ::polygon_offset::PolygonOffset,
                                                                    depth_steps)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPolygonOffset::setScaleFactor(float scaleFactor)```</span>
  ///
  ///
  pub fn set_scale_factor(&mut self, scale_factor: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_setScaleFactor(self as *mut ::polygon_offset::PolygonOffset,
                                                                     scale_factor)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPolygonOffset::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPolygonOffset::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::polygon_offset::PolygonOffset {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QPolygonOffset_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PolygonOffset`.
  pub struct Signals<'a>(&'a ::polygon_offset::PolygonOffset);
  /// Represents a built-in Qt signal `Qt3DRender::QPolygonOffset::depthStepsChanged`.
  ///
  /// An object of this type can be created from `PolygonOffset` with `object.signals().depth_steps_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PolygonOffset` object.
  pub struct DepthStepsChanged<'a>(&'a ::polygon_offset::PolygonOffset);
  impl<'a> ::qt_core::connection::Receiver for DepthStepsChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2depthStepsChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DepthStepsChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPolygonOffset::scaleFactorChanged`.
  ///
  /// An object of this type can be created from `PolygonOffset` with `object.signals().scale_factor_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PolygonOffset` object.
  pub struct ScaleFactorChanged<'a>(&'a ::polygon_offset::PolygonOffset);
  impl<'a> ::qt_core::connection::Receiver for ScaleFactorChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scaleFactorChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScaleFactorChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPolygonOffset::depthStepsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn depth_steps_changed(&self) -> DepthStepsChanged {
      DepthStepsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPolygonOffset::scaleFactorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_factor_changed(&self) -> ScaleFactorChanged {
      ScaleFactorChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PolygonOffset`.
  pub struct Slots<'a>(&'a ::polygon_offset::PolygonOffset);
  /// Represents a built-in Qt slot `Qt3DRender::QPolygonOffset::setScaleFactor`.
  ///
  /// An object of this type can be created from `PolygonOffset` with `object.slots().set_scale_factor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PolygonOffset` object.
  pub struct SetScaleFactor<'a>(&'a ::polygon_offset::PolygonOffset);
  impl<'a> ::qt_core::connection::Receiver for SetScaleFactor<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setScaleFactor(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPolygonOffset::setDepthSteps`.
  ///
  /// An object of this type can be created from `PolygonOffset` with `object.slots().set_depth_steps()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PolygonOffset` object.
  pub struct SetDepthSteps<'a>(&'a ::polygon_offset::PolygonOffset);
  impl<'a> ::qt_core::connection::Receiver for SetDepthSteps<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDepthSteps(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPolygonOffset::setScaleFactor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_scale_factor(&self) -> SetScaleFactor {
      SetScaleFactor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPolygonOffset::setDepthSteps`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_depth_steps(&self) -> SetDepthSteps {
      SetDepthSteps(self.0)
    }
  }
  impl ::polygon_offset::PolygonOffset {
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

impl ::cpp_utils::DynamicCast<::polygon_offset::PolygonOffset> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::polygon_offset::PolygonOffset> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_dynamic_cast_Qt3DRender_QPolygonOffset_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::polygon_offset::PolygonOffset> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_dynamic_cast_Qt3DRender_QPolygonOffset_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::polygon_offset::PolygonOffset {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::polygon_offset::PolygonOffset) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::polygon_offset::PolygonOffset as *mut ::polygon_offset::PolygonOffset) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::polygon_offset::PolygonOffset {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::polygon_offset::PolygonOffset) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::polygon_offset::PolygonOffset as *mut ::polygon_offset::PolygonOffset) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::polygon_offset::PolygonOffset {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_QObject_ptr(self as *mut ::polygon_offset::PolygonOffset)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_QObject_ptr(self as *const ::polygon_offset::PolygonOffset as *mut ::polygon_offset::PolygonOffset) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::polygon_offset::PolygonOffset> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::polygon_offset::PolygonOffset {
    let ffi_result = ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::polygon_offset::PolygonOffset {
    let ffi_result = ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::polygon_offset::PolygonOffset> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::polygon_offset::PolygonOffset {
    let ffi_result = ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::polygon_offset::PolygonOffset {
    let ffi_result = ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::polygon_offset::PolygonOffset> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::polygon_offset::PolygonOffset {
    let ffi_result = ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::polygon_offset::PolygonOffset {
    let ffi_result = ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::polygon_offset::PolygonOffset {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::polygon_offset::PolygonOffset as *mut ::polygon_offset::PolygonOffset) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::polygon_offset::PolygonOffset {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::polygon_offset::PolygonOffset) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

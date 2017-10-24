/// C++ type: <span style='color: green;'>```Qt3DRender::QViewport```</span>
#[repr(C)]
pub struct Viewport(u8);

impl Viewport {
  /// C++ method: <span style='color: green;'>```float Qt3DRender::QViewport::gamma() const```</span>
  ///
  ///
  pub fn gamma(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QViewport_gamma(self as *const ::viewport::Viewport) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QViewport::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QViewport_metaObject(self as *const ::viewport::Viewport) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QViewport::QViewport()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::viewport::Viewport> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QViewport_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QViewport::QViewport(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::viewport::Viewport> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QViewport_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QRectF Qt3DRender::QViewport::normalizedRect() const```</span>
  ///
  ///
  pub fn normalized_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QViewport_normalizedRect_to_output(self as *const ::viewport::Viewport,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QViewport::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QViewport_qt_metacall(self as *mut ::viewport::Viewport,
                                                           arg1 as *const ::qt_core::meta_object::Call,
                                                           arg2,
                                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QViewport::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QViewport_qt_metacast(self as *mut ::viewport::Viewport, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QViewport::setGamma(float gamma)```</span>
  ///
  ///
  pub fn set_gamma(&mut self, gamma: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QViewport_setGamma(self as *mut ::viewport::Viewport, gamma) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QViewport::setNormalizedRect(const QRectF& normalizedRect)```</span>
  ///
  ///
  pub fn set_normalized_rect(&mut self, normalized_rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QViewport_setNormalizedRect(self as *mut ::viewport::Viewport,
                                                                   normalized_rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QViewport::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QViewport_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QViewport::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QViewport_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::viewport::Viewport {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QViewport_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Viewport`.
  pub struct Signals<'a>(&'a ::viewport::Viewport);
  /// Represents a built-in Qt signal `Qt3DRender::QViewport::normalizedRectChanged`.
  ///
  /// An object of this type can be created from `Viewport` with `object.signals().normalized_rect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Viewport` object.
  pub struct NormalizedRectChanged<'a>(&'a ::viewport::Viewport);
  impl<'a> ::qt_core::connection::Receiver for NormalizedRectChanged<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2normalizedRectChanged(const QRectF&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NormalizedRectChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QViewport::gammaChanged`.
  ///
  /// An object of this type can be created from `Viewport` with `object.signals().gamma_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Viewport` object.
  pub struct GammaChanged<'a>(&'a ::viewport::Viewport);
  impl<'a> ::qt_core::connection::Receiver for GammaChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2gammaChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GammaChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QViewport::normalizedRectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn normalized_rect_changed(&self) -> NormalizedRectChanged {
      NormalizedRectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QViewport::gammaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn gamma_changed(&self) -> GammaChanged {
      GammaChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Viewport`.
  pub struct Slots<'a>(&'a ::viewport::Viewport);
  /// Represents a built-in Qt slot `Qt3DRender::QViewport::setGamma`.
  ///
  /// An object of this type can be created from `Viewport` with `object.slots().set_gamma()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Viewport` object.
  pub struct SetGamma<'a>(&'a ::viewport::Viewport);
  impl<'a> ::qt_core::connection::Receiver for SetGamma<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGamma(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QViewport::setNormalizedRect`.
  ///
  /// An object of this type can be created from `Viewport` with `object.slots().set_normalized_rect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Viewport` object.
  pub struct SetNormalizedRect<'a>(&'a ::viewport::Viewport);
  impl<'a> ::qt_core::connection::Receiver for SetNormalizedRect<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setNormalizedRect(const QRectF&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QViewport::setGamma`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_gamma(&self) -> SetGamma {
      SetGamma(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QViewport::setNormalizedRect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_normalized_rect(&self) -> SetNormalizedRect {
      SetNormalizedRect(self.0)
    }
  }
  impl ::viewport::Viewport {
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

impl ::cpp_utils::DynamicCast<::viewport::Viewport> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::viewport::Viewport> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QViewport_G_dynamic_cast_Qt3DRender_QViewport_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::viewport::Viewport> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QViewport_G_dynamic_cast_Qt3DRender_QViewport_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::viewport::Viewport {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::viewport::Viewport) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::viewport::Viewport as *mut ::viewport::Viewport) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::viewport::Viewport {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::viewport::Viewport)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::viewport::Viewport as *mut ::viewport::Viewport) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::viewport::Viewport {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QViewport_G_static_cast_QObject_ptr(self as *mut ::viewport::Viewport) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QViewport_G_static_cast_QObject_ptr(self as *const ::viewport::Viewport as *mut ::viewport::Viewport) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::viewport::Viewport> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::viewport::Viewport {
    let ffi_result = ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::viewport::Viewport {
    let ffi_result = ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::viewport::Viewport> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::viewport::Viewport {
    let ffi_result = ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::viewport::Viewport {
    let ffi_result = ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::viewport::Viewport> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::viewport::Viewport {
    let ffi_result = ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::viewport::Viewport {
    let ffi_result = ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::viewport::Viewport {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::viewport::Viewport as *mut ::viewport::Viewport) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::viewport::Viewport {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::viewport::Viewport)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

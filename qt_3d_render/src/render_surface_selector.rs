/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderSurfaceSelector```</span>
#[repr(C)]
pub struct RenderSurfaceSelector(u8);

impl RenderSurfaceSelector {
  /// C++ method: <span style='color: green;'>```QSize Qt3DRender::QRenderSurfaceSelector::externalRenderTargetSize() const```</span>
  ///
  ///
  pub fn external_render_target_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_externalRenderTargetSize_to_output(self as *const ::render_surface_selector::RenderSurfaceSelector, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderSurfaceSelector::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_metaObject(self as *const ::render_surface_selector::RenderSurfaceSelector) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderSurfaceSelector::QRenderSurfaceSelector()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_surface_selector::RenderSurfaceSelector> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderSurfaceSelector::QRenderSurfaceSelector(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::render_surface_selector::RenderSurfaceSelector> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderSurfaceSelector::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_qt_metacall(self as *mut ::render_surface_selector::RenderSurfaceSelector, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderSurfaceSelector::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_qt_metacast(self as *mut ::render_surface_selector::RenderSurfaceSelector, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderSurfaceSelector::setExternalRenderTargetSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_external_render_target_size(&mut self, size: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_setExternalRenderTargetSize(self as *mut ::render_surface_selector::RenderSurfaceSelector, size as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderSurfaceSelector::setSurface(QObject* surfaceObject)```</span>
  ///
  ///
  pub unsafe fn set_surface(&mut self, surface_object: *mut ::qt_core::object::Object) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_setSurface(self as *mut ::render_surface_selector::RenderSurfaceSelector, surface_object)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderSurfaceSelector::setSurfacePixelRatio(float ratio)```</span>
  ///
  ///
  pub fn set_surface_pixel_ratio(&mut self, ratio: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_setSurfacePixelRatio(self as *mut ::render_surface_selector::RenderSurfaceSelector, ratio) }
  }

  /// C++ method: <span style='color: green;'>```QObject* Qt3DRender::QRenderSurfaceSelector::surface() const```</span>
  ///
  ///
  pub fn surface(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_surface(self as *const ::render_surface_selector::RenderSurfaceSelector) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QRenderSurfaceSelector::surfacePixelRatio() const```</span>
  ///
  ///
  pub fn surface_pixel_ratio(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_surfacePixelRatio(self as *const ::render_surface_selector::RenderSurfaceSelector) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderSurfaceSelector::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderSurfaceSelector::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_surface_selector::RenderSurfaceSelector {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderSurfaceSelector_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RenderSurfaceSelector`.
  pub struct Signals<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSurfaceSelector::surfacePixelRatioChanged`.
  ///
  /// An object of this type can be created from `RenderSurfaceSelector` with `object.signals().surface_pixel_ratio_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSurfaceSelector` object.
  pub struct SurfacePixelRatioChanged<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  impl<'a> ::qt_core::connection::Receiver for SurfacePixelRatioChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2surfacePixelRatioChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SurfacePixelRatioChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSurfaceSelector::externalRenderTargetSizeChanged`.
  ///
  /// An object of this type can be created from `RenderSurfaceSelector` with `object.signals().external_render_target_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSurfaceSelector` object.
  pub struct ExternalRenderTargetSizeChanged<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  impl<'a> ::qt_core::connection::Receiver for ExternalRenderTargetSizeChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2externalRenderTargetSizeChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ExternalRenderTargetSizeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderSurfaceSelector::surfaceChanged`.
  ///
  /// An object of this type can be created from `RenderSurfaceSelector` with `object.signals().surface_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSurfaceSelector` object.
  pub struct SurfaceChanged<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  impl<'a> ::qt_core::connection::Receiver for SurfaceChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2surfaceChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SurfaceChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSurfaceSelector::surfacePixelRatioChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn surface_pixel_ratio_changed(&self) -> SurfacePixelRatioChanged {
      SurfacePixelRatioChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSurfaceSelector::externalRenderTargetSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn external_render_target_size_changed(&self) -> ExternalRenderTargetSizeChanged {
      ExternalRenderTargetSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderSurfaceSelector::surfaceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn surface_changed(&self) -> SurfaceChanged {
      SurfaceChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `RenderSurfaceSelector`.
  pub struct Slots<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  /// Represents a built-in Qt slot `Qt3DRender::QRenderSurfaceSelector::setExternalRenderTargetSize`.
  ///
  /// An object of this type can be created from `RenderSurfaceSelector` with `object.slots().set_external_render_target_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSurfaceSelector` object.
  pub struct SetExternalRenderTargetSize<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  impl<'a> ::qt_core::connection::Receiver for SetExternalRenderTargetSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setExternalRenderTargetSize(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderSurfaceSelector::setSurfacePixelRatio`.
  ///
  /// An object of this type can be created from `RenderSurfaceSelector` with `object.slots().set_surface_pixel_ratio()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSurfaceSelector` object.
  pub struct SetSurfacePixelRatio<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  impl<'a> ::qt_core::connection::Receiver for SetSurfacePixelRatio<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSurfacePixelRatio(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderSurfaceSelector::setSurface`.
  ///
  /// An object of this type can be created from `RenderSurfaceSelector` with `object.slots().set_surface()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderSurfaceSelector` object.
  pub struct SetSurface<'a>(&'a ::render_surface_selector::RenderSurfaceSelector);
  impl<'a> ::qt_core::connection::Receiver for SetSurface<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSurface(QObject*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderSurfaceSelector::setExternalRenderTargetSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_external_render_target_size(&self) -> SetExternalRenderTargetSize {
      SetExternalRenderTargetSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderSurfaceSelector::setSurfacePixelRatio`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_surface_pixel_ratio(&self) -> SetSurfacePixelRatio {
      SetSurfacePixelRatio(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderSurfaceSelector::setSurface`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_surface(&self) -> SetSurface {
      SetSurface(self.0)
    }
  }
  impl ::render_surface_selector::RenderSurfaceSelector {
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

impl ::cpp_utils::DynamicCast<::render_surface_selector::RenderSurfaceSelector> for ::frame_graph_node::FrameGraphNode {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::render_surface_selector::RenderSurfaceSelector> {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_dynamic_cast_Qt3DRender_QRenderSurfaceSelector_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::render_surface_selector::RenderSurfaceSelector> {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_dynamic_cast_Qt3DRender_QRenderSurfaceSelector_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_surface_selector::RenderSurfaceSelector {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_surface_selector::RenderSurfaceSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_surface_selector::RenderSurfaceSelector as *mut ::render_surface_selector::RenderSurfaceSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::render_surface_selector::RenderSurfaceSelector {
fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_surface_selector::RenderSurfaceSelector) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_surface_selector::RenderSurfaceSelector as *mut ::render_surface_selector::RenderSurfaceSelector) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_surface_selector::RenderSurfaceSelector {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_QObject_ptr(self as *mut ::render_surface_selector::RenderSurfaceSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_QObject_ptr(self as *const ::render_surface_selector::RenderSurfaceSelector as *mut ::render_surface_selector::RenderSurfaceSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_surface_selector::RenderSurfaceSelector> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_surface_selector::RenderSurfaceSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_surface_selector::RenderSurfaceSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_surface_selector::RenderSurfaceSelector> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_surface_selector::RenderSurfaceSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_surface_selector::RenderSurfaceSelector {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_surface_selector::RenderSurfaceSelector> for ::frame_graph_node::FrameGraphNode {
unsafe fn static_cast_mut(&mut self) -> &mut ::render_surface_selector::RenderSurfaceSelector {
let ffi_result = ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::render_surface_selector::RenderSurfaceSelector {
let ffi_result = ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QRenderSurfaceSelector_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::render_surface_selector::RenderSurfaceSelector {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_surface_selector::RenderSurfaceSelector as *mut ::render_surface_selector::RenderSurfaceSelector) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_surface_selector::RenderSurfaceSelector {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderSurfaceSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_surface_selector::RenderSurfaceSelector) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

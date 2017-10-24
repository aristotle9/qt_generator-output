/// C++ type: <span style='color: green;'>```Qt3DExtras::QForwardRenderer```</span>
#[repr(C)]
pub struct ForwardRenderer(u8);

impl ForwardRenderer {
  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* Qt3DExtras::QForwardRenderer::camera() const```</span>
  ///
  ///
  pub fn camera(&self) -> *mut ::qt_3d_core::entity::Entity {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_camera(self as *const ::forward_renderer::ForwardRenderer)
    }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DExtras::QForwardRenderer::clearColor() const```</span>
  ///
  ///
  pub fn clear_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_clearColor_to_output(self as *const ::forward_renderer::ForwardRenderer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QForwardRenderer::externalRenderTargetSize() const```</span>
  ///
  ///
  pub fn external_render_target_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_externalRenderTargetSize_to_output(self as *const ::forward_renderer::ForwardRenderer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QForwardRenderer::gamma() const```</span>
  ///
  ///
  pub fn gamma(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_gamma(self as *const ::forward_renderer::ForwardRenderer)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QForwardRenderer::isFrustumCullingEnabled() const```</span>
  ///
  ///
  pub fn is_frustum_culling_enabled(&self) -> bool {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_isFrustumCullingEnabled(self as *const ::forward_renderer::ForwardRenderer) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QForwardRenderer::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_metaObject(self as *const ::forward_renderer::ForwardRenderer)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QForwardRenderer::QForwardRenderer()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::forward_renderer::ForwardRenderer> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QForwardRenderer::QForwardRenderer(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::forward_renderer::ForwardRenderer> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QForwardRenderer::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_qt_metacall(self as *mut ::forward_renderer::ForwardRenderer,
                                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                                  arg2,
                                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QForwardRenderer::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_qt_metacast(self as *mut ::forward_renderer::ForwardRenderer,
                                                                  arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QForwardRenderer::setCamera(Qt3DCore::QEntity* camera)```</span>
  ///
  ///
  pub unsafe fn set_camera(&mut self, camera: *mut ::qt_3d_core::entity::Entity) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setCamera(self as *mut ::forward_renderer::ForwardRenderer,
                                                                camera)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QForwardRenderer::setClearColor(const QColor& clearColor)```</span>
  ///
  ///
  pub fn set_clear_color(&mut self, clear_color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setClearColor(self as *mut ::forward_renderer::ForwardRenderer, clear_color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QForwardRenderer::setExternalRenderTargetSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_external_render_target_size(&mut self, size: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setExternalRenderTargetSize(self as *mut ::forward_renderer::ForwardRenderer, size as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QForwardRenderer::setFrustumCullingEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_frustum_culling_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setFrustumCullingEnabled(self as *mut ::forward_renderer::ForwardRenderer, enabled) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QForwardRenderer::setGamma(float gamma)```</span>
  ///
  ///
  pub fn set_gamma(&mut self, gamma: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setGamma(self as *mut ::forward_renderer::ForwardRenderer,
                                                                 gamma)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QForwardRenderer::setSurface(QObject* surface)```</span>
  ///
  ///
  pub unsafe fn set_surface(&mut self, surface: *mut ::qt_core::object::Object) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setSurface(self as *mut ::forward_renderer::ForwardRenderer,
                                                                 surface)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QForwardRenderer::setViewportRect(const QRectF& viewportRect)```</span>
  ///
  ///
  pub fn set_viewport_rect(&mut self, viewport_rect: &::qt_core::rect_f::RectF) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_setViewportRect(self as *mut ::forward_renderer::ForwardRenderer, viewport_rect as *const ::qt_core::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```QObject* Qt3DExtras::QForwardRenderer::surface() const```</span>
  ///
  ///
  pub fn surface(&self) -> *mut ::qt_core::object::Object {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_surface(self as *const ::forward_renderer::ForwardRenderer)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QForwardRenderer::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QForwardRenderer::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF Qt3DExtras::QForwardRenderer::viewportRect() const```</span>
  ///
  ///
  pub fn viewport_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_viewportRect_to_output(self as *const ::forward_renderer::ForwardRenderer, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::forward_renderer::ForwardRenderer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QForwardRenderer_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ForwardRenderer`.
  pub struct Signals<'a>(&'a ::forward_renderer::ForwardRenderer);
  /// Represents a built-in Qt signal `Qt3DExtras::QForwardRenderer::surfaceChanged`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.signals().surface_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SurfaceChanged<'a>(&'a ::forward_renderer::ForwardRenderer);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QForwardRenderer::clearColorChanged`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.signals().clear_color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct ClearColorChanged<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for ClearColorChanged<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clearColorChanged(const QColor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ClearColorChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QForwardRenderer::frustumCullingEnabledChanged`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.signals().frustum_culling_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct FrustumCullingEnabledChanged<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for FrustumCullingEnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2frustumCullingEnabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FrustumCullingEnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QForwardRenderer::gammaChanged`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.signals().gamma_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct GammaChanged<'a>(&'a ::forward_renderer::ForwardRenderer);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QForwardRenderer::externalRenderTargetSizeChanged`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.signals().external_render_target_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct ExternalRenderTargetSizeChanged<'a>(&'a ::forward_renderer::ForwardRenderer);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QForwardRenderer::viewportRectChanged`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.signals().viewport_rect_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct ViewportRectChanged<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for ViewportRectChanged<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2viewportRectChanged(const QRectF&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ViewportRectChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QForwardRenderer::cameraChanged`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.signals().camera_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct CameraChanged<'a>(&'a ::forward_renderer::ForwardRenderer);
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
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QForwardRenderer::surfaceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn surface_changed(&self) -> SurfaceChanged {
      SurfaceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QForwardRenderer::clearColorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_color_changed(&self) -> ClearColorChanged {
      ClearColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QForwardRenderer::frustumCullingEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn frustum_culling_enabled_changed(&self) -> FrustumCullingEnabledChanged {
      FrustumCullingEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QForwardRenderer::gammaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn gamma_changed(&self) -> GammaChanged {
      GammaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QForwardRenderer::externalRenderTargetSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn external_render_target_size_changed(&self) -> ExternalRenderTargetSizeChanged {
      ExternalRenderTargetSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QForwardRenderer::viewportRectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn viewport_rect_changed(&self) -> ViewportRectChanged {
      ViewportRectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QForwardRenderer::cameraChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn camera_changed(&self) -> CameraChanged {
      CameraChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ForwardRenderer`.
  pub struct Slots<'a>(&'a ::forward_renderer::ForwardRenderer);
  /// Represents a built-in Qt slot `Qt3DExtras::QForwardRenderer::setSurface`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.slots().set_surface()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SetSurface<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetSurface<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSurface(QObject*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QForwardRenderer::setFrustumCullingEnabled`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.slots().set_frustum_culling_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SetFrustumCullingEnabled<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetFrustumCullingEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFrustumCullingEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QForwardRenderer::setViewportRect`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.slots().set_viewport_rect()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SetViewportRect<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetViewportRect<'a> {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setViewportRect(const QRectF&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QForwardRenderer::setCamera`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.slots().set_camera()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SetCamera<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetCamera<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCamera(Qt3DCore::QEntity*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QForwardRenderer::setClearColor`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.slots().set_clear_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SetClearColor<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetClearColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setClearColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QForwardRenderer::setGamma`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.slots().set_gamma()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SetGamma<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetGamma<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGamma(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QForwardRenderer::setExternalRenderTargetSize`.
  ///
  /// An object of this type can be created from `ForwardRenderer` with `object.slots().set_external_render_target_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ForwardRenderer` object.
  pub struct SetExternalRenderTargetSize<'a>(&'a ::forward_renderer::ForwardRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetExternalRenderTargetSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setExternalRenderTargetSize(const QSize&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QForwardRenderer::setSurface`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_surface(&self) -> SetSurface {
      SetSurface(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QForwardRenderer::setFrustumCullingEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_frustum_culling_enabled(&self) -> SetFrustumCullingEnabled {
      SetFrustumCullingEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QForwardRenderer::setViewportRect`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_viewport_rect(&self) -> SetViewportRect {
      SetViewportRect(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QForwardRenderer::setCamera`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_camera(&self) -> SetCamera {
      SetCamera(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QForwardRenderer::setClearColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_clear_color(&self) -> SetClearColor {
      SetClearColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QForwardRenderer::setGamma`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_gamma(&self) -> SetGamma {
      SetGamma(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QForwardRenderer::setExternalRenderTargetSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_external_render_target_size(&self) -> SetExternalRenderTargetSize {
      SetExternalRenderTargetSize(self.0)
    }
  }
  impl ::forward_renderer::ForwardRenderer {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::forward_renderer::ForwardRenderer {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::forward_renderer::ForwardRenderer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::forward_renderer::ForwardRenderer as *mut ::forward_renderer::ForwardRenderer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::frame_graph_node::FrameGraphNode> for ::forward_renderer::ForwardRenderer {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::frame_graph_node::FrameGraphNode {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::forward_renderer::ForwardRenderer) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_render::frame_graph_node::FrameGraphNode {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::forward_renderer::ForwardRenderer as *mut ::forward_renderer::ForwardRenderer) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_render::technique_filter::TechniqueFilter> for ::forward_renderer::ForwardRenderer {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::technique_filter::TechniqueFilter {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QTechniqueFilter_ptr(self as *mut ::forward_renderer::ForwardRenderer) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_render::technique_filter::TechniqueFilter {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QTechniqueFilter_ptr(self as *const ::forward_renderer::ForwardRenderer as *mut ::forward_renderer::ForwardRenderer) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::forward_renderer::ForwardRenderer {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_QObject_ptr(self as *mut ::forward_renderer::ForwardRenderer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_QObject_ptr(self as *const ::forward_renderer::ForwardRenderer as *mut ::forward_renderer::ForwardRenderer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::forward_renderer::ForwardRenderer> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::forward_renderer::ForwardRenderer {
    let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::forward_renderer::ForwardRenderer {
    let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::forward_renderer::ForwardRenderer> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::forward_renderer::ForwardRenderer {
    let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::forward_renderer::ForwardRenderer {
    let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::forward_renderer::ForwardRenderer> for ::qt_3d_render::frame_graph_node::FrameGraphNode {
unsafe fn static_cast_mut(&mut self) -> &mut ::forward_renderer::ForwardRenderer {
let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::qt_3d_render::frame_graph_node::FrameGraphNode);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::forward_renderer::ForwardRenderer {
let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DRender_QFrameGraphNode(self as *const ::qt_3d_render::frame_graph_node::FrameGraphNode as *mut ::qt_3d_render::frame_graph_node::FrameGraphNode);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::forward_renderer::ForwardRenderer> for ::qt_3d_render::technique_filter::TechniqueFilter {
unsafe fn static_cast_mut(&mut self) -> &mut ::forward_renderer::ForwardRenderer {
let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DRender_QTechniqueFilter(self as *mut ::qt_3d_render::technique_filter::TechniqueFilter);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::forward_renderer::ForwardRenderer {
let ffi_result = ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DExtras_QForwardRenderer_ptr_Qt3DRender_QTechniqueFilter(self as *const ::qt_3d_render::technique_filter::TechniqueFilter as *mut ::qt_3d_render::technique_filter::TechniqueFilter);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::forward_renderer::ForwardRenderer {
  type Target = ::qt_3d_render::technique_filter::TechniqueFilter;
  fn deref(&self) -> &::qt_3d_render::technique_filter::TechniqueFilter {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QTechniqueFilter_ptr(self as *const ::forward_renderer::ForwardRenderer as *mut ::forward_renderer::ForwardRenderer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::forward_renderer::ForwardRenderer {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::technique_filter::TechniqueFilter {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QForwardRenderer_G_static_cast_Qt3DRender_QTechniqueFilter_ptr(self as *mut ::forward_renderer::ForwardRenderer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

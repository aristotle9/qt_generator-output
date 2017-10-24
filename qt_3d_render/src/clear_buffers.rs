/// C++ type: <span style='color: green;'>```Qt3DRender::QClearBuffers::BufferType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BufferType {
  /// C++ enum variant: <span style='color: green;'>```AllBuffers = -1```</span>
  AllBuffers = -1,
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```ColorBuffer = 1```</span>
  ColorBuffer = 1,
  /// C++ enum variant: <span style='color: green;'>```DepthBuffer = 2```</span>
  DepthBuffer = 2,
  /// C++ enum variant: <span style='color: green;'>```ColorDepthBuffer = 3```</span>
  ColorDepthBuffer = 3,
  /// C++ enum variant: <span style='color: green;'>```StencilBuffer = 4```</span>
  StencilBuffer = 4,
  /// C++ enum variant: <span style='color: green;'>```DepthStencilBuffer = 6```</span>
  DepthStencilBuffer = 6,
  /// C++ enum variant: <span style='color: green;'>```ColorDepthStencilBuffer = 7```</span>
  ColorDepthStencilBuffer = 7,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QClearBuffers```</span>
#[repr(C)]
pub struct ClearBuffers(u8);

impl ClearBuffers {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QClearBuffers::BufferType Qt3DRender::QClearBuffers::buffers() const```</span>
  ///
  ///
  pub fn buffers(&self) -> ::clear_buffers::BufferType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_buffers(self as *const ::clear_buffers::ClearBuffers) }
  }

  /// C++ method: <span style='color: green;'>```QColor Qt3DRender::QClearBuffers::clearColor() const```</span>
  ///
  ///
  pub fn clear_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_clearColor_to_output(self as *const ::clear_buffers::ClearBuffers, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QClearBuffers::clearDepthValue() const```</span>
  ///
  ///
  pub fn clear_depth_value(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_clearDepthValue(self as *const ::clear_buffers::ClearBuffers)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QClearBuffers::clearStencilValue() const```</span>
  ///
  ///
  pub fn clear_stencil_value(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_clearStencilValue(self as *const ::clear_buffers::ClearBuffers)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* Qt3DRender::QClearBuffers::colorBuffer() const```</span>
  ///
  ///
  pub fn color_buffer(&self) -> *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_colorBuffer(self as *const ::clear_buffers::ClearBuffers) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QClearBuffers::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_metaObject(self as *const ::clear_buffers::ClearBuffers) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QClearBuffers::QClearBuffers()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::clear_buffers::ClearBuffers> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QClearBuffers::QClearBuffers(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::clear_buffers::ClearBuffers> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QClearBuffers::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_qt_metacall(self as *mut ::clear_buffers::ClearBuffers,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QClearBuffers::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_qt_metacast(self as *mut ::clear_buffers::ClearBuffers, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClearBuffers::setBuffers(Qt3DRender::QClearBuffers::BufferType buffers)```</span>
  ///
  ///
  pub fn set_buffers(&mut self, buffers: ::clear_buffers::BufferType) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_setBuffers(self as *mut ::clear_buffers::ClearBuffers, buffers)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClearBuffers::setClearColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_clear_color(&mut self, color: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_setClearColor(self as *mut ::clear_buffers::ClearBuffers,
                                                                   color as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClearBuffers::setClearDepthValue(float clearDepthValue)```</span>
  ///
  ///
  pub fn set_clear_depth_value(&mut self, clear_depth_value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_setClearDepthValue(self as *mut ::clear_buffers::ClearBuffers,
                                                                        clear_depth_value)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClearBuffers::setClearStencilValue(int clearStencilValue)```</span>
  ///
  ///
  pub fn set_clear_stencil_value(&mut self, clear_stencil_value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_setClearStencilValue(self as *mut ::clear_buffers::ClearBuffers,
                                                                          clear_stencil_value)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QClearBuffers::setColorBuffer(Qt3DRender::QRenderTargetOutput* buffer)```</span>
  ///
  ///
  pub unsafe fn set_color_buffer(&mut self, buffer: *mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_setColorBuffer(self as *mut ::clear_buffers::ClearBuffers, buffer)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QClearBuffers::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QClearBuffers::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::clear_buffers::ClearBuffers {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QClearBuffers_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ClearBuffers`.
  pub struct Signals<'a>(&'a ::clear_buffers::ClearBuffers);
  /// Represents a built-in Qt signal `Qt3DRender::QClearBuffers::clearDepthValueChanged`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.signals().clear_depth_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct ClearDepthValueChanged<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for ClearDepthValueChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clearDepthValueChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ClearDepthValueChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QClearBuffers::colorBufferChanged`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.signals().color_buffer_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct ColorBufferChanged<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for ColorBufferChanged<'a> {
    type Arguments = (*mut ::render_target_output::RenderTargetOutput,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2colorBufferChanged(Qt3DRender::QRenderTargetOutput*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColorBufferChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QClearBuffers::clearColorChanged`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.signals().clear_color_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct ClearColorChanged<'a>(&'a ::clear_buffers::ClearBuffers);
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
  /// Represents a built-in Qt signal `Qt3DRender::QClearBuffers::buffersChanged`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.signals().buffers_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct BuffersChanged<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for BuffersChanged<'a> {
    type Arguments = (::clear_buffers::BufferType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buffersChanged(Qt3DRender::QClearBuffers::BufferType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BuffersChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QClearBuffers::clearStencilValueChanged`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.signals().clear_stencil_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct ClearStencilValueChanged<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for ClearStencilValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clearStencilValueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ClearStencilValueChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClearBuffers::clearDepthValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_depth_value_changed(&self) -> ClearDepthValueChanged {
      ClearDepthValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClearBuffers::colorBufferChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn color_buffer_changed(&self) -> ColorBufferChanged {
      ColorBufferChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClearBuffers::clearColorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_color_changed(&self) -> ClearColorChanged {
      ClearColorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClearBuffers::buffersChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn buffers_changed(&self) -> BuffersChanged {
      BuffersChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QClearBuffers::clearStencilValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_stencil_value_changed(&self) -> ClearStencilValueChanged {
      ClearStencilValueChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ClearBuffers`.
  pub struct Slots<'a>(&'a ::clear_buffers::ClearBuffers);
  /// Represents a built-in Qt slot `Qt3DRender::QClearBuffers::setColorBuffer`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.slots().set_color_buffer()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct SetColorBuffer<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for SetColorBuffer<'a> {
    type Arguments = (*mut ::render_target_output::RenderTargetOutput,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setColorBuffer(Qt3DRender::QRenderTargetOutput*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QClearBuffers::setClearDepthValue`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.slots().set_clear_depth_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct SetClearDepthValue<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for SetClearDepthValue<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setClearDepthValue(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QClearBuffers::setClearStencilValue`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.slots().set_clear_stencil_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct SetClearStencilValue<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for SetClearStencilValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setClearStencilValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QClearBuffers::setBuffers`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.slots().set_buffers()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct SetBuffers<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for SetBuffers<'a> {
    type Arguments = (::clear_buffers::BufferType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBuffers(Qt3DRender::QClearBuffers::BufferType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QClearBuffers::setClearColor`.
  ///
  /// An object of this type can be created from `ClearBuffers` with `object.slots().set_clear_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ClearBuffers` object.
  pub struct SetClearColor<'a>(&'a ::clear_buffers::ClearBuffers);
  impl<'a> ::qt_core::connection::Receiver for SetClearColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setClearColor(const QColor&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClearBuffers::setColorBuffer`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_color_buffer(&self) -> SetColorBuffer {
      SetColorBuffer(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClearBuffers::setClearDepthValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_clear_depth_value(&self) -> SetClearDepthValue {
      SetClearDepthValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClearBuffers::setClearStencilValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_clear_stencil_value(&self) -> SetClearStencilValue {
      SetClearStencilValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClearBuffers::setBuffers`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_buffers(&self) -> SetBuffers {
      SetBuffers(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QClearBuffers::setClearColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_clear_color(&self) -> SetClearColor {
      SetClearColor(self.0)
    }
  }
  impl ::clear_buffers::ClearBuffers {
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

impl ::cpp_utils::DynamicCast<::clear_buffers::ClearBuffers> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::clear_buffers::ClearBuffers> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_dynamic_cast_Qt3DRender_QClearBuffers_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::clear_buffers::ClearBuffers> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_dynamic_cast_Qt3DRender_QClearBuffers_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::clear_buffers::ClearBuffers {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::clear_buffers::ClearBuffers)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::clear_buffers::ClearBuffers as *mut ::clear_buffers::ClearBuffers) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::clear_buffers::ClearBuffers {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::clear_buffers::ClearBuffers) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::clear_buffers::ClearBuffers as *mut ::clear_buffers::ClearBuffers) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::clear_buffers::ClearBuffers {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_QObject_ptr(self as *mut ::clear_buffers::ClearBuffers)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_QObject_ptr(self as *const ::clear_buffers::ClearBuffers as *mut ::clear_buffers::ClearBuffers) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::clear_buffers::ClearBuffers> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::clear_buffers::ClearBuffers {
    let ffi_result = ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::clear_buffers::ClearBuffers {
    let ffi_result = ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::clear_buffers::ClearBuffers> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::clear_buffers::ClearBuffers {
    let ffi_result = ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::clear_buffers::ClearBuffers {
    let ffi_result = ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::clear_buffers::ClearBuffers> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::clear_buffers::ClearBuffers {
    let ffi_result = ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::clear_buffers::ClearBuffers {
    let ffi_result = ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QClearBuffers_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::clear_buffers::ClearBuffers {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::clear_buffers::ClearBuffers as *mut ::clear_buffers::ClearBuffers) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::clear_buffers::ClearBuffers {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QClearBuffers_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::clear_buffers::ClearBuffers) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

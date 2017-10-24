/// C++ type: <span style='color: green;'>```Qt3DRender::QBlendEquationArguments```</span>
#[repr(C)]
pub struct BlendEquationArguments(u8);

impl BlendEquationArguments {
  /// C++ method: <span style='color: green;'>```int Qt3DRender::QBlendEquationArguments::bufferIndex() const```</span>
  ///
  ///
  pub fn buffer_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_bufferIndex(self as *const ::blend_equation_arguments::BlendEquationArguments) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBlendEquationArguments::Blending Qt3DRender::QBlendEquationArguments::destinationAlpha() const```</span>
  ///
  ///
  pub fn destination_alpha(&self) -> ::blend_equation_arguments::Blending {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_destinationAlpha(self as *const ::blend_equation_arguments::BlendEquationArguments) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBlendEquationArguments::Blending Qt3DRender::QBlendEquationArguments::destinationRgb() const```</span>
  ///
  ///
  pub fn destination_rgb(&self) -> ::blend_equation_arguments::Blending {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_destinationRgb(self as *const ::blend_equation_arguments::BlendEquationArguments) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QBlendEquationArguments::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_metaObject(self as *const ::blend_equation_arguments::BlendEquationArguments) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBlendEquationArguments::QBlendEquationArguments()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::blend_equation_arguments::BlendEquationArguments> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBlendEquationArguments::QBlendEquationArguments(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::blend_equation_arguments::BlendEquationArguments> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QBlendEquationArguments::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_qt_metacall(self as *mut ::blend_equation_arguments::BlendEquationArguments, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QBlendEquationArguments::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_qt_metacast(self as *mut ::blend_equation_arguments::BlendEquationArguments, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquationArguments::setBufferIndex(int index)```</span>
  ///
  ///
  pub fn set_buffer_index(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_setBufferIndex(self as *mut ::blend_equation_arguments::BlendEquationArguments, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquationArguments::setDestinationAlpha(Qt3DRender::QBlendEquationArguments::Blending destinationAlpha)```</span>
  ///
  ///
  pub fn set_destination_alpha(&mut self, destination_alpha: ::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_setDestinationAlpha(self as *mut ::blend_equation_arguments::BlendEquationArguments, destination_alpha) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquationArguments::setDestinationRgb(Qt3DRender::QBlendEquationArguments::Blending destinationRgb)```</span>
  ///
  ///
  pub fn set_destination_rgb(&mut self, destination_rgb: ::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_setDestinationRgb(self as *mut ::blend_equation_arguments::BlendEquationArguments, destination_rgb) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquationArguments::setDestinationRgba(Qt3DRender::QBlendEquationArguments::Blending destinationRgba)```</span>
  ///
  ///
  pub fn set_destination_rgba(&mut self, destination_rgba: ::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_setDestinationRgba(self as *mut ::blend_equation_arguments::BlendEquationArguments, destination_rgba) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquationArguments::setSourceAlpha(Qt3DRender::QBlendEquationArguments::Blending sourceAlpha)```</span>
  ///
  ///
  pub fn set_source_alpha(&mut self, source_alpha: ::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_setSourceAlpha(self as *mut ::blend_equation_arguments::BlendEquationArguments, source_alpha) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquationArguments::setSourceRgb(Qt3DRender::QBlendEquationArguments::Blending sourceRgb)```</span>
  ///
  ///
  pub fn set_source_rgb(&mut self, source_rgb: ::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_setSourceRgb(self as *mut ::blend_equation_arguments::BlendEquationArguments, source_rgb) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBlendEquationArguments::setSourceRgba(Qt3DRender::QBlendEquationArguments::Blending sourceRgba)```</span>
  ///
  ///
  pub fn set_source_rgba(&mut self, source_rgba: ::blend_equation_arguments::Blending) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_setSourceRgba(self as *mut ::blend_equation_arguments::BlendEquationArguments, source_rgba) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBlendEquationArguments::Blending Qt3DRender::QBlendEquationArguments::sourceAlpha() const```</span>
  ///
  ///
  pub fn source_alpha(&self) -> ::blend_equation_arguments::Blending {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_sourceAlpha(self as *const ::blend_equation_arguments::BlendEquationArguments) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBlendEquationArguments::Blending Qt3DRender::QBlendEquationArguments::sourceRgb() const```</span>
  ///
  ///
  pub fn source_rgb(&self) -> ::blend_equation_arguments::Blending {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_sourceRgb(self as *const ::blend_equation_arguments::BlendEquationArguments) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBlendEquationArguments::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBlendEquationArguments::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::blend_equation_arguments::BlendEquationArguments {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QBlendEquationArguments_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `BlendEquationArguments`.
  pub struct Signals<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquationArguments::sourceAlphaChanged`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.signals().source_alpha_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SourceAlphaChanged<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SourceAlphaChanged<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceAlphaChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceAlphaChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquationArguments::destinationRgbChanged`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.signals().destination_rgb_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct DestinationRgbChanged<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for DestinationRgbChanged<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2destinationRgbChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DestinationRgbChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquationArguments::destinationRgbaChanged`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.signals().destination_rgba_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct DestinationRgbaChanged<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for DestinationRgbaChanged<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2destinationRgbaChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DestinationRgbaChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquationArguments::destinationAlphaChanged`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.signals().destination_alpha_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct DestinationAlphaChanged<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for DestinationAlphaChanged<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2destinationAlphaChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DestinationAlphaChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquationArguments::sourceRgbaChanged`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.signals().source_rgba_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SourceRgbaChanged<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SourceRgbaChanged<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceRgbaChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceRgbaChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquationArguments::sourceRgbChanged`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.signals().source_rgb_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SourceRgbChanged<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SourceRgbChanged<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceRgbChanged(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceRgbChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBlendEquationArguments::bufferIndexChanged`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.signals().buffer_index_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct BufferIndexChanged<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for BufferIndexChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bufferIndexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BufferIndexChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquationArguments::sourceAlphaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_alpha_changed(&self) -> SourceAlphaChanged {
      SourceAlphaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquationArguments::destinationRgbChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn destination_rgb_changed(&self) -> DestinationRgbChanged {
      DestinationRgbChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquationArguments::destinationRgbaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn destination_rgba_changed(&self) -> DestinationRgbaChanged {
      DestinationRgbaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquationArguments::destinationAlphaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn destination_alpha_changed(&self) -> DestinationAlphaChanged {
      DestinationAlphaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquationArguments::sourceRgbaChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_rgba_changed(&self) -> SourceRgbaChanged {
      SourceRgbaChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquationArguments::sourceRgbChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_rgb_changed(&self) -> SourceRgbChanged {
      SourceRgbChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBlendEquationArguments::bufferIndexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn buffer_index_changed(&self) -> BufferIndexChanged {
      BufferIndexChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `BlendEquationArguments`.
  pub struct Slots<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setSourceRgb`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.slots().set_source_rgb()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SetSourceRgb<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetSourceRgb<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceRgb(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setDestinationRgb`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.slots().set_destination_rgb()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SetDestinationRgb<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetDestinationRgb<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDestinationRgb(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setDestinationAlpha`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.slots().set_destination_alpha()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SetDestinationAlpha<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetDestinationAlpha<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDestinationAlpha(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setBufferIndex`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.slots().set_buffer_index()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SetBufferIndex<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetBufferIndex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBufferIndex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setDestinationRgba`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.slots().set_destination_rgba()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SetDestinationRgba<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetDestinationRgba<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDestinationRgba(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setSourceAlpha`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.slots().set_source_alpha()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SetSourceAlpha<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetSourceAlpha<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceAlpha(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setSourceRgba`.
  ///
  /// An object of this type can be created from `BlendEquationArguments` with `object.slots().set_source_rgba()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `BlendEquationArguments` object.
  pub struct SetSourceRgba<'a>(&'a ::blend_equation_arguments::BlendEquationArguments);
  impl<'a> ::qt_core::connection::Receiver for SetSourceRgba<'a> {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceRgba(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setSourceRgb`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_rgb(&self) -> SetSourceRgb {
      SetSourceRgb(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setDestinationRgb`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_destination_rgb(&self) -> SetDestinationRgb {
      SetDestinationRgb(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setDestinationAlpha`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_destination_alpha(&self) -> SetDestinationAlpha {
      SetDestinationAlpha(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setBufferIndex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_buffer_index(&self) -> SetBufferIndex {
      SetBufferIndex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setDestinationRgba`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_destination_rgba(&self) -> SetDestinationRgba {
      SetDestinationRgba(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setSourceAlpha`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_alpha(&self) -> SetSourceAlpha {
      SetSourceAlpha(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBlendEquationArguments::setSourceRgba`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_rgba(&self) -> SetSourceRgba {
      SetSourceRgba(self.0)
    }
  }
  impl ::blend_equation_arguments::BlendEquationArguments {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QBlendEquationArguments::Blending```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Blending {
  /// C++ enum variant: <span style='color: green;'>```Zero = 0```</span>
  Zero = 0,
  /// C++ enum variant: <span style='color: green;'>```One = 1```</span>
  One = 1,
  /// C++ enum variant: <span style='color: green;'>```SourceColor = 768```</span>
  SourceColor = 768,
  /// C++ enum variant: <span style='color: green;'>```OneMinusSourceColor = 769```</span>
  OneMinusSourceColor = 769,
  /// C++ enum variant: <span style='color: green;'>```SourceAlpha = 770```</span>
  SourceAlpha = 770,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Source1Alpha = 771```</span>
  /// - <span style='color: green;'>```OneMinusSourceAlpha = 771```</span>
  ///
  Source1Alpha = 771,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Source1Color = 772```</span>
  /// - <span style='color: green;'>```DestinationAlpha = 772```</span>
  ///
  Source1Color = 772,
  /// C++ enum variant: <span style='color: green;'>```OneMinusDestinationAlpha = 773```</span>
  OneMinusDestinationAlpha = 773,
  /// C++ enum variant: <span style='color: green;'>```DestinationColor = 774```</span>
  DestinationColor = 774,
  /// C++ enum variant: <span style='color: green;'>```OneMinusDestinationColor = 775```</span>
  OneMinusDestinationColor = 775,
  /// C++ enum variant: <span style='color: green;'>```SourceAlphaSaturate = 776```</span>
  SourceAlphaSaturate = 776,
  /// C++ enum variant: <span style='color: green;'>```ConstantColor = 32769```</span>
  ConstantColor = 32769,
  /// C++ enum variant: <span style='color: green;'>```OneMinusConstantColor = 32770```</span>
  OneMinusConstantColor = 32770,
  /// C++ enum variant: <span style='color: green;'>```ConstantAlpha = 32771```</span>
  ConstantAlpha = 32771,
  /// C++ enum variant: <span style='color: green;'>```OneMinusConstantAlpha = 32772```</span>
  OneMinusConstantAlpha = 32772,
  /// C++ enum variant: <span style='color: green;'>```OneMinusSource1Alpha = 32773```</span>
  OneMinusSource1Alpha = 32773,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```OneMinusSource1Color = 32774```</span>
  /// - <span style='color: green;'>```OneMinusSource1Color0 = 32774```</span>
  ///
  OneMinusSource1Color = 32774,
}

impl ::cpp_utils::DynamicCast<::blend_equation_arguments::BlendEquationArguments> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::blend_equation_arguments::BlendEquationArguments> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_dynamic_cast_Qt3DRender_QBlendEquationArguments_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::blend_equation_arguments::BlendEquationArguments> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_dynamic_cast_Qt3DRender_QBlendEquationArguments_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::blend_equation_arguments::BlendEquationArguments {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::blend_equation_arguments::BlendEquationArguments as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::blend_equation_arguments::BlendEquationArguments {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::blend_equation_arguments::BlendEquationArguments as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::blend_equation_arguments::BlendEquationArguments {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_QObject_ptr(self as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_QObject_ptr(self as *const ::blend_equation_arguments::BlendEquationArguments as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::blend_equation_arguments::BlendEquationArguments> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::blend_equation_arguments::BlendEquationArguments {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QBlendEquationArguments_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::blend_equation_arguments::BlendEquationArguments {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QBlendEquationArguments_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::blend_equation_arguments::BlendEquationArguments> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::blend_equation_arguments::BlendEquationArguments {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QBlendEquationArguments_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::blend_equation_arguments::BlendEquationArguments {
    let ffi_result = ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QBlendEquationArguments_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::blend_equation_arguments::BlendEquationArguments> for ::render_state::RenderState {
unsafe fn static_cast_mut(&mut self) -> &mut ::blend_equation_arguments::BlendEquationArguments {
let ffi_result = ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QBlendEquationArguments_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::blend_equation_arguments::BlendEquationArguments {
let ffi_result = ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QBlendEquationArguments_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::blend_equation_arguments::BlendEquationArguments {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::blend_equation_arguments::BlendEquationArguments as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::blend_equation_arguments::BlendEquationArguments {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBlendEquationArguments_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::blend_equation_arguments::BlendEquationArguments) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

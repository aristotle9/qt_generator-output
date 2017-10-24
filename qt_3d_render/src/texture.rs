/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureLoader```</span>
#[repr(C)]
pub struct TextureLoader(u8);

impl TextureLoader {
  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QTextureLoader::isMirrored() const```</span>
  ///
  ///
  pub fn is_mirrored(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_isMirrored(self as *const ::texture::TextureLoader) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QTextureLoader::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_metaObject(self as *const ::texture::TextureLoader) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureLoader::QTextureLoader()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::texture::TextureLoader> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureLoader::QTextureLoader(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::texture::TextureLoader> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QTextureLoader::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_qt_metacall(self as *mut ::texture::TextureLoader,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QTextureLoader::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_qt_metacast(self as *mut ::texture::TextureLoader, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QTextureLoader::setMirrored(bool mirrored)```</span>
  ///
  ///
  pub fn set_mirrored(&mut self, mirrored: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_setMirrored(self as *mut ::texture::TextureLoader, mirrored)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QTextureLoader::setSource(const QUrl& source)```</span>
  ///
  ///
  pub fn set_source(&mut self, source: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_setSource(self as *mut ::texture::TextureLoader,
                                                                source as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl Qt3DRender::QTextureLoader::source() const```</span>
  ///
  ///
  pub fn source(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_source_to_output(self as *const ::texture::TextureLoader,
                                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTextureLoader::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTextureLoader::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::texture::TextureLoader {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureLoader_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextureLoader`.
  pub struct Signals<'a>(&'a ::texture::TextureLoader);
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::generateMipMapsChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().generate_mip_maps_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct GenerateMipMapsChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for GenerateMipMapsChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2generateMipMapsChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GenerateMipMapsChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::magnificationFilterChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().magnification_filter_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct MagnificationFilterChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for MagnificationFilterChanged<'a> {
    type Arguments = (::abstract_texture::Filter,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2magnificationFilterChanged(Qt3DRender::QAbstractTexture::Filter)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MagnificationFilterChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::comparisonModeChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().comparison_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct ComparisonModeChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for ComparisonModeChanged<'a> {
    type Arguments = (::abstract_texture::ComparisonMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2comparisonModeChanged(Qt3DRender::QAbstractTexture::ComparisonMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ComparisonModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::samplesChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().samples_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SamplesChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SamplesChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2samplesChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SamplesChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::statusChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().status_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct StatusChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for StatusChanged<'a> {
    type Arguments = (::abstract_texture::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2statusChanged(Qt3DRender::QAbstractTexture::Status)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StatusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::comparisonFunctionChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().comparison_function_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct ComparisonFunctionChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for ComparisonFunctionChanged<'a> {
    type Arguments = (::abstract_texture::ComparisonFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2comparisonFunctionChanged(Qt3DRender::QAbstractTexture::ComparisonFunction)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ComparisonFunctionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::maximumAnisotropyChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().maximum_anisotropy_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct MaximumAnisotropyChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for MaximumAnisotropyChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2maximumAnisotropyChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MaximumAnisotropyChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::layersChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().layers_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct LayersChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for LayersChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layersChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LayersChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::minificationFilterChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().minification_filter_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct MinificationFilterChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for MinificationFilterChanged<'a> {
    type Arguments = (::abstract_texture::Filter,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2minificationFilterChanged(Qt3DRender::QAbstractTexture::Filter)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MinificationFilterChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::heightChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct HeightChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::sourceChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().source_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SourceChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SourceChanged<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceChanged(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::depthChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().depth_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct DepthChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for DepthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2depthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DepthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::formatChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().format_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct FormatChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for FormatChanged<'a> {
    type Arguments = (::abstract_texture::TextureFormat,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2formatChanged(Qt3DRender::QAbstractTexture::TextureFormat)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FormatChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::widthChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct WidthChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureLoader::mirroredChanged`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.signals().mirrored_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct MirroredChanged<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for MirroredChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mirroredChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MirroredChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::generateMipMapsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn generate_mip_maps_changed(&self) -> GenerateMipMapsChanged {
      GenerateMipMapsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::magnificationFilterChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn magnification_filter_changed(&self) -> MagnificationFilterChanged {
      MagnificationFilterChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::comparisonModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn comparison_mode_changed(&self) -> ComparisonModeChanged {
      ComparisonModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::samplesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn samples_changed(&self) -> SamplesChanged {
      SamplesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::statusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn status_changed(&self) -> StatusChanged {
      StatusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::comparisonFunctionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn comparison_function_changed(&self) -> ComparisonFunctionChanged {
      ComparisonFunctionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::maximumAnisotropyChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_anisotropy_changed(&self) -> MaximumAnisotropyChanged {
      MaximumAnisotropyChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::layersChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layers_changed(&self) -> LayersChanged {
      LayersChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::minificationFilterChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minification_filter_changed(&self) -> MinificationFilterChanged {
      MinificationFilterChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::sourceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_changed(&self) -> SourceChanged {
      SourceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::depthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn depth_changed(&self) -> DepthChanged {
      DepthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::formatChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn format_changed(&self) -> FormatChanged {
      FormatChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureLoader::mirroredChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mirrored_changed(&self) -> MirroredChanged {
      MirroredChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TextureLoader`.
  pub struct Slots<'a>(&'a ::texture::TextureLoader);
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setMinificationFilter`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_minification_filter()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetMinificationFilter<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetMinificationFilter<'a> {
    type Arguments = (::abstract_texture::Filter,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinificationFilter(Qt3DRender::QAbstractTexture::Filter)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setMagnificationFilter`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_magnification_filter()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetMagnificationFilter<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetMagnificationFilter<'a> {
    type Arguments = (::abstract_texture::Filter,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMagnificationFilter(Qt3DRender::QAbstractTexture::Filter)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setSource`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_source()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetSource<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetSource<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSource(const QUrl&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setFormat`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_format()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetFormat<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetFormat<'a> {
    type Arguments = (::abstract_texture::TextureFormat,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFormat(Qt3DRender::QAbstractTexture::TextureFormat)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setComparisonMode`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_comparison_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetComparisonMode<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetComparisonMode<'a> {
    type Arguments = (::abstract_texture::ComparisonMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setComparisonMode(Qt3DRender::QAbstractTexture::ComparisonMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setMirrored`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_mirrored()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetMirrored<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetMirrored<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMirrored(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setLayers`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_layers()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetLayers<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetLayers<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLayers(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setGenerateMipMaps`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_generate_mip_maps()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetGenerateMipMaps<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetGenerateMipMaps<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGenerateMipMaps(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setMaximumAnisotropy`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_maximum_anisotropy()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetMaximumAnisotropy<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumAnisotropy<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumAnisotropy(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setHeight`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetHeight<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setDepth`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_depth()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetDepth<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetDepth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDepth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setComparisonFunction`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_comparison_function()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetComparisonFunction<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetComparisonFunction<'a> {
    type Arguments = (::abstract_texture::ComparisonFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setComparisonFunction(Qt3DRender::QAbstractTexture::ComparisonFunction)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setSamples`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_samples()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetSamples<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetSamples<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSamples(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureLoader::setWidth`.
  ///
  /// An object of this type can be created from `TextureLoader` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureLoader` object.
  pub struct SetWidth<'a>(&'a ::texture::TextureLoader);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setMinificationFilter`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minification_filter(&self) -> SetMinificationFilter {
      SetMinificationFilter(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setMagnificationFilter`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_magnification_filter(&self) -> SetMagnificationFilter {
      SetMagnificationFilter(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setSource`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source(&self) -> SetSource {
      SetSource(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setFormat`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_format(&self) -> SetFormat {
      SetFormat(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setComparisonMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_comparison_mode(&self) -> SetComparisonMode {
      SetComparisonMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setMirrored`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mirrored(&self) -> SetMirrored {
      SetMirrored(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setLayers`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_layers(&self) -> SetLayers {
      SetLayers(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setGenerateMipMaps`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_generate_mip_maps(&self) -> SetGenerateMipMaps {
      SetGenerateMipMaps(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setMaximumAnisotropy`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_anisotropy(&self) -> SetMaximumAnisotropy {
      SetMaximumAnisotropy(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setDepth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_depth(&self) -> SetDepth {
      SetDepth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setComparisonFunction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_comparison_function(&self) -> SetComparisonFunction {
      SetComparisonFunction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setSamples`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_samples(&self) -> SetSamples {
      SetSamples(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureLoader::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
  }
  impl ::texture::TextureLoader {
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

impl ::cpp_utils::DynamicCast<::texture::TextureLoader> for ::abstract_texture::AbstractTexture {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::texture::TextureLoader> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_dynamic_cast_Qt3DRender_QTextureLoader_ptr(self as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::texture::TextureLoader> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_dynamic_cast_Qt3DRender_QTextureLoader_ptr(self as *const ::abstract_texture::AbstractTexture as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::texture::TextureLoader {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::texture::TextureLoader as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_texture::AbstractTexture> for ::texture::TextureLoader {
  fn static_cast_mut(&mut self) -> &mut ::abstract_texture::AbstractTexture {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr(self as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_texture::AbstractTexture {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr(self as *const ::texture::TextureLoader as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::texture::TextureLoader {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_QObject_ptr(self as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_QObject_ptr(self as *const ::texture::TextureLoader as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture::TextureLoader> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture::TextureLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture::TextureLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture::TextureLoader> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture::TextureLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture::TextureLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture::TextureLoader> for ::abstract_texture::AbstractTexture {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture::TextureLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DRender_QAbstractTexture(self as *mut ::abstract_texture::AbstractTexture);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture::TextureLoader {
    let ffi_result = ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QTextureLoader_ptr_Qt3DRender_QAbstractTexture(self as *const ::abstract_texture::AbstractTexture as *mut ::abstract_texture::AbstractTexture);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::texture::TextureLoader {
  type Target = ::abstract_texture::AbstractTexture;
  fn deref(&self) -> &::abstract_texture::AbstractTexture {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr(self as *const ::texture::TextureLoader as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::texture::TextureLoader {
  fn deref_mut(&mut self) -> &mut ::abstract_texture::AbstractTexture {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr(self as *mut ::texture::TextureLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

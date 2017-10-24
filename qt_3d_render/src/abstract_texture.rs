/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture```</span>
#[repr(C)]
pub struct AbstractTexture(u8);

impl AbstractTexture {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QAbstractTexture::addTextureImage(Qt3DRender::QAbstractTextureImage* textureImage)```</span>
  ///
  ///
  pub unsafe fn add_texture_image(&mut self, texture_image: *mut ::abstract_texture_image::AbstractTextureImage) {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_addTextureImage(self as *mut ::abstract_texture::AbstractTexture, texture_image)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::ComparisonFunction Qt3DRender::QAbstractTexture::comparisonFunction() const```</span>
  ///
  ///
  pub fn comparison_function(&self) -> ::abstract_texture::ComparisonFunction {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_comparisonFunction(self as *const ::abstract_texture::AbstractTexture) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::ComparisonMode Qt3DRender::QAbstractTexture::comparisonMode() const```</span>
  ///
  ///
  pub fn comparison_mode(&self) -> ::abstract_texture::ComparisonMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_comparisonMode(self as *const ::abstract_texture::AbstractTexture) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureGenerator> Qt3DRender::QAbstractTexture::dataGenerator() const```</span>
  ///
  ///
  pub fn data_generator(&self) -> ::shared_pointer::SharedPointerTextureGenerator {
    {
      let mut object: ::shared_pointer::SharedPointerTextureGenerator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_dataGenerator_to_output(self as *const ::abstract_texture::AbstractTexture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QAbstractTexture::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_depth(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::TextureFormat Qt3DRender::QAbstractTexture::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::abstract_texture::TextureFormat {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_format(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QAbstractTexture::generateMipMaps() const```</span>
  ///
  ///
  pub fn generate_mip_maps(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_generateMipMaps(self as *const ::abstract_texture::AbstractTexture) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QAbstractTexture::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_height(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QAbstractTexture::layers() const```</span>
  ///
  ///
  pub fn layers(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_layers(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::Filter Qt3DRender::QAbstractTexture::magnificationFilter() const```</span>
  ///
  ///
  pub fn magnification_filter(&self) -> ::abstract_texture::Filter {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_magnificationFilter(self as *const ::abstract_texture::AbstractTexture) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QAbstractTexture::maximumAnisotropy() const```</span>
  ///
  ///
  pub fn maximum_anisotropy(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_maximumAnisotropy(self as *const ::abstract_texture::AbstractTexture) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QAbstractTexture::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_metaObject(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::Filter Qt3DRender::QAbstractTexture::minificationFilter() const```</span>
  ///
  ///
  pub fn minification_filter(&self) -> ::abstract_texture::Filter {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_minificationFilter(self as *const ::abstract_texture::AbstractTexture) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QAbstractTexture::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_qt_metacall(self as *mut ::abstract_texture::AbstractTexture,
                                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                                  arg2,
                                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QAbstractTexture::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_qt_metacast(self as *mut ::abstract_texture::AbstractTexture,
                                                                  arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QAbstractTexture::removeTextureImage(Qt3DRender::QAbstractTextureImage* textureImage)```</span>
  ///
  ///
  pub unsafe fn remove_texture_image(&mut self, texture_image: *mut ::abstract_texture_image::AbstractTextureImage) {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_removeTextureImage(self as *mut ::abstract_texture::AbstractTexture, texture_image)
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QAbstractTexture::samples() const```</span>
  ///
  ///
  pub fn samples(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_samples(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setComparisonFunction(Qt3DRender::QAbstractTexture::ComparisonFunction function)```</span>
  ///
  ///
  pub fn set_comparison_function(&mut self, function: ::abstract_texture::ComparisonFunction) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setComparisonFunction(self as *mut ::abstract_texture::AbstractTexture, function) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setComparisonMode(Qt3DRender::QAbstractTexture::ComparisonMode mode)```</span>
  ///
  ///
  pub fn set_comparison_mode(&mut self, mode: ::abstract_texture::ComparisonMode) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setComparisonMode(self as *mut ::abstract_texture::AbstractTexture, mode) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setDepth(int depth)```</span>
  ///
  ///
  pub fn set_depth(&mut self, depth: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setDepth(self as *mut ::abstract_texture::AbstractTexture,
                                                                 depth)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setFormat(Qt3DRender::QAbstractTexture::TextureFormat format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: ::abstract_texture::TextureFormat) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setFormat(self as *mut ::abstract_texture::AbstractTexture,
                                                                  format)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setGenerateMipMaps(bool gen)```</span>
  ///
  ///
  pub fn set_generate_mip_maps(&mut self, gen: bool) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setGenerateMipMaps(self as *mut ::abstract_texture::AbstractTexture, gen) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setHeight(int height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setHeight(self as *mut ::abstract_texture::AbstractTexture,
                                                                  height)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setLayers(int layers)```</span>
  ///
  ///
  pub fn set_layers(&mut self, layers: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setLayers(self as *mut ::abstract_texture::AbstractTexture,
                                                                  layers)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setMagnificationFilter(Qt3DRender::QAbstractTexture::Filter f)```</span>
  ///
  ///
  pub fn set_magnification_filter(&mut self, f: ::abstract_texture::Filter) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setMagnificationFilter(self as *mut ::abstract_texture::AbstractTexture, f) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setMaximumAnisotropy(float anisotropy)```</span>
  ///
  ///
  pub fn set_maximum_anisotropy(&mut self, anisotropy: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setMaximumAnisotropy(self as *mut ::abstract_texture::AbstractTexture, anisotropy) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setMinificationFilter(Qt3DRender::QAbstractTexture::Filter f)```</span>
  ///
  ///
  pub fn set_minification_filter(&mut self, f: ::abstract_texture::Filter) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setMinificationFilter(self as *mut ::abstract_texture::AbstractTexture, f) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setSamples(int samples)```</span>
  ///
  ///
  pub fn set_samples(&mut self, samples: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setSamples(self as *mut ::abstract_texture::AbstractTexture,
                                                                   samples)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::setSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_size(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QAbstractTexture::setSize(int width)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_size(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QAbstractTexture::setSize(int width, int height = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_size(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QAbstractTexture::setSize(int width, int height = ?, int depth = ?)```</span>
  ///
  ///
  pub fn set_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::AbstractTextureSetSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTexture::setWidth(int width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setWidth(self as *mut ::abstract_texture::AbstractTexture,
                                                                 width)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QAbstractTexture::setWrapMode(const Qt3DRender::QTextureWrapMode& wrapMode)```</span>
  ///
  ///
  pub fn set_wrap_mode(&mut self, wrap_mode: &::texture_wrap_mode::TextureWrapMode) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setWrapMode(self as *mut ::abstract_texture::AbstractTexture, wrap_mode as *const ::texture_wrap_mode::TextureWrapMode) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::Status Qt3DRender::QAbstractTexture::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::abstract_texture::Status {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_status(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture::Target Qt3DRender::QAbstractTexture::target() const```</span>
  ///
  ///
  pub fn target(&self) -> ::abstract_texture::Target {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_target(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*> Qt3DRender::QAbstractTexture::textureImages() const```</span>
  ///
  ///
  pub fn texture_images(&self) -> ::vector::VectorAbstractTextureImageMutPtr {
    {
      let mut object: ::vector::VectorAbstractTextureImageMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_textureImages_to_output(self as *const ::abstract_texture::AbstractTexture, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAbstractTexture::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAbstractTexture::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QAbstractTexture::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_width(self as *const ::abstract_texture::AbstractTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureWrapMode* Qt3DRender::QAbstractTexture::wrapMode()```</span>
  ///
  ///
  pub fn wrap_mode(&mut self) -> *mut ::texture_wrap_mode::TextureWrapMode {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_wrapMode(self as *mut ::abstract_texture::AbstractTexture)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_texture::AbstractTexture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractTexture`.
  pub struct Signals<'a>(&'a ::abstract_texture::AbstractTexture);
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::depthChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().depth_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct DepthChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::comparisonModeChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().comparison_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct ComparisonModeChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::minificationFilterChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().minification_filter_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct MinificationFilterChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::generateMipMapsChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().generate_mip_maps_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct GenerateMipMapsChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::parentChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct ParentChanged<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::comparisonFunctionChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().comparison_function_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct ComparisonFunctionChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::samplesChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().samples_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SamplesChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::statusChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().status_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct StatusChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::widthChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct WidthChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::layersChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().layers_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct LayersChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::maximumAnisotropyChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().maximum_anisotropy_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct MaximumAnisotropyChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::magnificationFilterChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().magnification_filter_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct MagnificationFilterChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::enabledChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct EnabledChanged<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::nodeDestroyed`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct NodeDestroyed<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for NodeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nodeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NodeDestroyed<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::heightChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct HeightChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTexture::formatChanged`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.signals().format_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct FormatChanged<'a>(&'a ::abstract_texture::AbstractTexture);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::depthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn depth_changed(&self) -> DepthChanged {
      DepthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::comparisonModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn comparison_mode_changed(&self) -> ComparisonModeChanged {
      ComparisonModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::minificationFilterChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minification_filter_changed(&self) -> MinificationFilterChanged {
      MinificationFilterChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::generateMipMapsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn generate_mip_maps_changed(&self) -> GenerateMipMapsChanged {
      GenerateMipMapsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::comparisonFunctionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn comparison_function_changed(&self) -> ComparisonFunctionChanged {
      ComparisonFunctionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::samplesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn samples_changed(&self) -> SamplesChanged {
      SamplesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::statusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn status_changed(&self) -> StatusChanged {
      StatusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::layersChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layers_changed(&self) -> LayersChanged {
      LayersChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::maximumAnisotropyChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn maximum_anisotropy_changed(&self) -> MaximumAnisotropyChanged {
      MaximumAnisotropyChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::magnificationFilterChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn magnification_filter_changed(&self) -> MagnificationFilterChanged {
      MagnificationFilterChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTexture::formatChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn format_changed(&self) -> FormatChanged {
      FormatChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractTexture`.
  pub struct Slots<'a>(&'a ::abstract_texture::AbstractTexture);
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setDepth`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_depth()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetDepth<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetDepth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDepth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setComparisonMode`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_comparison_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetComparisonMode<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetComparisonMode<'a> {
    type Arguments = (::abstract_texture::ComparisonMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setComparisonMode(Qt3DRender::QAbstractTexture::ComparisonMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setComparisonFunction`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_comparison_function()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetComparisonFunction<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetComparisonFunction<'a> {
    type Arguments = (::abstract_texture::ComparisonFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setComparisonFunction(Qt3DRender::QAbstractTexture::ComparisonFunction)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setParent`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetParent<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setFormat`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_format()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetFormat<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetFormat<'a> {
    type Arguments = (::abstract_texture::TextureFormat,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFormat(Qt3DRender::QAbstractTexture::TextureFormat)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setSamples`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_samples()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetSamples<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetSamples<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSamples(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setMagnificationFilter`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_magnification_filter()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetMagnificationFilter<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetMagnificationFilter<'a> {
    type Arguments = (::abstract_texture::Filter,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMagnificationFilter(Qt3DRender::QAbstractTexture::Filter)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setMaximumAnisotropy`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_maximum_anisotropy()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetMaximumAnisotropy<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetMaximumAnisotropy<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMaximumAnisotropy(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setWidth`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetWidth<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setMinificationFilter`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_minification_filter()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetMinificationFilter<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetMinificationFilter<'a> {
    type Arguments = (::abstract_texture::Filter,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinificationFilter(Qt3DRender::QAbstractTexture::Filter)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setEnabled`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetEnabled<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setLayers`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_layers()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetLayers<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetLayers<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLayers(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setGenerateMipMaps`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_generate_mip_maps()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetGenerateMipMaps<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetGenerateMipMaps<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGenerateMipMaps(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTexture::setHeight`.
  ///
  /// An object of this type can be created from `AbstractTexture` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTexture` object.
  pub struct SetHeight<'a>(&'a ::abstract_texture::AbstractTexture);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setDepth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_depth(&self) -> SetDepth {
      SetDepth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setComparisonMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_comparison_mode(&self) -> SetComparisonMode {
      SetComparisonMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setComparisonFunction`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_comparison_function(&self) -> SetComparisonFunction {
      SetComparisonFunction(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setFormat`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_format(&self) -> SetFormat {
      SetFormat(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setSamples`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_samples(&self) -> SetSamples {
      SetSamples(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setMagnificationFilter`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_magnification_filter(&self) -> SetMagnificationFilter {
      SetMagnificationFilter(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setMaximumAnisotropy`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_maximum_anisotropy(&self) -> SetMaximumAnisotropy {
      SetMaximumAnisotropy(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setMinificationFilter`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minification_filter(&self) -> SetMinificationFilter {
      SetMinificationFilter(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setLayers`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_layers(&self) -> SetLayers {
      SetLayers(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setGenerateMipMaps`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_generate_mip_maps(&self) -> SetGenerateMipMaps {
      SetGenerateMipMaps(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTexture::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
  }
  impl ::abstract_texture::AbstractTexture {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture::ComparisonFunction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ComparisonFunction {
  /// C++ enum variant: <span style='color: green;'>```CompareNever = 512```</span>
  CompareNever = 512,
  /// C++ enum variant: <span style='color: green;'>```CompareLess = 513```</span>
  CompareLess = 513,
  /// C++ enum variant: <span style='color: green;'>```CompareEqual = 514```</span>
  CompareEqual = 514,
  /// C++ enum variant: <span style='color: green;'>```CompareLessEqual = 515```</span>
  CompareLessEqual = 515,
  /// C++ enum variant: <span style='color: green;'>```CompareGreater = 516```</span>
  CompareGreater = 516,
  /// C++ enum variant: <span style='color: green;'>```CommpareNotEqual = 517```</span>
  CommpareNotEqual = 517,
  /// C++ enum variant: <span style='color: green;'>```CompareGreaterEqual = 518```</span>
  CompareGreaterEqual = 518,
  /// C++ enum variant: <span style='color: green;'>```CompareAlways = 519```</span>
  CompareAlways = 519,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture::ComparisonMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ComparisonMode {
  /// C++ enum variant: <span style='color: green;'>```CompareNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```CompareRefToTexture = 34894```</span>
  RefToTexture = 34894,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture::CubeMapFace```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CubeMapFace {
  /// C++ enum variant: <span style='color: green;'>```CubeMapPositiveX = 34069```</span>
  CubeMapPositiveX = 34069,
  /// C++ enum variant: <span style='color: green;'>```CubeMapNegativeX = 34070```</span>
  CubeMapNegativeX = 34070,
  /// C++ enum variant: <span style='color: green;'>```CubeMapPositiveY = 34071```</span>
  CubeMapPositiveY = 34071,
  /// C++ enum variant: <span style='color: green;'>```CubeMapNegativeY = 34072```</span>
  CubeMapNegativeY = 34072,
  /// C++ enum variant: <span style='color: green;'>```CubeMapPositiveZ = 34073```</span>
  CubeMapPositiveZ = 34073,
  /// C++ enum variant: <span style='color: green;'>```CubeMapNegativeZ = 34074```</span>
  CubeMapNegativeZ = 34074,
  /// C++ enum variant: <span style='color: green;'>```AllFaces = 34075```</span>
  AllFaces = 34075,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture::Filter```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Filter {
  /// C++ enum variant: <span style='color: green;'>```Nearest = 9728```</span>
  Nearest = 9728,
  /// C++ enum variant: <span style='color: green;'>```Linear = 9729```</span>
  Linear = 9729,
  /// C++ enum variant: <span style='color: green;'>```NearestMipMapNearest = 9984```</span>
  NearestMipMapNearest = 9984,
  /// C++ enum variant: <span style='color: green;'>```LinearMipMapNearest = 9985```</span>
  LinearMipMapNearest = 9985,
  /// C++ enum variant: <span style='color: green;'>```NearestMipMapLinear = 9986```</span>
  NearestMipMapLinear = 9986,
  /// C++ enum variant: <span style='color: green;'>```LinearMipMapLinear = 9987```</span>
  LinearMipMapLinear = 9987,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture::Status```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Status {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Loading = 1```</span>
  Loading = 1,
  /// C++ enum variant: <span style='color: green;'>```Ready = 2```</span>
  Ready = 2,
  /// C++ enum variant: <span style='color: green;'>```Error = 3```</span>
  Error = 3,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture::Target```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Target {
  /// C++ enum variant: <span style='color: green;'>```TargetAutomatic = 0```</span>
  TargetAutomatic = 0,
  /// C++ enum variant: <span style='color: green;'>```Target1D = 3552```</span>
  Target1D = 3552,
  /// C++ enum variant: <span style='color: green;'>```Target2D = 3553```</span>
  Target2D = 3553,
  /// C++ enum variant: <span style='color: green;'>```Target3D = 32879```</span>
  Target3D = 32879,
  /// C++ enum variant: <span style='color: green;'>```TargetRectangle = 34037```</span>
  TargetRectangle = 34037,
  /// C++ enum variant: <span style='color: green;'>```TargetCubeMap = 34067```</span>
  TargetCubeMap = 34067,
  /// C++ enum variant: <span style='color: green;'>```Target1DArray = 35864```</span>
  Target1DArray = 35864,
  /// C++ enum variant: <span style='color: green;'>```Target2DArray = 35866```</span>
  Target2DArray = 35866,
  /// C++ enum variant: <span style='color: green;'>```TargetBuffer = 35882```</span>
  TargetBuffer = 35882,
  /// C++ enum variant: <span style='color: green;'>```TargetCubeMapArray = 36873```</span>
  TargetCubeMapArray = 36873,
  /// C++ enum variant: <span style='color: green;'>```Target2DMultisample = 37120```</span>
  Target2DMultisample = 37120,
  /// C++ enum variant: <span style='color: green;'>```Target2DMultisampleArray = 37122```</span>
  Target2DMultisampleArray = 37122,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTexture::TextureFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextureFormat {
  /// C++ enum variant: <span style='color: green;'>```NoFormat = 0```</span>
  NoFormat = 0,
  /// C++ enum variant: <span style='color: green;'>```Automatic = 1```</span>
  Automatic = 1,
  /// C++ enum variant: <span style='color: green;'>```DepthFormat = 6402```</span>
  DepthFormat = 6402,
  /// C++ enum variant: <span style='color: green;'>```AlphaFormat = 6406```</span>
  AlphaFormat = 6406,
  /// C++ enum variant: <span style='color: green;'>```RGBFormat = 6407```</span>
  RGBFormat = 6407,
  /// C++ enum variant: <span style='color: green;'>```RGBAFormat = 6408```</span>
  RGBAFormat = 6408,
  /// C++ enum variant: <span style='color: green;'>```LuminanceFormat = 6409```</span>
  LuminanceFormat = 6409,
  /// C++ enum variant: <span style='color: green;'>```LuminanceAlphaFormat = 6410```</span>
  LuminanceAlphaFormat = 6410,
  /// C++ enum variant: <span style='color: green;'>```RG3B2 = 10768```</span>
  RG3B2 = 10768,
  /// C++ enum variant: <span style='color: green;'>```RGB8_UNorm = 32849```</span>
  RGB8UNorm = 32849,
  /// C++ enum variant: <span style='color: green;'>```RGB16_UNorm = 32852```</span>
  RGB16UNorm = 32852,
  /// C++ enum variant: <span style='color: green;'>```RGBA4 = 32854```</span>
  RGBA4 = 32854,
  /// C++ enum variant: <span style='color: green;'>```RGB5A1 = 32855```</span>
  RGB5A1 = 32855,
  /// C++ enum variant: <span style='color: green;'>```RGBA8_UNorm = 32856```</span>
  RGBA8UNorm = 32856,
  /// C++ enum variant: <span style='color: green;'>```RGBA16_UNorm = 32859```</span>
  RGBA16UNorm = 32859,
  /// C++ enum variant: <span style='color: green;'>```D16 = 33189```</span>
  D16 = 33189,
  /// C++ enum variant: <span style='color: green;'>```D24 = 33190```</span>
  D24 = 33190,
  /// C++ enum variant: <span style='color: green;'>```D32 = 33191```</span>
  D32 = 33191,
  /// C++ enum variant: <span style='color: green;'>```R8_UNorm = 33321```</span>
  R8UNorm = 33321,
  /// C++ enum variant: <span style='color: green;'>```R16_UNorm = 33322```</span>
  R16UNorm = 33322,
  /// C++ enum variant: <span style='color: green;'>```RG8_UNorm = 33323```</span>
  RG8UNorm = 33323,
  /// C++ enum variant: <span style='color: green;'>```RG16_UNorm = 33324```</span>
  RG16UNorm = 33324,
  /// C++ enum variant: <span style='color: green;'>```R16F = 33325```</span>
  R16F = 33325,
  /// C++ enum variant: <span style='color: green;'>```R32F = 33326```</span>
  R32F = 33326,
  /// C++ enum variant: <span style='color: green;'>```RG16F = 33327```</span>
  RG16F = 33327,
  /// C++ enum variant: <span style='color: green;'>```RG32F = 33328```</span>
  RG32F = 33328,
  /// C++ enum variant: <span style='color: green;'>```R8I = 33329```</span>
  R8I = 33329,
  /// C++ enum variant: <span style='color: green;'>```R8U = 33330```</span>
  R8U = 33330,
  /// C++ enum variant: <span style='color: green;'>```R16I = 33331```</span>
  R16I = 33331,
  /// C++ enum variant: <span style='color: green;'>```R16U = 33332```</span>
  R16U = 33332,
  /// C++ enum variant: <span style='color: green;'>```R32I = 33333```</span>
  R32I = 33333,
  /// C++ enum variant: <span style='color: green;'>```R32U = 33334```</span>
  R32U = 33334,
  /// C++ enum variant: <span style='color: green;'>```RG8I = 33335```</span>
  RG8I = 33335,
  /// C++ enum variant: <span style='color: green;'>```RG8U = 33336```</span>
  RG8U = 33336,
  /// C++ enum variant: <span style='color: green;'>```RG16I = 33337```</span>
  RG16I = 33337,
  /// C++ enum variant: <span style='color: green;'>```RG16U = 33338```</span>
  RG16U = 33338,
  /// C++ enum variant: <span style='color: green;'>```RG32I = 33339```</span>
  RG32I = 33339,
  /// C++ enum variant: <span style='color: green;'>```RG32U = 33340```</span>
  RG32U = 33340,
  /// C++ enum variant: <span style='color: green;'>```RGB_DXT1 = 33776```</span>
  RGBDXT1 = 33776,
  /// C++ enum variant: <span style='color: green;'>```RGBA_DXT1 = 33777```</span>
  RGBADXT1 = 33777,
  /// C++ enum variant: <span style='color: green;'>```RGBA_DXT3 = 33778```</span>
  RGBADXT3 = 33778,
  /// C++ enum variant: <span style='color: green;'>```RGBA_DXT5 = 33779```</span>
  RGBADXT5 = 33779,
  /// C++ enum variant: <span style='color: green;'>```RGBA32F = 34836```</span>
  RGBA32F = 34836,
  /// C++ enum variant: <span style='color: green;'>```RGB32F = 34837```</span>
  RGB32F = 34837,
  /// C++ enum variant: <span style='color: green;'>```RGBA16F = 34842```</span>
  RGBA16F = 34842,
  /// C++ enum variant: <span style='color: green;'>```RGB16F = 34843```</span>
  RGB16F = 34843,
  /// C++ enum variant: <span style='color: green;'>```D24S8 = 35056```</span>
  D24S8 = 35056,
  /// C++ enum variant: <span style='color: green;'>```RG11B10F = 35898```</span>
  RG11B10F = 35898,
  /// C++ enum variant: <span style='color: green;'>```RGB9E5 = 35901```</span>
  RGB9E5 = 35901,
  /// C++ enum variant: <span style='color: green;'>```SRGB8 = 35905```</span>
  SRGB8 = 35905,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8 = 35907```</span>
  SRGB8Alpha8 = 35907,
  /// C++ enum variant: <span style='color: green;'>```SRGB_DXT1 = 35916```</span>
  SRGBDXT1 = 35916,
  /// C++ enum variant: <span style='color: green;'>```SRGB_Alpha_DXT1 = 35917```</span>
  SRGBAlphaDXT1 = 35917,
  /// C++ enum variant: <span style='color: green;'>```SRGB_Alpha_DXT3 = 35918```</span>
  SRGBAlphaDXT3 = 35918,
  /// C++ enum variant: <span style='color: green;'>```SRGB_Alpha_DXT5 = 35919```</span>
  SRGBAlphaDXT5 = 35919,
  /// C++ enum variant: <span style='color: green;'>```D32F = 36012```</span>
  D32F = 36012,
  /// C++ enum variant: <span style='color: green;'>```D32FS8X24 = 36013```</span>
  D32FS8X24 = 36013,
  /// C++ enum variant: <span style='color: green;'>```R5G6B5 = 36194```</span>
  R5G6B5 = 36194,
  /// C++ enum variant: <span style='color: green;'>```RGB8_ETC1 = 36196```</span>
  RGB8ETC1 = 36196,
  /// C++ enum variant: <span style='color: green;'>```RGBA32U = 36208```</span>
  RGBA32U = 36208,
  /// C++ enum variant: <span style='color: green;'>```RGB32U = 36209```</span>
  RGB32U = 36209,
  /// C++ enum variant: <span style='color: green;'>```RGBA16U = 36214```</span>
  RGBA16U = 36214,
  /// C++ enum variant: <span style='color: green;'>```RGB16U = 36215```</span>
  RGB16U = 36215,
  /// C++ enum variant: <span style='color: green;'>```RGBA8U = 36220```</span>
  RGBA8U = 36220,
  /// C++ enum variant: <span style='color: green;'>```RGB8U = 36221```</span>
  RGB8U = 36221,
  /// C++ enum variant: <span style='color: green;'>```RGBA32I = 36226```</span>
  RGBA32I = 36226,
  /// C++ enum variant: <span style='color: green;'>```RGB32I = 36227```</span>
  RGB32I = 36227,
  /// C++ enum variant: <span style='color: green;'>```RGBA16I = 36232```</span>
  RGBA16I = 36232,
  /// C++ enum variant: <span style='color: green;'>```RGB16I = 36233```</span>
  RGB16I = 36233,
  /// C++ enum variant: <span style='color: green;'>```RGBA8I = 36238```</span>
  RGBA8I = 36238,
  /// C++ enum variant: <span style='color: green;'>```RGB8I = 36239```</span>
  RGB8I = 36239,
  /// C++ enum variant: <span style='color: green;'>```R_ATI1N_UNorm = 36283```</span>
  RATI1NUNorm = 36283,
  /// C++ enum variant: <span style='color: green;'>```R_ATI1N_SNorm = 36284```</span>
  RATI1NSNorm = 36284,
  /// C++ enum variant: <span style='color: green;'>```RG_ATI2N_UNorm = 36285```</span>
  RGATI2NUNorm = 36285,
  /// C++ enum variant: <span style='color: green;'>```RG_ATI2N_SNorm = 36286```</span>
  RGATI2NSNorm = 36286,
  /// C++ enum variant: <span style='color: green;'>```RGB_BP_UNorm = 36492```</span>
  RGBBPUNorm = 36492,
  /// C++ enum variant: <span style='color: green;'>```SRGB_BP_UNorm = 36493```</span>
  SRGBBPUNorm = 36493,
  /// C++ enum variant: <span style='color: green;'>```RGB_BP_SIGNED_FLOAT = 36494```</span>
  RGBBPSIGNEDFLOAT = 36494,
  /// C++ enum variant: <span style='color: green;'>```RGB_BP_UNSIGNED_FLOAT = 36495```</span>
  RGBBPUNSIGNEDFLOAT = 36495,
  /// C++ enum variant: <span style='color: green;'>```R8_SNorm = 36756```</span>
  R8SNorm = 36756,
  /// C++ enum variant: <span style='color: green;'>```RG8_SNorm = 36757```</span>
  RG8SNorm = 36757,
  /// C++ enum variant: <span style='color: green;'>```RGB8_SNorm = 36758```</span>
  RGB8SNorm = 36758,
  /// C++ enum variant: <span style='color: green;'>```RGBA8_SNorm = 36759```</span>
  RGBA8SNorm = 36759,
  /// C++ enum variant: <span style='color: green;'>```R16_SNorm = 36760```</span>
  R16SNorm = 36760,
  /// C++ enum variant: <span style='color: green;'>```RG16_SNorm = 36761```</span>
  RG16SNorm = 36761,
  /// C++ enum variant: <span style='color: green;'>```RGB16_SNorm = 36762```</span>
  RGB16SNorm = 36762,
  /// C++ enum variant: <span style='color: green;'>```RGBA16_SNorm = 36763```</span>
  RGBA16SNorm = 36763,
  /// C++ enum variant: <span style='color: green;'>```RGB10A2 = 36975```</span>
  RGB10A2 = 36975,
  /// C++ enum variant: <span style='color: green;'>```R11_EAC_UNorm = 37488```</span>
  R11EACUNorm = 37488,
  /// C++ enum variant: <span style='color: green;'>```R11_EAC_SNorm = 37489```</span>
  R11EACSNorm = 37489,
  /// C++ enum variant: <span style='color: green;'>```RG11_EAC_UNorm = 37490```</span>
  RG11EACUNorm = 37490,
  /// C++ enum variant: <span style='color: green;'>```RG11_EAC_SNorm = 37491```</span>
  RG11EACSNorm = 37491,
  /// C++ enum variant: <span style='color: green;'>```RGB8_ETC2 = 37492```</span>
  RGB8ETC2 = 37492,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_ETC2 = 37493```</span>
  SRGB8ETC2 = 37493,
  /// C++ enum variant: <span style='color: green;'>```RGB8_PunchThrough_Alpha1_ETC2 = 37494```</span>
  RGB8PunchThroughAlpha1ETC2 = 37494,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_PunchThrough_Alpha1_ETC2 = 37495```</span>
  SRGB8PunchThroughAlpha1ETC2 = 37495,
  /// C++ enum variant: <span style='color: green;'>```RGBA8_ETC2_EAC = 37496```</span>
  RGBA8ETC2EAC = 37496,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ETC2_EAC = 37497```</span>
  SRGB8Alpha8ETC2EAC = 37497,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::abstract_texture::AbstractTexture {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_texture::AbstractTexture as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_texture::AbstractTexture {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_QObject_ptr(self as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_QObject_ptr(self as *const ::abstract_texture::AbstractTexture as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_texture::AbstractTexture> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_texture::AbstractTexture {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_texture::AbstractTexture {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_texture::AbstractTexture> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_texture::AbstractTexture {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_texture::AbstractTexture {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DRender_QAbstractTexture_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_texture::AbstractTexture {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_texture::AbstractTexture as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_texture::AbstractTexture {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTexture_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_texture::AbstractTexture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractTexture::set_size](../struct.AbstractTexture.html#method.set_size) method.
  pub trait AbstractTextureSetSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_texture::AbstractTexture) -> ();
  }
  impl<'largs> AbstractTextureSetSizeArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_texture::AbstractTexture) -> () {
      let width = self;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width(original_self as *mut ::abstract_texture::AbstractTexture, width) }
    }
  }
  impl<'largs> AbstractTextureSetSizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_texture::AbstractTexture) -> () {
      let width = self.0;
      let height = self.1;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width_height(original_self as *mut ::abstract_texture::AbstractTexture, width, height) }
    }
  }
  impl<'largs> AbstractTextureSetSizeArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_texture::AbstractTexture) -> () {
      let width = self.0;
      let height = self.1;
      let depth = self.2;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTexture_setSize_width_height_depth(original_self as *mut ::abstract_texture::AbstractTexture, width, height, depth) }
    }
  }
}

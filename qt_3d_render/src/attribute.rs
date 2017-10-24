/// C++ type: <span style='color: green;'>```Qt3DRender::QAttribute```</span>
#[repr(C)]
pub struct Attribute(u8);

impl Attribute {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute::AttributeType Qt3DRender::QAttribute::attributeType() const```</span>
  ///
  ///
  pub fn attribute_type(&self) -> ::attribute::AttributeType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_attributeType(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBuffer* Qt3DRender::QAttribute::buffer() const```</span>
  ///
  ///
  pub fn buffer(&self) -> *mut ::buffer::Buffer {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_buffer(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QAttribute::byteOffset() const```</span>
  ///
  ///
  pub fn byte_offset(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_byteOffset(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QAttribute::byteStride() const```</span>
  ///
  ///
  pub fn byte_stride(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_byteStride(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QAttribute::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_count(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAttribute::defaultColorAttributeName()```</span>
  ///
  ///
  pub fn default_color_attribute_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_defaultColorAttributeName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAttribute::defaultNormalAttributeName()```</span>
  ///
  ///
  pub fn default_normal_attribute_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_defaultNormalAttributeName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAttribute::defaultPositionAttributeName()```</span>
  ///
  ///
  pub fn default_position_attribute_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_defaultPositionAttributeName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAttribute::defaultTangentAttributeName()```</span>
  ///
  ///
  pub fn default_tangent_attribute_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_defaultTangentAttributeName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAttribute::defaultTextureCoordinateAttributeName()```</span>
  ///
  ///
  pub fn default_texture_coordinate_attribute_name() -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_defaultTextureCoordinateAttributeName_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QAttribute::divisor() const```</span>
  ///
  ///
  pub fn divisor(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_divisor(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QAttribute::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_metaObject(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DRender::QAttribute::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_name_to_output(self as *const ::attribute::Attribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::attribute::Attribute> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute::QAttribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset = ?, unsigned int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, *mut ::qt_3d_core::node::Node)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset = ?, unsigned int stride = ?, Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, &::qt_core::string::String, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, const QString& name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, &::qt_core::string::String, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, const QString& name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, &::qt_core::string::String, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, const QString& name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset = ?, unsigned int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::buffer::Buffer, &::qt_core::string::String, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint, *mut ::qt_3d_core::node::Node)) -> ::cpp_utils::CppBox<::attribute::Attribute>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QAttribute::QAttribute(Qt3DRender::QBuffer* buf, const QString& name, Qt3DRender::QAttribute::VertexBaseType vertexBaseType, unsigned int vertexSize, unsigned int count, unsigned int offset = ?, unsigned int stride = ?, Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::attribute::Attribute>
    where Args: overloading::AttributeNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QAttribute::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_qt_metacall(self as *mut ::attribute::Attribute,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QAttribute::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_qt_metacast(self as *mut ::attribute::Attribute, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setAttributeType(Qt3DRender::QAttribute::AttributeType attributeType)```</span>
  ///
  ///
  pub fn set_attribute_type(&mut self, attribute_type: ::attribute::AttributeType) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setAttributeType(self as *mut ::attribute::Attribute, attribute_type)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setBuffer(Qt3DRender::QBuffer* buffer)```</span>
  ///
  ///
  pub unsafe fn set_buffer(&mut self, buffer: *mut ::buffer::Buffer) {
    ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setBuffer(self as *mut ::attribute::Attribute, buffer)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setByteOffset(unsigned int byteOffset)```</span>
  ///
  ///
  pub fn set_byte_offset(&mut self, byte_offset: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setByteOffset(self as *mut ::attribute::Attribute, byte_offset)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setByteStride(unsigned int byteStride)```</span>
  ///
  ///
  pub fn set_byte_stride(&mut self, byte_stride: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setByteStride(self as *mut ::attribute::Attribute, byte_stride)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setCount(unsigned int count)```</span>
  ///
  ///
  pub fn set_count(&mut self, count: ::libc::c_uint) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setCount(self as *mut ::attribute::Attribute, count) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setDataSize(unsigned int size)```</span>
  ///
  ///
  pub fn set_data_size(&mut self, size: ::libc::c_uint) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setDataSize(self as *mut ::attribute::Attribute, size) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setDataType(Qt3DRender::QAttribute::VertexBaseType type)```</span>
  ///
  ///
  pub fn set_data_type(&mut self, type_: ::attribute::VertexBaseType) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setDataType(self as *mut ::attribute::Attribute, type_) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setDivisor(unsigned int divisor)```</span>
  ///
  ///
  pub fn set_divisor(&mut self, divisor: ::libc::c_uint) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setDivisor(self as *mut ::attribute::Attribute, divisor) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setName(const QString& name)```</span>
  ///
  ///
  pub fn set_name(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setName(self as *mut ::attribute::Attribute,
                                                          name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setVertexBaseType(Qt3DRender::QAttribute::VertexBaseType type)```</span>
  ///
  ///
  pub fn set_vertex_base_type(&mut self, type_: ::attribute::VertexBaseType) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setVertexBaseType(self as *mut ::attribute::Attribute, type_) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAttribute::setVertexSize(unsigned int size)```</span>
  ///
  ///
  pub fn set_vertex_size(&mut self, size: ::libc::c_uint) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_setVertexSize(self as *mut ::attribute::Attribute, size) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAttribute::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAttribute::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute::VertexBaseType Qt3DRender::QAttribute::vertexBaseType() const```</span>
  ///
  ///
  pub fn vertex_base_type(&self) -> ::attribute::VertexBaseType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_vertexBaseType(self as *const ::attribute::Attribute) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int Qt3DRender::QAttribute::vertexSize() const```</span>
  ///
  ///
  pub fn vertex_size(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_vertexSize(self as *const ::attribute::Attribute) }
  }
}

impl ::cpp_utils::CppDeletable for ::attribute::Attribute {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Attribute`.
  pub struct Signals<'a>(&'a ::attribute::Attribute);
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::attribute::Attribute);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::vertexBaseTypeChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().vertex_base_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct VertexBaseTypeChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for VertexBaseTypeChanged<'a> {
    type Arguments = (::attribute::VertexBaseType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2vertexBaseTypeChanged(Qt3DRender::QAttribute::VertexBaseType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VertexBaseTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::byteStrideChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().byte_stride_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct ByteStrideChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for ByteStrideChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2byteStrideChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ByteStrideChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::countChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct CountChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for CountChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2countChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CountChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::nodeDestroyed`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct NodeDestroyed<'a>(&'a ::attribute::Attribute);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::enabledChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct EnabledChanged<'a>(&'a ::attribute::Attribute);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::vertexSizeChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().vertex_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct VertexSizeChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for VertexSizeChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2vertexSizeChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VertexSizeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::nameChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct NameChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for NameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NameChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::dataSizeChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().data_size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct DataSizeChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for DataSizeChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataSizeChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DataSizeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::dataTypeChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().data_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct DataTypeChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for DataTypeChanged<'a> {
    type Arguments = (::attribute::VertexBaseType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataTypeChanged(Qt3DRender::QAttribute::VertexBaseType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DataTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::parentChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct ParentChanged<'a>(&'a ::attribute::Attribute);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::bufferChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().buffer_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct BufferChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for BufferChanged<'a> {
    type Arguments = (*mut ::buffer::Buffer,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bufferChanged(Qt3DRender::QBuffer*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BufferChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::divisorChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().divisor_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct DivisorChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for DivisorChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2divisorChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DivisorChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::byteOffsetChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().byte_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct ByteOffsetChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for ByteOffsetChanged<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2byteOffsetChanged(unsigned int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ByteOffsetChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAttribute::attributeTypeChanged`.
  ///
  /// An object of this type can be created from `Attribute` with `object.signals().attribute_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct AttributeTypeChanged<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for AttributeTypeChanged<'a> {
    type Arguments = (::attribute::AttributeType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2attributeTypeChanged(Qt3DRender::QAttribute::AttributeType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AttributeTypeChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::vertexBaseTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_base_type_changed(&self) -> VertexBaseTypeChanged {
      VertexBaseTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::byteStrideChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn byte_stride_changed(&self) -> ByteStrideChanged {
      ByteStrideChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::countChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn count_changed(&self) -> CountChanged {
      CountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::vertexSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_size_changed(&self) -> VertexSizeChanged {
      VertexSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::nameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn name_changed(&self) -> NameChanged {
      NameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::dataSizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_size_changed(&self) -> DataSizeChanged {
      DataSizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::dataTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_type_changed(&self) -> DataTypeChanged {
      DataTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::bufferChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn buffer_changed(&self) -> BufferChanged {
      BufferChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::divisorChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn divisor_changed(&self) -> DivisorChanged {
      DivisorChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::byteOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn byte_offset_changed(&self) -> ByteOffsetChanged {
      ByteOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAttribute::attributeTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn attribute_type_changed(&self) -> AttributeTypeChanged {
      AttributeTypeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Attribute`.
  pub struct Slots<'a>(&'a ::attribute::Attribute);
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setDataType`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_data_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetDataType<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetDataType<'a> {
    type Arguments = (::attribute::VertexBaseType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDataType(Qt3DRender::QAttribute::VertexBaseType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setByteStride`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_byte_stride()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetByteStride<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetByteStride<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setByteStride(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setName`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_name()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetName<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetName<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setName(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setDivisor`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_divisor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetDivisor<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetDivisor<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDivisor(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setByteOffset`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_byte_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetByteOffset<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetByteOffset<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setByteOffset(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setCount`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_count()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetCount<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetCount<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCount(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setVertexSize`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_vertex_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetVertexSize<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetVertexSize<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVertexSize(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setEnabled`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetEnabled<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setDataSize`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_data_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetDataSize<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetDataSize<'a> {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDataSize(unsigned int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setParent`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetParent<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setVertexBaseType`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_vertex_base_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetVertexBaseType<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetVertexBaseType<'a> {
    type Arguments = (::attribute::VertexBaseType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVertexBaseType(Qt3DRender::QAttribute::VertexBaseType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setBuffer`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_buffer()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetBuffer<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetBuffer<'a> {
    type Arguments = (*mut ::buffer::Buffer,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBuffer(Qt3DRender::QBuffer*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAttribute::setAttributeType`.
  ///
  /// An object of this type can be created from `Attribute` with `object.slots().set_attribute_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Attribute` object.
  pub struct SetAttributeType<'a>(&'a ::attribute::Attribute);
  impl<'a> ::qt_core::connection::Receiver for SetAttributeType<'a> {
    type Arguments = (::attribute::AttributeType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAttributeType(Qt3DRender::QAttribute::AttributeType)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setDataType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_data_type(&self) -> SetDataType {
      SetDataType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setByteStride`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_byte_stride(&self) -> SetByteStride {
      SetByteStride(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setName`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_name(&self) -> SetName {
      SetName(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setDivisor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_divisor(&self) -> SetDivisor {
      SetDivisor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setByteOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_byte_offset(&self) -> SetByteOffset {
      SetByteOffset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setCount`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_count(&self) -> SetCount {
      SetCount(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setVertexSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertex_size(&self) -> SetVertexSize {
      SetVertexSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setDataSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_data_size(&self) -> SetDataSize {
      SetDataSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setVertexBaseType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertex_base_type(&self) -> SetVertexBaseType {
      SetVertexBaseType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setBuffer`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_buffer(&self) -> SetBuffer {
      SetBuffer(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAttribute::setAttributeType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_attribute_type(&self) -> SetAttributeType {
      SetAttributeType(self.0)
    }
  }
  impl ::attribute::Attribute {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QAttribute::AttributeType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AttributeType {
  /// C++ enum variant: <span style='color: green;'>```VertexAttribute = 0```</span>
  Vertex = 0,
  /// C++ enum variant: <span style='color: green;'>```IndexAttribute = 1```</span>
  Index = 1,
  /// C++ enum variant: <span style='color: green;'>```DrawIndirectAttribute = 2```</span>
  DrawIndirect = 2,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QAttribute::VertexBaseType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum VertexBaseType {
  /// C++ enum variant: <span style='color: green;'>```Byte = 0```</span>
  Byte = 0,
  /// C++ enum variant: <span style='color: green;'>```UnsignedByte = 1```</span>
  UnsignedByte = 1,
  /// C++ enum variant: <span style='color: green;'>```Short = 2```</span>
  Short = 2,
  /// C++ enum variant: <span style='color: green;'>```UnsignedShort = 3```</span>
  UnsignedShort = 3,
  /// C++ enum variant: <span style='color: green;'>```Int = 4```</span>
  Int = 4,
  /// C++ enum variant: <span style='color: green;'>```UnsignedInt = 5```</span>
  UnsignedInt = 5,
  /// C++ enum variant: <span style='color: green;'>```HalfFloat = 6```</span>
  HalfFloat = 6,
  /// C++ enum variant: <span style='color: green;'>```Float = 7```</span>
  Float = 7,
  /// C++ enum variant: <span style='color: green;'>```Double = 8```</span>
  Double = 8,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::attribute::Attribute {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::attribute::Attribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::attribute::Attribute as *mut ::attribute::Attribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::attribute::Attribute {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QAttribute_G_static_cast_QObject_ptr(self as *mut ::attribute::Attribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAttribute_G_static_cast_QObject_ptr(self as *const ::attribute::Attribute as *mut ::attribute::Attribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::attribute::Attribute> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::attribute::Attribute {
    let ffi_result = ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::attribute::Attribute {
    let ffi_result = ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::attribute::Attribute> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::attribute::Attribute {
    let ffi_result = ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::attribute::Attribute {
    let ffi_result = ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DRender_QAttribute_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::attribute::Attribute {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::attribute::Attribute as *mut ::attribute::Attribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::attribute::Attribute {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QAttribute_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::attribute::Attribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Attribute::new_unsafe](../struct.Attribute.html#method.new_unsafe) method.
  pub trait AttributeNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute>;
  }
  impl<'a> AttributeNewUnsafeArgs
    for (*mut ::buffer::Buffer,
                                           &'a ::qt_core::string::String,
                                           ::attribute::VertexBaseType,
                                           ::libc::c_uint,
                                           ::libc::c_uint) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let name = self.1;
      let vertex_base_type = self.2;
      let vertex_size = self.3;
      let count = self.4;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count(buf, name as *const ::qt_core::string::String, vertex_base_type, vertex_size, count);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AttributeNewUnsafeArgs
    for (*mut ::buffer::Buffer,
                                           &'a ::qt_core::string::String,
                                           ::attribute::VertexBaseType,
                                           ::libc::c_uint,
                                           ::libc::c_uint,
                                           ::libc::c_uint) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let name = self.1;
      let vertex_base_type = self.2;
      let vertex_size = self.3;
      let count = self.4;
      let offset = self.5;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset(buf, name as *const ::qt_core::string::String, vertex_base_type, vertex_size, count, offset);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AttributeNewUnsafeArgs
    for (*mut ::buffer::Buffer,
                                           &'a ::qt_core::string::String,
                                           ::attribute::VertexBaseType,
                                           ::libc::c_uint,
                                           ::libc::c_uint,
                                           ::libc::c_uint,
                                           ::libc::c_uint) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let name = self.1;
      let vertex_base_type = self.2;
      let vertex_size = self.3;
      let count = self.4;
      let offset = self.5;
      let stride = self.6;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset_stride(buf, name as *const ::qt_core::string::String, vertex_base_type, vertex_size, count, offset, stride);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AttributeNewUnsafeArgs
    for (*mut ::buffer::Buffer,
                                           &'a ::qt_core::string::String,
                                           ::attribute::VertexBaseType,
                                           ::libc::c_uint,
                                           ::libc::c_uint,
                                           ::libc::c_uint,
                                           ::libc::c_uint,
                                           *mut ::qt_3d_core::node::Node) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let name = self.1;
      let vertex_base_type = self.2;
      let vertex_size = self.3;
      let count = self.4;
      let offset = self.5;
      let stride = self.6;
      let parent = self.7;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_name_vertexBaseType_vertexSize_count_offset_stride_parent(buf, name as *const ::qt_core::string::String, vertex_base_type, vertex_size, count, offset, stride, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AttributeNewUnsafeArgs for (*mut ::buffer::Buffer, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let vertex_base_type = self.1;
      let vertex_size = self.2;
      let count = self.3;
      let ffi_result =
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count(buf,
                                                                                            vertex_base_type,
                                                                                            vertex_size,
                                                                                            count);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AttributeNewUnsafeArgs
    for (*mut ::buffer::Buffer, ::attribute::VertexBaseType, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let vertex_base_type = self.1;
      let vertex_size = self.2;
      let count = self.3;
      let offset = self.4;
      let ffi_result =
        ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset(buf,
                                                                                                   vertex_base_type,
                                                                                                   vertex_size,
                                                                                                   count,
                                                                                                   offset);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AttributeNewUnsafeArgs
    for (*mut ::buffer::Buffer,
                                       ::attribute::VertexBaseType,
                                       ::libc::c_uint,
                                       ::libc::c_uint,
                                       ::libc::c_uint,
                                       ::libc::c_uint) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let vertex_base_type = self.1;
      let vertex_size = self.2;
      let count = self.3;
      let offset = self.4;
      let stride = self.5;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset_stride(buf, vertex_base_type, vertex_size, count, offset, stride);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AttributeNewUnsafeArgs
    for (*mut ::buffer::Buffer,
                                       ::attribute::VertexBaseType,
                                       ::libc::c_uint,
                                       ::libc::c_uint,
                                       ::libc::c_uint,
                                       ::libc::c_uint,
                                       *mut ::qt_3d_core::node::Node) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let buf = self.0;
      let vertex_base_type = self.1;
      let vertex_size = self.2;
      let count = self.3;
      let offset = self.4;
      let stride = self.5;
      let parent = self.6;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_buf_vertexBaseType_vertexSize_count_offset_stride_parent(buf, vertex_base_type, vertex_size, count, offset, stride, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AttributeNewUnsafeArgs for *mut ::qt_3d_core::node::Node {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::attribute::Attribute> {
      let parent = self;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QAttribute_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

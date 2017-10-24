/// C++ type: <span style='color: green;'>```Qt3DRender::QBuffer::AccessType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AccessType {
  /// C++ enum variant: <span style='color: green;'>```Write = 1```</span>
  Write = 1,
  /// C++ enum variant: <span style='color: green;'>```Read = 2```</span>
  Read = 2,
  /// C++ enum variant: <span style='color: green;'>```ReadWrite = 3```</span>
  ReadWrite = 3,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QBuffer```</span>
#[repr(C)]
pub struct Buffer(u8);

impl Buffer {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QBuffer::AccessType Qt3DRender::QBuffer::accessType() const```</span>
  ///
  ///
  pub fn access_type(&self) -> ::buffer::AccessType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_accessType(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QBuffer::data() const```</span>
  ///
  ///
  pub fn data(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_data_to_output(self as *const ::buffer::Buffer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QBufferDataGenerator> Qt3DRender::QBuffer::dataGenerator() const```</span>
  ///
  ///
  pub fn data_generator(&self) -> ::shared_pointer::SharedPointerBufferDataGenerator {
    {
      let mut object: ::shared_pointer::SharedPointerBufferDataGenerator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_dataGenerator_to_output(self as *const ::buffer::Buffer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QBuffer::isSyncData() const```</span>
  ///
  ///
  pub fn is_sync_data(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_isSyncData(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QBuffer::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_metaObject(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBuffer::QBuffer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::buffer::Buffer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBuffer::QBuffer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::buffer::BufferType) -> ::cpp_utils::CppBox<::buffer::Buffer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBuffer::QBuffer(Qt3DRender::QBuffer::BufferType ty = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::buffer::Buffer>
    where Args: overloading::BufferNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBuffer::QBuffer(Qt3DRender::QBuffer::BufferType ty = ?, Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(ty: ::buffer::BufferType,
                           parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::buffer::Buffer> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_new_ty_parent(ty, parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QBuffer::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_qt_metacall(self as *mut ::buffer::Buffer,
                                                         arg1 as *const ::qt_core::meta_object::Call,
                                                         arg2,
                                                         arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QBuffer::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_qt_metacast(self as *mut ::buffer::Buffer, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBuffer::setAccessType(Qt3DRender::QBuffer::AccessType access)```</span>
  ///
  ///
  pub fn set_access_type(&mut self, access: ::buffer::AccessType) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_setAccessType(self as *mut ::buffer::Buffer, access) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QBuffer::setData(const QByteArray& bytes)```</span>
  ///
  ///
  pub fn set_data(&mut self, bytes: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_setData(self as *mut ::buffer::Buffer,
                                                       bytes as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QBuffer::setDataGenerator(const QSharedPointer<Qt3DRender::QBufferDataGenerator>& functor)```</span>
  ///
  ///
  pub fn set_data_generator(&mut self, functor: &::shared_pointer::SharedPointerBufferDataGenerator) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_setDataGenerator(self as *mut ::buffer::Buffer, functor as *const ::shared_pointer::SharedPointerBufferDataGenerator) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBuffer::setSyncData(bool syncData)```</span>
  ///
  ///
  pub fn set_sync_data(&mut self, sync_data: bool) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_setSyncData(self as *mut ::buffer::Buffer, sync_data) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBuffer::setType(Qt3DRender::QBuffer::BufferType type)```</span>
  ///
  ///
  pub fn set_type(&mut self, type_: ::buffer::BufferType) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_setType(self as *mut ::buffer::Buffer, type_) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QBuffer::setUsage(Qt3DRender::QBuffer::UsageType usage)```</span>
  ///
  ///
  pub fn set_usage(&mut self, usage: ::buffer::UsageType) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_setUsage(self as *mut ::buffer::Buffer, usage) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBuffer::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBuffer::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBuffer::BufferType Qt3DRender::QBuffer::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::buffer::BufferType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_type(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QBuffer::updateData(int offset, const QByteArray& bytes)```</span>
  ///
  ///
  pub fn update_data(&mut self, offset: ::libc::c_int, bytes: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_updateData(self as *mut ::buffer::Buffer,
                                                          offset,
                                                          bytes as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QBuffer::UsageType Qt3DRender::QBuffer::usage() const```</span>
  ///
  ///
  pub fn usage(&self) -> ::buffer::UsageType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_usage(self as *const ::buffer::Buffer) }
  }
}

impl ::cpp_utils::CppDeletable for ::buffer::Buffer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Buffer`.
  pub struct Signals<'a>(&'a ::buffer::Buffer);
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::nodeDestroyed`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct NodeDestroyed<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::enabledChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct EnabledChanged<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::dataAvailable`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().data_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct DataAvailable<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for DataAvailable<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataAvailable()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DataAvailable<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::syncDataChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().sync_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SyncDataChanged<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SyncDataChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2syncDataChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SyncDataChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::typeChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct TypeChanged<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for TypeChanged<'a> {
    type Arguments = (::buffer::BufferType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2typeChanged(Qt3DRender::QBuffer::BufferType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::accessTypeChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().access_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct AccessTypeChanged<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for AccessTypeChanged<'a> {
    type Arguments = (::buffer::AccessType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2accessTypeChanged(Qt3DRender::QBuffer::AccessType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AccessTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::parentChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct ParentChanged<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::usageChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().usage_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct UsageChanged<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for UsageChanged<'a> {
    type Arguments = (::buffer::UsageType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2usageChanged(Qt3DRender::QBuffer::UsageType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UsageChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QBuffer::dataChanged`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct DataChanged<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for DataChanged<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataChanged(const QByteArray&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DataChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::dataAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_available(&self) -> DataAvailable {
      DataAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::syncDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sync_data_changed(&self) -> SyncDataChanged {
      SyncDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::typeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn type_changed(&self) -> TypeChanged {
      TypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::accessTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn access_type_changed(&self) -> AccessTypeChanged {
      AccessTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::usageChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn usage_changed(&self) -> UsageChanged {
      UsageChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QBuffer::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Buffer`.
  pub struct Slots<'a>(&'a ::buffer::Buffer);
  /// Represents a built-in Qt slot `Qt3DRender::QBuffer::setParent`.
  ///
  /// An object of this type can be created from `Buffer` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SetParent<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBuffer::setType`.
  ///
  /// An object of this type can be created from `Buffer` with `object.slots().set_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SetType<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SetType<'a> {
    type Arguments = (::buffer::BufferType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setType(Qt3DRender::QBuffer::BufferType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBuffer::setAccessType`.
  ///
  /// An object of this type can be created from `Buffer` with `object.slots().set_access_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SetAccessType<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SetAccessType<'a> {
    type Arguments = (::buffer::AccessType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAccessType(Qt3DRender::QBuffer::AccessType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBuffer::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `Buffer` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBuffer::setUsage`.
  ///
  /// An object of this type can be created from `Buffer` with `object.slots().set_usage()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SetUsage<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SetUsage<'a> {
    type Arguments = (::buffer::UsageType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setUsage(Qt3DRender::QBuffer::UsageType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBuffer::setEnabled`.
  ///
  /// An object of this type can be created from `Buffer` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SetEnabled<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QBuffer::setSyncData`.
  ///
  /// An object of this type can be created from `Buffer` with `object.slots().set_sync_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct SetSyncData<'a>(&'a ::buffer::Buffer);
  impl<'a> ::qt_core::connection::Receiver for SetSyncData<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSyncData(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBuffer::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBuffer::setType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_type(&self) -> SetType {
      SetType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBuffer::setAccessType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_access_type(&self) -> SetAccessType {
      SetAccessType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBuffer::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBuffer::setUsage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_usage(&self) -> SetUsage {
      SetUsage(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBuffer::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QBuffer::setSyncData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_sync_data(&self) -> SetSyncData {
      SetSyncData(self.0)
    }
  }
  impl ::buffer::Buffer {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QBuffer::BufferType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BufferType {
  /// C++ enum variant: <span style='color: green;'>```VertexBuffer = 34962```</span>
  Vertex = 34962,
  /// C++ enum variant: <span style='color: green;'>```IndexBuffer = 34963```</span>
  Index = 34963,
  /// C++ enum variant: <span style='color: green;'>```PixelPackBuffer = 35051```</span>
  PixelPack = 35051,
  /// C++ enum variant: <span style='color: green;'>```PixelUnpackBuffer = 35052```</span>
  PixelUnpack = 35052,
  /// C++ enum variant: <span style='color: green;'>```UniformBuffer = 35345```</span>
  Uniform = 35345,
  /// C++ enum variant: <span style='color: green;'>```DrawIndirectBuffer = 36671```</span>
  DrawIndirect = 36671,
  /// C++ enum variant: <span style='color: green;'>```ShaderStorageBuffer = 37074```</span>
  ShaderStorage = 37074,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QBuffer::UsageType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UsageType {
  /// C++ enum variant: <span style='color: green;'>```StreamDraw = 35040```</span>
  StreamDraw = 35040,
  /// C++ enum variant: <span style='color: green;'>```StreamRead = 35041```</span>
  StreamRead = 35041,
  /// C++ enum variant: <span style='color: green;'>```StreamCopy = 35042```</span>
  StreamCopy = 35042,
  /// C++ enum variant: <span style='color: green;'>```StaticDraw = 35044```</span>
  StaticDraw = 35044,
  /// C++ enum variant: <span style='color: green;'>```StaticRead = 35045```</span>
  StaticRead = 35045,
  /// C++ enum variant: <span style='color: green;'>```StaticCopy = 35046```</span>
  StaticCopy = 35046,
  /// C++ enum variant: <span style='color: green;'>```DynamicDraw = 35048```</span>
  DynamicDraw = 35048,
  /// C++ enum variant: <span style='color: green;'>```DynamicRead = 35049```</span>
  DynamicRead = 35049,
  /// C++ enum variant: <span style='color: green;'>```DynamicCopy = 35050```</span>
  DynamicCopy = 35050,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::buffer::Buffer {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::buffer::Buffer as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::buffer::Buffer {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBuffer_G_static_cast_QObject_ptr(self as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBuffer_G_static_cast_QObject_ptr(self as *const ::buffer::Buffer as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer::Buffer> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::buffer::Buffer {
    let ffi_result = ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::buffer::Buffer {
    let ffi_result = ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer::Buffer> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::buffer::Buffer {
    let ffi_result = ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::buffer::Buffer {
    let ffi_result = ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DRender_QBuffer_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::buffer::Buffer {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::buffer::Buffer as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::buffer::Buffer {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QBuffer_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Buffer::new](../struct.Buffer.html#method.new) method.
  pub trait BufferNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::buffer::Buffer>;
  }
  impl BufferNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::buffer::Buffer> {

      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl BufferNewArgs for ::buffer::BufferType {
    fn exec(self) -> ::cpp_utils::CppBox<::buffer::Buffer> {
      let ty = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBuffer_new_ty(ty) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}

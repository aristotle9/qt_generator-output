/// C++ type: <span style='color: green;'>```QSharedMemory::AccessMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AccessMode {
  /// C++ enum variant: <span style='color: green;'>```ReadOnly = 0```</span>
  Only = 0,
  /// C++ enum variant: <span style='color: green;'>```ReadWrite = 1```</span>
  Write = 1,
}

/// C++ type: <span style='color: green;'>```QSharedMemory```</span>
#[repr(C)]
pub struct SharedMemory(u8);

impl SharedMemory {
  /// C++ method: <span style='color: green;'>```QSharedMemory::attach```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn attach(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSharedMemory::attach()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn attach(&mut self, ::shared_memory::AccessMode) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSharedMemory::attach(QSharedMemory::AccessMode mode = ?)```</span>
  ///
  ///
  pub fn attach<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SharedMemoryAttachArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const void* QSharedMemory::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QSharedMemory_constData(self as *const ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```QSharedMemory::create```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSharedMemory::create(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create(&mut self, (::libc::c_int, ::shared_memory::AccessMode)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSharedMemory::create(int size, QSharedMemory::AccessMode mode = ?)```</span>
  ///
  ///
  pub fn create<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SharedMemoryCreateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const void* QSharedMemory::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QSharedMemory_data_const(self as *const ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```void* QSharedMemory::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QSharedMemory_data(self as *mut ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedMemory::detach()```</span>
  ///
  ///
  pub fn detach(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QSharedMemory_detach(self as *mut ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```QSharedMemory::SharedMemoryError QSharedMemory::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::shared_memory::SharedMemoryError {
    unsafe { ::ffi::qt_core_c_QSharedMemory_error(self as *const ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```QString QSharedMemory::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSharedMemory_errorString_to_output(self as *const ::shared_memory::SharedMemory, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedMemory::isAttached() const```</span>
  ///
  ///
  pub fn is_attached(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSharedMemory_isAttached(self as *const ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```QString QSharedMemory::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSharedMemory_key_to_output(self as *const ::shared_memory::SharedMemory, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedMemory::lock()```</span>
  ///
  ///
  pub fn lock(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QSharedMemory_lock(self as *mut ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSharedMemory::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QSharedMemory_metaObject(self as *const ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```QString QSharedMemory::nativeKey() const```</span>
  ///
  ///
  pub fn native_key(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSharedMemory_nativeKey_to_output(self as *const ::shared_memory::SharedMemory, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedMemory::QSharedMemory```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedMemory::QSharedMemory()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedMemory::QSharedMemory(const QString& key)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>
    where Args: overloading::SharedMemoryNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSharedMemory::QSharedMemory```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedMemory::QSharedMemory(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSharedMemory::QSharedMemory(const QString& key, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>
    where Args: overloading::SharedMemoryNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QSharedMemory::setKey(const QString& key)```</span>
  ///
  ///
  pub fn set_key(&mut self, key: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QSharedMemory_setKey(self as *mut ::shared_memory::SharedMemory,
                                            key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSharedMemory::setNativeKey(const QString& key)```</span>
  ///
  ///
  pub fn set_native_key(&mut self, key: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QSharedMemory_setNativeKey(self as *mut ::shared_memory::SharedMemory,
                                                  key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```int QSharedMemory::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSharedMemory_size(self as *const ::shared_memory::SharedMemory) }
  }

  /// C++ method: <span style='color: green;'>```static QString QSharedMemory::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSharedMemory_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSharedMemory::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSharedMemory_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSharedMemory::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QSharedMemory_unlock(self as *mut ::shared_memory::SharedMemory) }
  }
}

impl ::cpp_utils::CppDeletable for ::shared_memory::SharedMemory {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSharedMemory_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SharedMemory`.
  pub struct Signals<'a>(&'a ::shared_memory::SharedMemory);
  /// Represents a built-in Qt signal `QSharedMemory::objectNameChanged`.
  ///
  /// An object of this type can be created from `SharedMemory` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SharedMemory` object.
  pub struct ObjectNameChanged<'a>(&'a ::shared_memory::SharedMemory);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSharedMemory::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::shared_memory::SharedMemory {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QSharedMemory::SharedMemoryError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SharedMemoryError {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  NoError = 0,
  /// C++ enum variant: <span style='color: green;'>```PermissionDenied = 1```</span>
  PermissionDenied = 1,
  /// C++ enum variant: <span style='color: green;'>```InvalidSize = 2```</span>
  InvalidSize = 2,
  /// C++ enum variant: <span style='color: green;'>```KeyError = 3```</span>
  KeyError = 3,
  /// C++ enum variant: <span style='color: green;'>```AlreadyExists = 4```</span>
  AlreadyExists = 4,
  /// C++ enum variant: <span style='color: green;'>```NotFound = 5```</span>
  NotFound = 5,
  /// C++ enum variant: <span style='color: green;'>```LockError = 6```</span>
  LockError = 6,
  /// C++ enum variant: <span style='color: green;'>```OutOfResources = 7```</span>
  OutOfResources = 7,
  /// C++ enum variant: <span style='color: green;'>```UnknownError = 8```</span>
  UnknownError = 8,
}

impl ::cpp_utils::DynamicCast<::shared_memory::SharedMemory> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::shared_memory::SharedMemory> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSharedMemory_G_dynamic_cast_QSharedMemory_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::shared_memory::SharedMemory> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSharedMemory_G_dynamic_cast_QSharedMemory_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::shared_memory::SharedMemory {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSharedMemory_G_static_cast_QObject_ptr(self as *mut ::shared_memory::SharedMemory) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSharedMemory_G_static_cast_QObject_ptr(self as *const ::shared_memory::SharedMemory as *mut ::shared_memory::SharedMemory) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shared_memory::SharedMemory> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shared_memory::SharedMemory {
    let ffi_result = ::ffi::qt_core_c_QSharedMemory_G_static_cast_QSharedMemory_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shared_memory::SharedMemory {
    let ffi_result = ::ffi::qt_core_c_QSharedMemory_G_static_cast_QSharedMemory_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::shared_memory::SharedMemory {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSharedMemory_G_static_cast_QObject_ptr(self as *const ::shared_memory::SharedMemory as *mut ::shared_memory::SharedMemory) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::shared_memory::SharedMemory {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSharedMemory_G_static_cast_QObject_ptr(self as *mut ::shared_memory::SharedMemory) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SharedMemory::attach](../struct.SharedMemory.html#method.attach) method.
  pub trait SharedMemoryAttachArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::shared_memory::SharedMemory) -> bool;
  }
  impl<'largs> SharedMemoryAttachArgs<'largs> for ::shared_memory::AccessMode {
    fn exec(self, original_self: &'largs mut ::shared_memory::SharedMemory) -> bool {
      let mode = self;
      unsafe { ::ffi::qt_core_c_QSharedMemory_attach_mode(original_self as *mut ::shared_memory::SharedMemory, mode) }
    }
  }
  impl<'largs> SharedMemoryAttachArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::shared_memory::SharedMemory) -> bool {

      unsafe { ::ffi::qt_core_c_QSharedMemory_attach_no_args(original_self as *mut ::shared_memory::SharedMemory) }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedMemory::create](../struct.SharedMemory.html#method.create) method.
  pub trait SharedMemoryCreateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::shared_memory::SharedMemory) -> bool;
  }
  impl<'largs> SharedMemoryCreateArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::shared_memory::SharedMemory) -> bool {
      let size = self;
      unsafe { ::ffi::qt_core_c_QSharedMemory_create_size(original_self as *mut ::shared_memory::SharedMemory, size) }
    }
  }
  impl<'largs> SharedMemoryCreateArgs<'largs> for (::libc::c_int, ::shared_memory::AccessMode) {
    fn exec(self, original_self: &'largs mut ::shared_memory::SharedMemory) -> bool {
      let size = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_core_c_QSharedMemory_create_size_mode(original_self as *mut ::shared_memory::SharedMemory,
                                                        size,
                                                        mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedMemory::new](../struct.SharedMemory.html#method.new) method.
  pub trait SharedMemoryNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>;
  }
  impl<'a> SharedMemoryNewArgs for &'a ::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory> {
      let key = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QSharedMemory_new_key(key as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl SharedMemoryNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QSharedMemory_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [SharedMemory::new_unsafe](../struct.SharedMemory.html#method.new_unsafe) method.
  pub trait SharedMemoryNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory>;
  }
  impl<'a> SharedMemoryNewUnsafeArgs for (&'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory> {
      let key = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QSharedMemory_new_key_parent(key as *const ::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SharedMemoryNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::shared_memory::SharedMemory> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QSharedMemory_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}

/// C++ type: <span style='color: green;'>```QSignalMapper```</span>
#[repr(C)]
pub struct SignalMapper(u8);

impl SignalMapper {
  /// C++ method: <span style='color: green;'>```[slot] void QSignalMapper::map()```</span>
  ///
  ///
  pub fn map(&mut self) {
    unsafe { ::ffi::qt_core_c_QSignalMapper_map_no_args(self as *mut ::signal_mapper::SignalMapper) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSignalMapper::map(QObject* sender)```</span>
  ///
  ///
  pub unsafe fn map_unsafe(&mut self, sender: *mut ::object::Object) {
    ::ffi::qt_core_c_QSignalMapper_map_sender(self as *mut ::signal_mapper::SignalMapper, sender)
  }

  /// C++ method: <span style='color: green;'>```QSignalMapper::mapping```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mapping(&self, &::string::String) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QSignalMapper::mapping(const QString& text) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mapping(&self, ::libc::c_int) -> *mut ::object::Object```<br>
  /// C++ method: <span style='color: green;'>```QObject* QSignalMapper::mapping(int id) const```</span>
  ///
  ///
  pub fn mapping<'largs, Args>(&'largs self, args: Args) -> *mut ::object::Object
    where Args: overloading::SignalMapperMappingArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QObject* QSignalMapper::mapping(QObject* object) const```</span>
  ///
  ///
  pub unsafe fn mapping_unsafe(&self, object: *mut ::object::Object) -> *mut ::object::Object {
    ::ffi::qt_core_c_QSignalMapper_mapping_object(self as *const ::signal_mapper::SignalMapper, object)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSignalMapper::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QSignalMapper_metaObject(self as *const ::signal_mapper::SignalMapper) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSignalMapper::QSignalMapper()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::signal_mapper::SignalMapper> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalMapper_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSignalMapper::QSignalMapper(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::signal_mapper::SignalMapper> {
    let ffi_result = ::ffi::qt_core_c_QSignalMapper_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QSignalMapper::removeMappings(QObject* sender)```</span>
  ///
  ///
  pub unsafe fn remove_mappings(&mut self, sender: *mut ::object::Object) {
    ::ffi::qt_core_c_QSignalMapper_removeMappings(self as *mut ::signal_mapper::SignalMapper, sender)
  }

  /// C++ method: <span style='color: green;'>```QSignalMapper::setMapping```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_mapping(&mut self, (*mut ::object::Object, *mut ::object::Object)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSignalMapper::setMapping(QObject* sender, QObject* object)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_mapping(&mut self, (*mut ::object::Object, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSignalMapper::setMapping(QObject* sender, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_mapping(&mut self, (*mut ::object::Object, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSignalMapper::setMapping(QObject* sender, int id)```</span>
  ///
  ///
  pub unsafe fn set_mapping<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SignalMapperSetMappingArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QSignalMapper::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSignalMapper_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSignalMapper::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSignalMapper_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::signal_mapper::SignalMapper {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSignalMapper_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SignalMapper`.
  pub struct Signals<'a>(&'a ::signal_mapper::SignalMapper);
  /// Represents a built-in Qt signal `QSignalMapper::mapped`.
  ///
  /// An object of this type can be created from `SignalMapper` with `object.signals().mapped_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalMapper` object.
  pub struct MappedCInt<'a>(&'a ::signal_mapper::SignalMapper);
  impl<'a> ::connection::Receiver for MappedCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mapped(int)\0"
    }
  }
  impl<'a> ::connection::Signal for MappedCInt<'a> {}
  /// Represents a built-in Qt signal `QSignalMapper::mapped`.
  ///
  /// An object of this type can be created from `SignalMapper` with `object.signals().mapped_string_ref()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalMapper` object.
  pub struct MappedStringRef<'a>(&'a ::signal_mapper::SignalMapper);
  impl<'a> ::connection::Receiver for MappedStringRef<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mapped(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for MappedStringRef<'a> {}
  /// Represents a built-in Qt signal `QSignalMapper::mapped`.
  ///
  /// An object of this type can be created from `SignalMapper` with `object.signals().mapped_object_mut_ptr()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalMapper` object.
  pub struct MappedObjectMutPtr<'a>(&'a ::signal_mapper::SignalMapper);
  impl<'a> ::connection::Receiver for MappedObjectMutPtr<'a> {
    type Arguments = (*mut ::object::Object,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mapped(QObject*)\0"
    }
  }
  impl<'a> ::connection::Signal for MappedObjectMutPtr<'a> {}
  /// Represents a built-in Qt signal `QSignalMapper::objectNameChanged`.
  ///
  /// An object of this type can be created from `SignalMapper` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalMapper` object.
  pub struct ObjectNameChanged<'a>(&'a ::signal_mapper::SignalMapper);
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
    /// Returns an object representing a built-in Qt signal `QSignalMapper::mapped`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mapped_c_int(&self) -> MappedCInt {
      MappedCInt(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSignalMapper::mapped`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mapped_string_ref(&self) -> MappedStringRef {
      MappedStringRef(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSignalMapper::mapped`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mapped_object_mut_ptr(&self) -> MappedObjectMutPtr {
      MappedObjectMutPtr(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSignalMapper::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SignalMapper`.
  pub struct Slots<'a>(&'a ::signal_mapper::SignalMapper);
  /// Represents a built-in Qt slot `QSignalMapper::map`.
  ///
  /// An object of this type can be created from `SignalMapper` with `object.slots().map()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalMapper` object.
  pub struct Map<'a>(&'a ::signal_mapper::SignalMapper);
  impl<'a> ::connection::Receiver for Map<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1map()\0"
    }
  }
  /// Represents a built-in Qt slot `QSignalMapper::map`.
  ///
  /// An object of this type can be created from `SignalMapper` with `object.slots().map_object_mut_ptr()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalMapper` object.
  pub struct MapObjectMutPtr<'a>(&'a ::signal_mapper::SignalMapper);
  impl<'a> ::connection::Receiver for MapObjectMutPtr<'a> {
    type Arguments = (*mut ::object::Object,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1map(QObject*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSignalMapper::map`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn map(&self) -> Map {
      Map(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSignalMapper::map`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn map_object_mut_ptr(&self) -> MapObjectMutPtr {
      MapObjectMutPtr(self.0)
    }
  }
  impl ::signal_mapper::SignalMapper {
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

impl ::cpp_utils::DynamicCast<::signal_mapper::SignalMapper> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::signal_mapper::SignalMapper> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSignalMapper_G_dynamic_cast_QSignalMapper_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::signal_mapper::SignalMapper> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalMapper_G_dynamic_cast_QSignalMapper_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::signal_mapper::SignalMapper {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSignalMapper_G_static_cast_QObject_ptr(self as *mut ::signal_mapper::SignalMapper) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalMapper_G_static_cast_QObject_ptr(self as *const ::signal_mapper::SignalMapper as *mut ::signal_mapper::SignalMapper) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::signal_mapper::SignalMapper> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::signal_mapper::SignalMapper {
    let ffi_result = ::ffi::qt_core_c_QSignalMapper_G_static_cast_QSignalMapper_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::signal_mapper::SignalMapper {
    let ffi_result = ::ffi::qt_core_c_QSignalMapper_G_static_cast_QSignalMapper_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::signal_mapper::SignalMapper {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalMapper_G_static_cast_QObject_ptr(self as *const ::signal_mapper::SignalMapper as *mut ::signal_mapper::SignalMapper) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::signal_mapper::SignalMapper {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSignalMapper_G_static_cast_QObject_ptr(self as *mut ::signal_mapper::SignalMapper) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SignalMapper::mapping](../struct.SignalMapper.html#method.mapping) method.
  pub trait SignalMapperMappingArgs<'largs> {
    fn exec(self, original_self: &'largs ::signal_mapper::SignalMapper) -> *mut ::object::Object;
  }
  impl<'largs> SignalMapperMappingArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::signal_mapper::SignalMapper) -> *mut ::object::Object {
      let id = self;
      unsafe { ::ffi::qt_core_c_QSignalMapper_mapping_id(original_self as *const ::signal_mapper::SignalMapper, id) }
    }
  }
  impl<'largs> SignalMapperMappingArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::signal_mapper::SignalMapper) -> *mut ::object::Object {
      let text = self;
      unsafe {
        ::ffi::qt_core_c_QSignalMapper_mapping_text(original_self as *const ::signal_mapper::SignalMapper,
                                                    text as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SignalMapper::set_mapping](../struct.SignalMapper.html#method.set_mapping) method.
  pub trait SignalMapperSetMappingArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::signal_mapper::SignalMapper) -> ();
  }
  impl<'largs> SignalMapperSetMappingArgs<'largs> for (*mut ::object::Object, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::signal_mapper::SignalMapper) -> () {
      let sender = self.0;
      let id = self.1;
      ::ffi::qt_core_c_QSignalMapper_setMapping_sender_id(original_self as *mut ::signal_mapper::SignalMapper,
                                                          sender,
                                                          id)
    }
  }
  impl<'largs> SignalMapperSetMappingArgs<'largs> for (*mut ::object::Object, *mut ::object::Object) {
    unsafe fn exec(self, original_self: &'largs mut ::signal_mapper::SignalMapper) -> () {
      let sender = self.0;
      let object = self.1;
      ::ffi::qt_core_c_QSignalMapper_setMapping_sender_object(original_self as *mut ::signal_mapper::SignalMapper,
                                                              sender,
                                                              object)
    }
  }
  impl<'largs> SignalMapperSetMappingArgs<'largs> for (*mut ::object::Object, &'largs ::string::String) {
    unsafe fn exec(self, original_self: &'largs mut ::signal_mapper::SignalMapper) -> () {
      let sender = self.0;
      let text = self.1;
      ::ffi::qt_core_c_QSignalMapper_setMapping_sender_text(original_self as *mut ::signal_mapper::SignalMapper,
                                                            sender,
                                                            text as *const ::string::String)
    }
  }
}

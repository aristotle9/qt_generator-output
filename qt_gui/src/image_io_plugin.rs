/// C++ type: <span style='color: green;'>```QImageIOPlugin::Capability```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Capability {
  /// C++ enum variant: <span style='color: green;'>```CanRead = 1```</span>
  Read = 1,
  /// C++ enum variant: <span style='color: green;'>```CanWrite = 2```</span>
  Write = 2,
  /// C++ enum variant: <span style='color: green;'>```CanReadIncremental = 4```</span>
  ReadIncremental = 4,
}

impl ::qt_core::flags::FlaggableEnum for Capability {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Capability"
  }
}

/// C++ type: <span style='color: green;'>```QImageIOPlugin```</span>
#[repr(C)]
pub struct ImageIOPlugin(u8);

impl ImageIOPlugin {
  /// C++ method: <span style='color: green;'>```pure virtual QFlags<QImageIOPlugin::Capability> QImageIOPlugin::capabilities(QIODevice* device, const QByteArray& format) const```</span>
  ///
  ///
  pub unsafe fn capabilities(&self,
                             device: *mut ::qt_core::io_device::IODevice,
                             format: &::qt_core::byte_array::ByteArray)
                             -> ::qt_core::flags::Flags<::image_io_plugin::Capability> {
    let ffi_result = ::ffi::qt_gui_c_QImageIOPlugin_capabilities(self as *const ::image_io_plugin::ImageIOPlugin,
                                                                 device,
                                                                 format as *const ::qt_core::byte_array::ByteArray);
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QImageIOPlugin::create```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create(&self, *mut ::qt_core::io_device::IODevice) -> *mut ::image_io_handler::ImageIOHandler```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QImageIOHandler* QImageIOPlugin::create(QIODevice* device) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create(&self, (*mut ::qt_core::io_device::IODevice, &::qt_core::byte_array::ByteArray)) -> *mut ::image_io_handler::ImageIOHandler```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QImageIOHandler* QImageIOPlugin::create(QIODevice* device, const QByteArray& format = ?) const```</span>
  ///
  ///
  pub unsafe fn create<'largs, Args>(&'largs self, args: Args) -> *mut ::image_io_handler::ImageIOHandler
    where Args: overloading::ImageIOPluginCreateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QImageIOPlugin::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QImageIOPlugin_metaObject(self as *const ::image_io_plugin::ImageIOPlugin) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QImageIOPlugin::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QImageIOPlugin_qt_metacall(self as *mut ::image_io_plugin::ImageIOPlugin,
                                               arg1 as *const ::qt_core::meta_object::Call,
                                               arg2,
                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QImageIOPlugin::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QImageIOPlugin_qt_metacast(self as *mut ::image_io_plugin::ImageIOPlugin, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QImageIOPlugin::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QImageIOPlugin_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QImageIOPlugin::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QImageIOPlugin_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::image_io_plugin::ImageIOPlugin {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QImageIOPlugin_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ImageIOPlugin`.
  pub struct Signals<'a>(&'a ::image_io_plugin::ImageIOPlugin);
  /// Represents a built-in Qt signal `QImageIOPlugin::objectNameChanged`.
  ///
  /// An object of this type can be created from `ImageIOPlugin` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ImageIOPlugin` object.
  pub struct ObjectNameChanged<'a>(&'a ::image_io_plugin::ImageIOPlugin);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QImageIOPlugin::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::image_io_plugin::ImageIOPlugin {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::image_io_plugin::ImageIOPlugin {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QImageIOPlugin_G_static_cast_QObject_ptr(self as *mut ::image_io_plugin::ImageIOPlugin)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImageIOPlugin_G_static_cast_QObject_ptr(self as *const ::image_io_plugin::ImageIOPlugin as *mut ::image_io_plugin::ImageIOPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::image_io_plugin::ImageIOPlugin> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::image_io_plugin::ImageIOPlugin {
    let ffi_result =
      ::ffi::qt_gui_c_QImageIOPlugin_G_static_cast_QImageIOPlugin_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::image_io_plugin::ImageIOPlugin {
    let ffi_result = ::ffi::qt_gui_c_QImageIOPlugin_G_static_cast_QImageIOPlugin_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::image_io_plugin::ImageIOPlugin {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImageIOPlugin_G_static_cast_QObject_ptr(self as *const ::image_io_plugin::ImageIOPlugin as *mut ::image_io_plugin::ImageIOPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::image_io_plugin::ImageIOPlugin {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QImageIOPlugin_G_static_cast_QObject_ptr(self as *mut ::image_io_plugin::ImageIOPlugin)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ImageIOPlugin::create](../struct.ImageIOPlugin.html#method.create) method.
  pub trait ImageIOPluginCreateArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs ::image_io_plugin::ImageIOPlugin)
                   -> *mut ::image_io_handler::ImageIOHandler;
  }
  impl<'largs> ImageIOPluginCreateArgs<'largs> for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self,
                   original_self: &'largs ::image_io_plugin::ImageIOPlugin)
                   -> *mut ::image_io_handler::ImageIOHandler {
      let device = self;
      ::ffi::qt_gui_c_QImageIOPlugin_create_device(original_self as *const ::image_io_plugin::ImageIOPlugin,
                                                   device)
    }
  }
  impl<'largs> ImageIOPluginCreateArgs<'largs>
    for (*mut ::qt_core::io_device::IODevice, &'largs ::qt_core::byte_array::ByteArray) {
    unsafe fn exec(self,
                   original_self: &'largs ::image_io_plugin::ImageIOPlugin)
                   -> *mut ::image_io_handler::ImageIOHandler {
      let device = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QImageIOPlugin_create_device_format(original_self as *const ::image_io_plugin::ImageIOPlugin,
                                                          device,
                                                          format as *const ::qt_core::byte_array::ByteArray)
    }
  }
}

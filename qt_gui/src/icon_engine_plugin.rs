/// C++ type: <span style='color: green;'>```QIconEnginePlugin```</span>
#[repr(C)]
pub struct IconEnginePlugin(u8);

impl IconEnginePlugin {
  /// C++ method: <span style='color: green;'>```QIconEnginePlugin::create```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create(&mut self, ()) -> *mut ::icon_engine::IconEngine```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QIconEngine* QIconEnginePlugin::create()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create(&mut self, &::qt_core::string::String) -> *mut ::icon_engine::IconEngine```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QIconEngine* QIconEnginePlugin::create(const QString& filename = ?)```</span>
  ///
  ///
  pub fn create<'largs, Args>(&'largs mut self, args: Args) -> *mut ::icon_engine::IconEngine
    where Args: overloading::IconEnginePluginCreateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QIconEnginePlugin::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QIconEnginePlugin_metaObject(self as *const ::icon_engine_plugin::IconEnginePlugin) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QIconEnginePlugin::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QIconEnginePlugin_qt_metacall(self as *mut ::icon_engine_plugin::IconEnginePlugin,
                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                  arg2,
                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QIconEnginePlugin::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QIconEnginePlugin_qt_metacast(self as *mut ::icon_engine_plugin::IconEnginePlugin, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString QIconEnginePlugin::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QIconEnginePlugin_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QIconEnginePlugin::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QIconEnginePlugin_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::icon_engine_plugin::IconEnginePlugin {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QIconEnginePlugin_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `IconEnginePlugin`.
  pub struct Signals<'a>(&'a ::icon_engine_plugin::IconEnginePlugin);
  /// Represents a built-in Qt signal `QIconEnginePlugin::objectNameChanged`.
  ///
  /// An object of this type can be created from `IconEnginePlugin` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IconEnginePlugin` object.
  pub struct ObjectNameChanged<'a>(&'a ::icon_engine_plugin::IconEnginePlugin);
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
    /// Returns an object representing a built-in Qt signal `QIconEnginePlugin::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::icon_engine_plugin::IconEnginePlugin {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::icon_engine_plugin::IconEnginePlugin {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEnginePlugin_G_static_cast_QObject_ptr(self as *mut ::icon_engine_plugin::IconEnginePlugin)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIconEnginePlugin_G_static_cast_QObject_ptr(self as *const ::icon_engine_plugin::IconEnginePlugin as *mut ::icon_engine_plugin::IconEnginePlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::icon_engine_plugin::IconEnginePlugin> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::icon_engine_plugin::IconEnginePlugin {
    let ffi_result =
      ::ffi::qt_gui_c_QIconEnginePlugin_G_static_cast_QIconEnginePlugin_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::icon_engine_plugin::IconEnginePlugin {
    let ffi_result = ::ffi::qt_gui_c_QIconEnginePlugin_G_static_cast_QIconEnginePlugin_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::icon_engine_plugin::IconEnginePlugin {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIconEnginePlugin_G_static_cast_QObject_ptr(self as *const ::icon_engine_plugin::IconEnginePlugin as *mut ::icon_engine_plugin::IconEnginePlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::icon_engine_plugin::IconEnginePlugin {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QIconEnginePlugin_G_static_cast_QObject_ptr(self as *mut ::icon_engine_plugin::IconEnginePlugin)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [IconEnginePlugin::create](../struct.IconEnginePlugin.html#method.create) method.
  pub trait IconEnginePluginCreateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::icon_engine_plugin::IconEnginePlugin) -> *mut ::icon_engine::IconEngine;
  }
  impl<'largs> IconEnginePluginCreateArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::icon_engine_plugin::IconEnginePlugin) -> *mut ::icon_engine::IconEngine {
      let filename = self;
      unsafe {
        ::ffi::qt_gui_c_QIconEnginePlugin_create_filename(original_self as *mut ::icon_engine_plugin::IconEnginePlugin, filename as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> IconEnginePluginCreateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::icon_engine_plugin::IconEnginePlugin) -> *mut ::icon_engine::IconEngine {

      unsafe {
        ::ffi::qt_gui_c_QIconEnginePlugin_create_no_args(original_self as *mut ::icon_engine_plugin::IconEnginePlugin)
      }
    }
  }
}

/// C++ type: <span style='color: green;'>```QStaticPlugin```</span>
#[repr(C)]
pub struct StaticPlugin([u8; ::type_sizes::QT_CORE_STATIC_PLUGIN_STATIC_PLUGIN]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StaticPlugin {
  unsafe fn new_uninitialized() -> StaticPlugin {
    StaticPlugin(::std::mem::uninitialized())
  }
}

impl StaticPlugin {
  /// C++ method: <span style='color: green;'>```QObject* (*FN_PTR)() QStaticPlugin::instance() const```</span>
  ///
  ///
  pub fn instance(&self) -> extern "C" fn() -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QStaticPlugin_instance(self as *const ::static_plugin::StaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject QStaticPlugin::metaData() const```</span>
  ///
  ///
  pub fn meta_data(&self) -> ::json_object::JsonObject {
    {
      let mut object: ::json_object::JsonObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStaticPlugin_metaData_to_output(self as *const ::static_plugin::StaticPlugin, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const char* (*FN_PTR)() QStaticPlugin::rawMetaData() const```</span>
  ///
  ///
  pub fn raw_meta_data(&self) -> extern "C" fn() -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QStaticPlugin_rawMetaData(self as *const ::static_plugin::StaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QStaticPlugin::set_instance(QObject* (*FN_PTR)() value)```</span>
  ///
  ///
  pub unsafe fn set_instance(&mut self, value: extern "C" fn() -> *mut ::object::Object) {
    ::ffi::qt_core_c_QStaticPlugin_set_instance(self as *mut ::static_plugin::StaticPlugin, value)
  }

  /// C++ method: <span style='color: green;'>```void QStaticPlugin::set_rawMetaData(const char* (*FN_PTR)() value)```</span>
  ///
  ///
  pub unsafe fn set_raw_meta_data(&mut self, value: extern "C" fn() -> *const ::libc::c_char) {
    ::ffi::qt_core_c_QStaticPlugin_set_rawMetaData(self as *mut ::static_plugin::StaticPlugin, value)
  }
}

impl Drop for ::static_plugin::StaticPlugin {
  /// C++ method: <span style='color: green;'>```[destructor] void QStaticPlugin::~QStaticPlugin()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QStaticPlugin_destructor(self as *mut ::static_plugin::StaticPlugin) }
  }
}

/// C++ method: <span style='color: green;'>```void qRegisterStaticPluginFunction(QStaticPlugin staticPlugin)```</span>
///
///
pub fn register_static_plugin_function(static_plugin: &::static_plugin::StaticPlugin) {
  unsafe { ::ffi::qt_core_c_QStaticPlugin_G_qRegisterStaticPluginFunction(static_plugin as *const ::static_plugin::StaticPlugin) }
}

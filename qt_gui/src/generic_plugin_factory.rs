/// C++ type: <span style='color: green;'>```QGenericPluginFactory```</span>
#[repr(C)]
pub struct GenericPluginFactory(u8);

impl GenericPluginFactory {
  /// C++ method: <span style='color: green;'>```static QObject* QGenericPluginFactory::create(const QString& arg1, const QString& arg2)```</span>
  ///
  ///
  pub fn create(arg1: &::qt_core::string::String, arg2: &::qt_core::string::String) -> *mut ::qt_core::object::Object {
    unsafe {
      ::ffi::qt_gui_c_QGenericPluginFactory_create(arg1 as *const ::qt_core::string::String,
                                                   arg2 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QGenericPluginFactory::keys()```</span>
  ///
  ///
  pub fn keys() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QGenericPluginFactory_keys_to_output(&mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::generic_plugin_factory::GenericPluginFactory {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QGenericPluginFactory_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleFactory```</span>
#[repr(C)]
pub struct StyleFactory(u8);

impl StyleFactory {
  /// C++ method: <span style='color: green;'>```static QStyle* QStyleFactory::create(const QString& arg1)```</span>
  ///
  ///
  pub fn create(arg1: &::qt_core::string::String) -> *mut ::style::Style {
    unsafe { ::ffi::qt_widgets_c_QStyleFactory_create(arg1 as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QStyleFactory::keys()```</span>
  ///
  ///
  pub fn keys() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QStyleFactory_keys_to_output(&mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_factory::StyleFactory {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleFactory_delete
  }
}

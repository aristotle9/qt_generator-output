/// C++ type: <span style='color: green;'>```QFactoryInterface```</span>
#[repr(C)]
pub struct FactoryInterface(u8);

impl FactoryInterface {
  /// C++ method: <span style='color: green;'>```pure virtual QStringList QFactoryInterface::keys() const```</span>
  ///
  ///
  pub fn keys(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFactoryInterface_keys_to_output(self as *const ::factory_interface::FactoryInterface,
                                                          &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::factory_interface::FactoryInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QFactoryInterface_delete
  }
}

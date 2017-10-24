/// C++ type: <span style='color: green;'>```QXmlStreamEntityResolver```</span>
#[repr(C)]
pub struct XmlStreamEntityResolver(u8);

impl XmlStreamEntityResolver {
  /// C++ method: <span style='color: green;'>```virtual QString QXmlStreamEntityResolver::resolveEntity(const QString& publicId, const QString& systemId)```</span>
  ///
  ///
  pub fn resolve_entity(&mut self, public_id: &::string::String, system_id: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamEntityResolver_resolveEntity_to_output(self as *mut ::xml_stream_entity_resolver::XmlStreamEntityResolver, public_id as *const ::string::String, system_id as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QXmlStreamEntityResolver::resolveUndeclaredEntity(const QString& name)```</span>
  ///
  ///
  pub fn resolve_undeclared_entity(&mut self, name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamEntityResolver_resolveUndeclaredEntity_to_output(self as *mut ::xml_stream_entity_resolver::XmlStreamEntityResolver, name as *const ::string::String, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::xml_stream_entity_resolver::XmlStreamEntityResolver {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QXmlStreamEntityResolver_delete
  }
}

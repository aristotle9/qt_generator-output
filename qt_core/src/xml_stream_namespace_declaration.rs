/// C++ type: <span style='color: green;'>```QXmlStreamNamespaceDeclaration```</span>
#[repr(C)]
pub struct XmlStreamNamespaceDeclaration([u8; ::type_sizes::QT_CORE_XML_STREAM_NAMESPACE_DECLARATION_XML_STREAM_NAMESPACE_DECLARATION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamNamespaceDeclaration {
  unsafe fn new_uninitialized() -> XmlStreamNamespaceDeclaration {
    XmlStreamNamespaceDeclaration(::std::mem::uninitialized())
  }
}

impl XmlStreamNamespaceDeclaration {
  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamNamespaceDeclaration::namespaceUri() const```</span>
  ///
  ///
  pub fn namespace_uri(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_namespaceUri_to_output(self as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String)) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QString& prefix, const QString& namespaceUri)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamNamespaceDeclaration::QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration
    where Args: overloading::XmlStreamNamespaceDeclarationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration& QXmlStreamNamespaceDeclaration::operator=(const QXmlStreamNamespaceDeclaration& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             arg1: &'l1 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration)
                             -> &'l0 mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_operator_assign(self as *mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, arg1 as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamNamespaceDeclaration::operator==(const QXmlStreamNamespaceDeclaration& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_operator_eq(self as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, other as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamNamespaceDeclaration::operator!=(const QXmlStreamNamespaceDeclaration& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_operator_neq(self as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, other as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamNamespaceDeclaration::prefix() const```</span>
  ///
  ///
  pub fn prefix(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_prefix_to_output(self as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, &mut object);
      }
      object
    }
  }
}

impl Drop for ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamNamespaceDeclaration::~QXmlStreamNamespaceDeclaration()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_destructor(self as *mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamNamespaceDeclaration::new](../struct.XmlStreamNamespaceDeclaration.html#method.new) method.
  pub trait XmlStreamNamespaceDeclarationNewArgs {
    fn exec(self) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration;
  }
  impl<'a> XmlStreamNamespaceDeclarationNewArgs for &'a ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, ) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let arg1 = self;
    {
let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_constructor_arg1(arg1 as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, &mut object); }object
}
  }
}
  impl XmlStreamNamespaceDeclarationNewArgs for () {
    fn exec(self) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

      {
        let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> XmlStreamNamespaceDeclarationNewArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
      let prefix = self.0;
      let namespace_uri = self.1;
      {
        let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamNamespaceDeclaration_constructor_prefix_namespaceUri(prefix as *const ::string::String, namespace_uri as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
}

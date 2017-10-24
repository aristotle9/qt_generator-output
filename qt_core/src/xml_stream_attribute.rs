/// C++ type: <span style='color: green;'>```QXmlStreamAttribute```</span>
#[repr(C)]
pub struct XmlStreamAttribute([u8; ::type_sizes::QT_CORE_XML_STREAM_ATTRIBUTE_XML_STREAM_ATTRIBUTE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for XmlStreamAttribute {
  unsafe fn new_uninitialized() -> XmlStreamAttribute {
    XmlStreamAttribute(::std::mem::uninitialized())
  }
}

impl XmlStreamAttribute {
  /// C++ method: <span style='color: green;'>```bool QXmlStreamAttribute::isDefault() const```</span>
  ///
  ///
  pub fn is_default(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QXmlStreamAttribute_isDefault(self as *const ::xml_stream_attribute::XmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttribute::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamAttribute_name_to_output(self as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttribute::namespaceUri() const```</span>
  ///
  ///
  pub fn namespace_uri(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamAttribute_namespaceUri_to_output(self as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute::QXmlStreamAttribute```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::xml_stream_attribute::XmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamAttribute::QXmlStreamAttribute()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String, &::string::String)) -> ::xml_stream_attribute::XmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamAttribute::QXmlStreamAttribute(const QString& namespaceUri, const QString& name, const QString& value)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String)) -> ::xml_stream_attribute::XmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamAttribute::QXmlStreamAttribute(const QString& qualifiedName, const QString& value)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::xml_stream_attribute::XmlStreamAttribute) -> ::xml_stream_attribute::XmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QXmlStreamAttribute::QXmlStreamAttribute(const QXmlStreamAttribute& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::xml_stream_attribute::XmlStreamAttribute
    where Args: overloading::XmlStreamAttributeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute& QXmlStreamAttribute::operator=(const QXmlStreamAttribute& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             arg1: &'l1 ::xml_stream_attribute::XmlStreamAttribute)
                             -> &'l0 mut ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QXmlStreamAttribute_operator_assign(self as *mut ::xml_stream_attribute::XmlStreamAttribute, arg1 as *const ::xml_stream_attribute::XmlStreamAttribute)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamAttribute::operator==(const QXmlStreamAttribute& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::xml_stream_attribute::XmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamAttribute_operator_eq(self as *const ::xml_stream_attribute::XmlStreamAttribute,
                                                       other as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QXmlStreamAttribute::operator!=(const QXmlStreamAttribute& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::xml_stream_attribute::XmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QXmlStreamAttribute_operator_neq(self as *const ::xml_stream_attribute::XmlStreamAttribute,
                                                        other as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttribute::prefix() const```</span>
  ///
  ///
  pub fn prefix(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamAttribute_prefix_to_output(self as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttribute::qualifiedName() const```</span>
  ///
  ///
  pub fn qualified_name(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamAttribute_qualifiedName_to_output(self as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QXmlStreamAttribute::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QXmlStreamAttribute_value_to_output(self as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
      }
      object
    }
  }
}

impl Drop for ::xml_stream_attribute::XmlStreamAttribute {
  /// C++ method: <span style='color: green;'>```[destructor] void QXmlStreamAttribute::~QXmlStreamAttribute()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QXmlStreamAttribute_destructor(self as *mut ::xml_stream_attribute::XmlStreamAttribute) }
  }
}

/// C++ method: <span style='color: green;'>```void swap(QXmlStreamStringRef& value1, QXmlStreamStringRef& value2)```</span>
///
///
pub fn swap(value1: &mut ::xml_stream_string_ref::XmlStreamStringRef,
            value2: &mut ::xml_stream_string_ref::XmlStreamStringRef) {
  unsafe {
    ::ffi::qt_core_c_QXmlStreamAttribute_G_swap(value1 as *mut ::xml_stream_string_ref::XmlStreamStringRef,
                                                value2 as *mut ::xml_stream_string_ref::XmlStreamStringRef)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [XmlStreamAttribute::new](../struct.XmlStreamAttribute.html#method.new) method.
  pub trait XmlStreamAttributeNewArgs {
    fn exec(self) -> ::xml_stream_attribute::XmlStreamAttribute;
  }
  impl<'a> XmlStreamAttributeNewArgs for &'a ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self) -> ::xml_stream_attribute::XmlStreamAttribute {
      let arg1 = self;
      {
        let mut object: ::xml_stream_attribute::XmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttribute_constructor_arg1(arg1 as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
        }
        object
      }
    }
  }
  impl<'a> XmlStreamAttributeNewArgs for (&'a ::string::String, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::xml_stream_attribute::XmlStreamAttribute {
      let namespace_uri = self.0;
      let name = self.1;
      let value = self.2;
      {
        let mut object: ::xml_stream_attribute::XmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttribute_constructor_namespaceUri_name_value(namespace_uri as *const ::string::String, name as *const ::string::String, value as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl XmlStreamAttributeNewArgs for () {
    fn exec(self) -> ::xml_stream_attribute::XmlStreamAttribute {

      {
        let mut object: ::xml_stream_attribute::XmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttribute_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> XmlStreamAttributeNewArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::xml_stream_attribute::XmlStreamAttribute {
      let qualified_name = self.0;
      let value = self.1;
      {
        let mut object: ::xml_stream_attribute::XmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QXmlStreamAttribute_constructor_qualifiedName_value(qualified_name as *const ::string::String, value as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
}
